//! Packet reassembly and fragmentation handler
//!
//! * [AssemblyBuffer] handles both the sender and received end of fragmentation and reassembly.

use super::*;
use range_set_blaze::RangeSetBlaze;
use std::io::{Error, ErrorKind};
use std::sync::atomic::{AtomicU16, Ordering};

// AssemblyBuffer Version 1 properties
const VERSION_1: u8 = 1;
type LengthType = u16;
type SequenceType = u16;
const HEADER_LEN: usize = 8;
const MAX_LEN: usize = LengthType::MAX as usize;

// XXX: keep statistics on all drops and why we dropped them
// XXX: move to config eventually?

/// The hard-coded maximum fragment size used by AssemblyBuffer
///
/// Eventually this should parameterized and made configurable.
pub const FRAGMENT_LEN: usize = 1280 - HEADER_LEN;

const MAX_CONCURRENT_HOSTS: usize = 256;
const MAX_ASSEMBLIES_PER_HOST: usize = 256;
const MAX_BUFFER_PER_HOST: usize = 256 * 1024;
const MAX_ASSEMBLY_AGE_US: u64 = 10_000_000;

/////////////////////////////////////////////////////////

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct PeerKey {
    remote_addr: SocketAddr,
}

#[derive(Clone, Eq, PartialEq)]
struct MessageAssembly {
    timestamp: u64,
    seq: SequenceType,
    data: Vec<u8>,
    parts: RangeSetBlaze<LengthType>,
}

#[derive(Clone, Eq, PartialEq)]
struct PeerMessages {
    total_buffer: usize,
    assemblies: VecDeque<MessageAssembly>,
}

impl PeerMessages {
    pub fn new() -> Self {
        Self {
            total_buffer: 0,
            assemblies: VecDeque::new(),
        }
    }

    fn merge_in_data(
        &mut self,
        timestamp: u64,
        ass: usize,
        off: LengthType,
        len: LengthType,
        chunk: &[u8],
    ) -> bool {
        let assembly = &mut self.assemblies[ass];

        // Ensure the new fragment hasn't redefined the message length, reusing the same seq
        if assembly.data.len() != len as usize {
            // Drop the assembly and just go with the new fragment as starting a new assembly
            let seq = assembly.seq;
            self.remove_assembly(ass);
            self.new_assembly(timestamp, seq, off, len, chunk);
            return false;
        }

        let part_start = off;
        let part_end = off + chunk.len() as LengthType - 1;
        let part = RangeSetBlaze::from_iter([part_start..=part_end]);

        // if fragments overlap, drop the old assembly and go with a new one
        if !assembly.parts.is_disjoint(&part) {
            let seq = assembly.seq;
            self.remove_assembly(ass);
            self.new_assembly(timestamp, seq, off, len, chunk);
            return false;
        }

        // Merge part
        assembly.parts |= part;
        assembly.data[part_start as usize..=part_end as usize].copy_from_slice(chunk);

        // Check to see if this part is done
        if assembly.parts.ranges_len() == 1
            && assembly.parts.first().unwrap() == 0
            && assembly.parts.last().unwrap() == len - 1
        {
            return true;
        }
        false
    }

    fn new_assembly(
        &mut self,
        timestamp: u64,
        seq: SequenceType,
        off: LengthType,
        len: LengthType,
        chunk: &[u8],
    ) -> usize {
        // ensure we have enough space for the new assembly
        self.reclaim_space(len as usize);

        // make the assembly
        let part_start = off;
        let part_end = off + chunk.len() as LengthType - 1;

        let mut assembly = MessageAssembly {
            timestamp,
            seq,
            data: vec![0u8; len as usize],
            parts: RangeSetBlaze::from_iter([part_start..=part_end]),
        };
        assembly.data[part_start as usize..=part_end as usize].copy_from_slice(chunk);

        // Add the buffer length in
        self.total_buffer += assembly.data.len();
        self.assemblies.push_front(assembly);

        // Was pushed front, return the front index
        0
    }

    fn remove_assembly(&mut self, index: usize) -> MessageAssembly {
        let assembly = self.assemblies.remove(index).unwrap();
        self.total_buffer -= assembly.data.len();
        assembly
    }

    fn truncate_assemblies(&mut self, new_len: usize) {
        for an in new_len..self.assemblies.len() {
            self.total_buffer -= self.assemblies[an].data.len();
        }
        self.assemblies.truncate(new_len);
    }

