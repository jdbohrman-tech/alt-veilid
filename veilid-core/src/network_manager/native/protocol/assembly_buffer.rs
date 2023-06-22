use super::*;
use range_set_blaze::RangeSetBlaze;
use std::io::{Error, ErrorKind};
use std::sync::atomic::{AtomicU16, Ordering};

// AssemblyBuffer Version 1 properties
const VERSION_1: u8 = 1;
type LengthType = u16;
type SequenceType = u16;
const HEADER_LEN: usize = 8;
const MAX_MESSAGE_LEN: usize = LengthType::MAX as usize;

// XXX: keep statistics on all drops and why we dropped them
// XXX: move to config
const FRAGMENT_LEN: usize = 1280 - HEADER_LEN;
const MAX_CONCURRENT_HOSTS: usize = 256;
const MAX_ASSEMBLIES_PER_HOST: usize = 256;
const MAX_BUFFER_PER_HOST: usize = 256 * 1024;

/////////////////////////////////////////////////////////

pub struct Message {
    data: Vec<u8>,
    remote_addr: SocketAddr,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct PeerKey {
    remote_addr: SocketAddr,
}

#[derive(Clone, Eq, PartialEq)]
struct MessageAssembly {
    seq: SequenceType,
    data: Vec<u8>,
    parts: RangeSetBlaze<LengthType>,
}

#[derive(Clone, Eq, PartialEq)]
struct PeerMessages {
    assemblies: Vec<MessageAssembly>,
}

impl PeerMessages {
    pub fn new() -> Self {
        Self {
            assemblies: Vec::new(),
        }
    }
}

/////////////////////////////////////////////////////////

struct AssemblyBufferInner {
    peer_message_map: HashMap<PeerKey, PeerMessages>,
}

struct AssemblyBufferUnlockedInner {
    outbound_lock_table: AsyncTagLockTable<SocketAddr>,
    next_seq: AtomicU16,
}

/// Packet reassembly and fragmentation handler
/// No retry, no acknowledgment, no flow control
/// Just trying to survive lower path MTU for larger messages
#[derive(Clone)]
pub struct AssemblyBuffer {
    inner: Arc<Mutex<AssemblyBufferInner>>,
    unlocked_inner: Arc<AssemblyBufferUnlockedInner>,
}

impl AssemblyBuffer {
    pub fn new_unlocked_inner() -> AssemblyBufferUnlockedInner {
        AssemblyBufferUnlockedInner {
            outbound_lock_table: AsyncTagLockTable::new(),
            next_seq: AtomicU16::new(0),
        }
    }
    pub fn new_inner() -> AssemblyBufferInner {
        AssemblyBufferInner {
            peer_message_map: HashMap::new(),
        }
    }

    pub fn new(frag_len: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Self::new_inner())),
            unlocked_inner: Arc::new(Self::new_unlocked_inner()),
        }
    }

    /// Receive a packet chunk and add to the message assembly
    /// if a message has been completely, return it
    pub fn receive_packet(&self, frame: &[u8], remote_addr: SocketAddr) -> Option<Message> {
        // If we receive a zero length frame, send it
        if frame.len() == 0 {
            return Some(Message {
                data: frame.to_vec(),
                remote_addr,
            });
        }

        // If we receive a frame smaller than or equal to the length of the header, drop it
        // or if this frame is larger than our max message length, then drop it
        if frame.len() <= HEADER_LEN || frame.len() > MAX_MESSAGE_LEN {
            return None;
        }

        // --- Decode the header

        // Drop versions we don't understand
        if frame[0] != VERSION_1 {
            return None;
        }
        // Version 1 header
        let seq = SequenceType::from_be_bytes(frame[2..4].try_into().unwrap());
        let off = LengthType::from_be_bytes(frame[4..6].try_into().unwrap());
        let len = LengthType::from_be_bytes(frame[6..HEADER_LEN].try_into().unwrap());
        let chunk = &frame[HEADER_LEN..];

        // See if we have a whole message and not a fragment
        if off == 0 && len as usize == chunk.len() {
            return Some(Message {
                data: frame.to_vec(),
                remote_addr,
            });
        }

        // Drop fragments with offsets greater than or equal to the message length
        if off >= len {
            return None;
        }
        // Drop fragments where the chunk would be applied beyond the message length
        if off as usize + chunk.len() > len as usize {
            return None;
        }

        // Get or create the peer message assemblies
        // and drop the packet if we have too many peers
        let mut inner = self.inner.lock();
        let peer_key = PeerKey { remote_addr };
        let peer_messages = match inner.peer_message_map.entry(peer_key) {
            std::collections::hash_map::Entry::Occupied(e) => e.get_mut(),
            std::collections::hash_map::Entry::Vacant(v) => {
                // See if we have room for one more
                if inner.peer_message_map.len() == MAX_CONCURRENT_HOSTS {
                    return None;
                }
                // Add the peer
                v.insert(PeerMessages::new())
            }
        };

        None
    }

    /// Add framing to chunk to send to the wire
    fn frame_chunk(chunk: &[u8], offset: usize, message_len: usize, seq: SequenceType) -> Vec<u8> {
        assert!(chunk.len() > 0);
        assert!(message_len <= MAX_MESSAGE_LEN);
        assert!(offset + chunk.len() <= message_len);

        let off: LengthType = offset as LengthType;
        let len: LengthType = message_len as LengthType;

        unsafe {
            // Uninitialized vector, careful!
            let mut out = unaligned_u8_vec_uninit(chunk.len() + HEADER_LEN);

            // Write out header
            out[0] = VERSION_1;
            out[1] = 0; // reserved
            out[2..4].copy_from_slice(&seq.to_be_bytes()); // sequence number
            out[4..6].copy_from_slice(&off.to_be_bytes()); // offset of chunk inside message
            out[6..HEADER_LEN].copy_from_slice(&len.to_be_bytes()); // total length of message

            // Write out body
            out[HEADER_LEN..out.len()].copy_from_slice(chunk);
            out
        }
    }

    /// Split a message into packets and send them serially, ensuring
    /// that they are sent consecutively to a particular remote address,
    /// never interleaving packets from one message and other to minimize reassembly problems
    pub async fn split_message<F>(&self, message: Message, sender: F) -> std::io::Result<()>
    where
        F: Fn(Vec<u8>, SocketAddr) -> SendPinBoxFuture<std::io::Result<()>>,
    {
        if message.data.len() > MAX_MESSAGE_LEN {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        // Do not frame or split anything zero bytes long, just send it
        if message.data.len() == 0 {
            sender(message.data, message.remote_addr).await?;
            return Ok(());
        }

        // Lock per remote addr
        let _tag_lock = self
            .unlocked_inner
            .outbound_lock_table
            .lock_tag(message.remote_addr)
            .await;

        // Get a message seq
        let seq = self.unlocked_inner.next_seq.fetch_add(1, Ordering::Relaxed);

        // Chunk it up
        let mut offset = 0usize;
        let message_len = message.data.len();
        for chunk in message.data.chunks(FRAGMENT_LEN) {
            // Frame chunk
            let framed_chunk = Self::frame_chunk(chunk, offset, message_len, seq);
            // Send chunk
            sender(framed_chunk, message.remote_addr).await?;
            // Go to next chunk
            offset += chunk.len()
        }

        Ok(())
    }
}
