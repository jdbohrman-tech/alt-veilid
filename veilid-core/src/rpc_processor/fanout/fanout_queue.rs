/// Fanout Queue
/// Keep a deque of unique nodes
/// Internally the 'front' of the list is the next out, and new nodes are added to the 'back' of the list
/// When passing in a 'cleanup' function, if it sorts the queue, the 'first' items in the queue are the 'next' out.
use super::*;

impl_veilid_log_facility!("fanout");
impl_veilid_component_registry_accessor!(FanoutQueue<'_>);

/// The status of a particular node we fanned out to
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FanoutNodeStatus {
    /// Node that needs processing
    Queued,
    /// Node currently being processed
    InProgress,
    /// Node that timed out during processing
    Timeout,
    /// Node that rejected the query
    Rejected,
    /// Node that accepted the query with a current result
    Accepted,
    /// Node that accepted the query but had an older result
    Stale,
    /// Node that has been disqualified for being too far away from the key
    Disqualified,
}

#[derive(Debug, Clone)]
pub struct FanoutNode {
    pub node_ref: NodeRef,
    pub status: FanoutNodeStatus,
}

pub type FanoutQueueSort<'a> =
    Box<dyn Fn(&TypedPublicKey, &TypedPublicKey) -> core::cmp::Ordering + Send + 'a>;

pub struct FanoutQueue<'a> {
    /// Link back to veilid component registry for logging
    registry: VeilidComponentRegistry,
    /// Crypto kind in use for this queue
    crypto_kind: CryptoKind,
    /// The status of all the nodes we have added so far
    nodes: HashMap<TypedPublicKey, FanoutNode>,
    /// Closer nodes to the record key are at the front of the list
    sorted_nodes: Vec<TypedPublicKey>,
    /// The sort function to use for the nodes
    node_sort: FanoutQueueSort<'a>,
    /// The channel to receive work requests to process
    sender: flume::Sender<flume::Sender<NodeRef>>,
    receiver: flume::Receiver<flume::Sender<NodeRef>>,
    /// Consensus count to use
    consensus_count: usize,
}

impl fmt::Debug for FanoutQueue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FanoutQueue")
            .field("crypto_kind", &self.crypto_kind)
            .field("nodes", &self.nodes)
            .field("sorted_nodes", &self.sorted_nodes)
            // .field("node_sort", &self.node_sort)
            .field("sender", &self.sender)
            .field("receiver", &self.receiver)
            .finish()
    }
}