    fn reclaim_space(&mut self, needed_space: usize) {
        // If we have too many assemblies or too much buffer rotate some out
        while self.assemblies.len() > (MAX_ASSEMBLIES_PER_HOST - 1)
            || self.total_buffer > (MAX_BUFFER_PER_HOST - needed_space)
        {
            self.remove_assembly(self.assemblies.len() - 1);
        }
    }

    pub fn insert_fragment(
        &mut self,
        seq: SequenceType,
        off: LengthType,
        len: LengthType,
        chunk: &[u8],
    ) -> Option<Vec<u8>> {
        // Get the current timestamp
        let cur_ts = get_timestamp();

        // Get the assembly this belongs to by its sequence number
        let mut ass = None;
        for an in 0..self.assemblies.len() {
            // If this assembly's timestamp is too old, then everything after it will be too, drop em all
            let age = cur_ts.saturating_sub(self.assemblies[an].timestamp);
            if age > MAX_ASSEMBLY_AGE_US {
                self.truncate_assemblies(an);
                break;
            }
            // If this assembly has a matching seq, then assemble with it
            if self.assemblies[an].seq == seq {
                ass = Some(an);
            }
        }
        if ass.is_none() {
            // Add a new assembly to the front and return the first index
            self.new_assembly(cur_ts, seq, off, len, chunk);
            return None;
        }
        let ass = ass.unwrap();

        // Now that we have an assembly, merge in the fragment
        let done = self.merge_in_data(cur_ts, ass, off, len, chunk);

        // If the assembly is now equal to the entire range, then return it
        if done {
            let assembly = self.remove_assembly(ass);
            return Some(assembly.data);
        }

        // Otherwise, do nothing
        None
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
///
/// Used to provide, for raw unordered protocols such as UDP, a means to achieve:
///
/// * Fragmentation of packets to ensure they are smaller than a common MTU
/// * Reassembly of fragments upon receipt accounting for:
///   * duplication
///   * drops
///   * overlaps
///     
/// AssemblyBuffer does not try to replicate TCP or other highly reliable protocols. Here are some
/// of the design limitations to be aware of when using AssemblyBuffer:
///
/// * No packet acknowledgment. The sender does not know if a packet was received.
/// * No flow control. If there are buffering problems or drops, the sender and receiver have no protocol to address this.
/// * No retries or retransmission.
/// * No sequencing of packets. Packets may still be delivered to the application out of order, but this guarantees that only whole packets will be delivered if all of their fragments are received.

#[derive(Clone)]
pub struct AssemblyBuffer {
    inner: Arc<Mutex<AssemblyBufferInner>>,
    unlocked_inner: Arc<AssemblyBufferUnlockedInner>,
}

impl AssemblyBuffer {
    fn new_unlocked_inner() -> AssemblyBufferUnlockedInner {
        AssemblyBufferUnlockedInner {
            outbound_lock_table: AsyncTagLockTable::new(),
            next_seq: AtomicU16::new(0),
        }
    }
    fn new_inner() -> AssemblyBufferInner {
        AssemblyBufferInner {
            peer_message_map: HashMap::new(),
        }
    }

    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Self::new_inner())),
            unlocked_inner: Arc::new(Self::new_unlocked_inner()),
        }
    }

    /// Receive a packet chunk and add to the message assembly
    /// if a message has been completely, return it
    pub fn insert_frame(
        &self,
        frame: &[u8],
        remote_addr: SocketAddr,
    ) -> NetworkResult<Option<Vec<u8>>> {
        // If we receive a zero length frame, send it
        if frame.is_empty() {
            return NetworkResult::value(Some(frame.to_vec()));
        }

        // If we receive a frame smaller than or equal to the length of the header, drop it
        // or if this frame is larger than our max message length, then drop it
        if frame.len() <= HEADER_LEN || frame.len() > MAX_LEN {
            if debug_target_enabled!("network_result") {
                return NetworkResult::invalid_message(format!(
                    "invalid header length: frame.len={}",
                    frame.len()
                ));
            }
            return NetworkResult::invalid_message("invalid header length");
        }

        // --- Decode the header

        // Drop versions we don't understand
        if frame[0] != VERSION_1 {
            if debug_target_enabled!("network_result") {
                return NetworkResult::invalid_message(format!(
                    "invalid frame version: frame[0]={}",
                    frame[0]
                ));
            }
            return NetworkResult::invalid_message("invalid frame version");
        }
        // Version 1 header
        let seq = SequenceType::from_be_bytes(frame[2..4].try_into().unwrap());
        let off = LengthType::from_be_bytes(frame[4..6].try_into().unwrap());
        let len = LengthType::from_be_bytes(frame[6..HEADER_LEN].try_into().unwrap());
        let chunk = &frame[HEADER_LEN..];

        // See if we have a whole message and not a fragment
        if off == 0 && len as usize == chunk.len() {
            return NetworkResult::value(Some(chunk.to_vec()));
        }

        // Drop fragments with offsets greater than or equal to the message length
        if off >= len {
            if debug_target_enabled!("network_result") {
                return NetworkResult::invalid_message(format!(
                    "offset greater than length: off={} >= len={}",
                    off, len
                ));
            }
            return NetworkResult::invalid_message("offset greater than length");
        }
        // Drop fragments where the chunk would be applied beyond the message length
        if off as usize + chunk.len() > len as usize {
            if debug_target_enabled!("network_result") {
                return NetworkResult::invalid_message(format!(
                    "chunk applied beyond message length: off={} + chunk.len={} > len={}",
                    off,
                    chunk.len(),
                    len
                ));
            }
            return NetworkResult::invalid_message("chunk applied beyond message length");
        }

        // Get or create the peer message assemblies
        // and drop the packet if we have too many peers
        let mut inner = self.inner.lock();
        let peer_key = PeerKey { remote_addr };
        let peer_count = inner.peer_message_map.len();
        match inner.peer_message_map.entry(peer_key) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let peer_messages = e.get_mut();

                // Insert the fragment and see what comes out
                let out = peer_messages.insert_fragment(seq, off, len, chunk);

                // If we are returning a message, see if there are any more assemblies for this peer
                // If not, remove the peer
                if out.is_some() && peer_messages.assemblies.is_empty() {
                    e.remove();
                }
                NetworkResult::value(out)
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                // See if we have room for one more
                if peer_count == MAX_CONCURRENT_HOSTS {
                    return NetworkResult::value(None);
                }
                // Add the peer
                let peer_messages = v.insert(PeerMessages::new());

                // Insert the fragment and see what comes out
                NetworkResult::value(peer_messages.insert_fragment(seq, off, len, chunk))
            }
        }
    }

    /// Add framing to chunk to send to the wire
    fn frame_chunk(chunk: &[u8], offset: usize, message_len: usize, seq: SequenceType) -> Vec<u8> {
        assert!(!chunk.is_empty());
        assert!(message_len <= MAX_LEN);
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
            out[HEADER_LEN..].copy_from_slice(chunk);
            out
        }
    }

    /// Split a message into packets and send them serially, ensuring
    /// that they are sent consecutively to a particular remote address,
    /// never interleaving packets from one message and another to minimize reassembly problems
    pub async fn split_message<S, F>(
        &self,
        data: Vec<u8>,
        remote_addr: SocketAddr,
        mut sender: S,
    ) -> std::io::Result<NetworkResult<()>>
    where
        S: FnMut(Vec<u8>, SocketAddr) -> F,
        F: Future<Output = std::io::Result<NetworkResult<()>>>,
    {
        if data.len() > MAX_LEN {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        // Do not frame or split anything zero bytes long, just send it
        if data.is_empty() {
            return sender(data, remote_addr).await;
        }

        // Lock per remote addr
        let _tag_lock = self
            .unlocked_inner
            .outbound_lock_table
            .lock_tag(remote_addr)
            .await;

        // Get a message seq
        let seq = self.unlocked_inner.next_seq.fetch_add(1, Ordering::AcqRel);

        // Chunk it up
        let mut offset = 0usize;
        let message_len = data.len();
        for chunk in data.chunks(FRAGMENT_LEN) {
            // Frame chunk
            let framed_chunk = Self::frame_chunk(chunk, offset, message_len, seq);
            // Send chunk
            network_result_try!(sender(framed_chunk, remote_addr).await?);
            // Go to next chunk
            offset += chunk.len()
        }

        Ok(NetworkResult::value(()))
    }
}

impl Default for AssemblyBuffer {
    fn default() -> Self {
        Self::new()
    }
}