impl fmt::Display for FanoutQueue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "nodes:\n{}",
            self.sorted_nodes
                .iter()
                .map(|x| format!("{}: {:?}", x, self.nodes.get(x).unwrap().status))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl<'a> FanoutQueue<'a> {
    /// Create a queue for fanout candidates that have a crypto-kind compatible node id
    pub fn new(
        registry: VeilidComponentRegistry,
        crypto_kind: CryptoKind,
        node_sort: FanoutQueueSort<'a>,
        consensus_count: usize,
    ) -> Self {
        let (sender, receiver) = flume::unbounded();
        Self {
            registry,
            crypto_kind,
            nodes: HashMap::new(),
            sorted_nodes: Vec::new(),
            node_sort,
            sender,
            receiver,
            consensus_count,
        }
    }

    /// Ask for more work when some is ready
    /// When work is ready it will be sent to work_sender so it can be received
    /// by the worker
    pub fn request_work(&mut self, work_sender: flume::Sender<NodeRef>) {
        let _ = self.sender.send(work_sender);

        // Send whatever work is available immediately
        self.send_more_work();
    }

    /// Add new nodes to a filtered and sorted list of fanout candidates
    pub fn add(&mut self, new_nodes: &[NodeRef]) {
        for node_ref in new_nodes {
            // Ensure the node has a comparable key with our current crypto kind
            let Some(key) = node_ref.node_ids().get(self.crypto_kind) else {
                continue;
            };
            // Check if we have already seen this node before (only one call per node ever)
            if self.nodes.contains_key(&key) {
                continue;
            }
            // Add the new node
            self.nodes.insert(
                key,
                FanoutNode {
                    node_ref: node_ref.clone(),
                    status: FanoutNodeStatus::Queued,
                },
            );
            self.sorted_nodes.push(key);
        }

        // Sort the node list
        self.sorted_nodes.sort_by(&self.node_sort);

        // Disqualify any nodes that can be
        self.disqualify();

        veilid_log!(self debug
            "FanoutQueue::add:\n  new_nodes={{\n{}}}\n  nodes={{\n{}}}\n",
            new_nodes.iter().map(|x| format!("  {}", x))
                .collect::<Vec<String>>()
                .join(",\n"),
            self.sorted_nodes
                .iter()
                .map(|x| format!("  {:?}", self.nodes.get(x).unwrap()))
                .collect::<Vec<String>>()
                .join(",\n")
        );
    }

    /// Send next fanout candidates if available to whatever workers are ready
    pub fn send_more_work(&mut self) {
        // Get the next work and send it along
        let registry = self.registry();
        for x in &mut self.sorted_nodes {
            // If there are no work receivers left then we should stop trying to send
            if self.receiver.is_empty() {
                break;
            }

            let node = self.nodes.get_mut(x).unwrap();
            if matches!(node.status, FanoutNodeStatus::Queued) {
                // Send node to a work request
                while let Ok(work_sender) = self.receiver.try_recv() {
                    let node_ref = node.node_ref.clone();
                    if work_sender.send(node_ref).is_ok() {
                        // Queued -> InProgress
                        node.status = FanoutNodeStatus::InProgress;
                        veilid_log!(registry debug "FanoutQueue::next: => {}", node.node_ref);
                        break;
                    }
                }
            }
        }
    }

    /// Transition node InProgress -> Timeout
    pub fn timeout(&mut self, node_ref: NodeRef) {
        let key = node_ref.node_ids().get(self.crypto_kind).unwrap();
        let node = self.nodes.get_mut(&key).unwrap();
        assert_eq!(node.status, FanoutNodeStatus::InProgress);
        node.status = FanoutNodeStatus::Timeout;
    }

    /// Transition node InProgress -> Rejected
    pub fn rejected(&mut self, node_ref: NodeRef) {
        let key = node_ref.node_ids().get(self.crypto_kind).unwrap();
        let node = self.nodes.get_mut(&key).unwrap();
        assert_eq!(node.status, FanoutNodeStatus::InProgress);
        node.status = FanoutNodeStatus::Rejected;

        self.disqualify();
    }

    /// Transition node InProgress -> Accepted
    pub fn accepted(&mut self, node_ref: NodeRef) {
        let key = node_ref.node_ids().get(self.crypto_kind).unwrap();
        let node = self.nodes.get_mut(&key).unwrap();
        assert_eq!(node.status, FanoutNodeStatus::InProgress);
        node.status = FanoutNodeStatus::Accepted;
    }

    /// Transition node InProgress -> Stale
    pub fn stale(&mut self, node_ref: NodeRef) {
        let key = node_ref.node_ids().get(self.crypto_kind).unwrap();
        let node = self.nodes.get_mut(&key).unwrap();
        assert_eq!(node.status, FanoutNodeStatus::InProgress);
        node.status = FanoutNodeStatus::Stale;
    }

    /// Transition all Accepted -> Queued, in the event a newer value for consensus is found and we want to try again
    pub fn all_accepted_to_queued(&mut self) {
        for node in &mut self.nodes {
            if matches!(node.1.status, FanoutNodeStatus::Accepted) {
                node.1.status = FanoutNodeStatus::Queued;
            }
        }
    }

    /// Transition all Accepted -> Stale, in the event a newer value for consensus is found but we don't want to try again
    pub fn all_accepted_to_stale(&mut self) {
        for node in &mut self.nodes {
            if matches!(node.1.status, FanoutNodeStatus::Accepted) {
                node.1.status = FanoutNodeStatus::Stale;
            }
        }
    }

    /// Transition all Queued | InProgress -> Timeout, in the event that the fanout is being cut short by a timeout
    pub fn all_unfinished_to_timeout(&mut self) {
        for node in &mut self.nodes {
            if matches!(
                node.1.status,
                FanoutNodeStatus::Queued | FanoutNodeStatus::InProgress
            ) {
                node.1.status = FanoutNodeStatus::Timeout;
            }
        }
    }

    /// Transition Queued -> Disqualified that are too far away from the record key
    fn disqualify(&mut self) {
        let mut consecutive_rejections = 0usize;
        let mut rejected_consensus = false;
        for node_id in &self.sorted_nodes {
            let node = self.nodes.get_mut(node_id).unwrap();
            if !rejected_consensus {
                if matches!(node.status, FanoutNodeStatus::Rejected) {
                    consecutive_rejections += 1;
                    if consecutive_rejections >= self.consensus_count {
                        rejected_consensus = true;
                    }
                    continue;
                } else {
                    consecutive_rejections = 0;
                }
            } else if matches!(node.status, FanoutNodeStatus::Queued) {
                node.status = FanoutNodeStatus::Disqualified;
            }
        }
    }

    /// Review the nodes in the queue
    pub fn with_nodes<
        R,
        F: FnOnce(&HashMap<TypedPublicKey, FanoutNode>, &[TypedPublicKey]) -> R,
    >(
        &self,
        func: F,
    ) -> R {
        func(&self.nodes, &self.sorted_nodes)
    }
}
