mod filtered_node_ref;
mod node_ref_filter;
mod node_ref_lock;
mod node_ref_lock_mut;
mod traits;

impl_veilid_log_facility!("rtab");

use super::*;

pub(crate) use filtered_node_ref::*;
pub(crate) use node_ref_filter::*;
pub(crate) use node_ref_lock::*;
pub(crate) use node_ref_lock_mut::*;
pub(crate) use traits::*;

///////////////////////////////////////////////////////////////////////////
// Default NodeRef

pub(crate) struct NodeRef {
    registry: VeilidComponentRegistry,
    entry: Arc<BucketEntry>,
    #[cfg(feature = "tracking")]
    track_id: usize,
}

impl_veilid_component_registry_accessor!(NodeRef);

impl NodeRef {
    pub fn new(registry: VeilidComponentRegistry, entry: Arc<BucketEntry>) -> Self {
        entry.ref_count.fetch_add(1u32, Ordering::AcqRel);

        Self {
            registry,
            entry,
            #[cfg(feature = "tracking")]
            track_id: entry.track(),
        }
    }

    pub fn default_filtered(&self) -> FilteredNodeRef {
        FilteredNodeRef::new(
            self.registry.clone(),
            self.entry.clone(),
            NodeRefFilter::new(),
            Sequencing::default(),
        )
    }

    pub fn sequencing_filtered(&self, sequencing: Sequencing) -> FilteredNodeRef {
        FilteredNodeRef::new(
            self.registry.clone(),
            self.entry.clone(),
            NodeRefFilter::new(),
            sequencing,
        )
    }

    pub fn routing_domain_filtered<R: Into<RoutingDomainSet>>(
        &self,
        routing_domain_set: R,
    ) -> FilteredNodeRef {
        FilteredNodeRef::new(
            self.registry.clone(),
            self.entry.clone(),
            NodeRefFilter::new().with_routing_domain_set(routing_domain_set.into()),
            Sequencing::default(),
        )
    }

    pub fn custom_filtered(&self, filter: NodeRefFilter) -> FilteredNodeRef {
        FilteredNodeRef::new(
            self.registry.clone(),
            self.entry.clone(),
            filter,
            Sequencing::default(),
        )
    }

    #[expect(dead_code)]
    pub fn dial_info_filtered(&self, filter: DialInfoFilter) -> FilteredNodeRef {
        FilteredNodeRef::new(
            self.registry.clone(),
            self.entry.clone(),
            NodeRefFilter::new().with_dial_info_filter(filter),
            Sequencing::default(),
        )
    }

    pub fn locked<'a>(&self, rti: &'a RoutingTableInner) -> LockedNodeRef<'a> {
        LockedNodeRef::new(rti, self.clone())
    }
    pub fn locked_mut<'a>(&self, rti: &'a mut RoutingTableInner) -> LockedMutNodeRef<'a> {
        LockedMutNodeRef::new(rti, self.clone())
    }
}

impl NodeRefAccessorsTrait for NodeRef {
    fn entry(&self) -> Arc<BucketEntry> {
        self.entry.clone()
    }

    fn sequencing(&self) -> Sequencing {
        Sequencing::NoPreference
    }

    fn routing_domain_set(&self) -> RoutingDomainSet {
        RoutingDomainSet::all()
    }

    fn filter(&self) -> NodeRefFilter {
        NodeRefFilter::new()
    }

    fn take_filter(&mut self) -> NodeRefFilter {
        NodeRefFilter::new()
    }

    fn dial_info_filter(&self) -> DialInfoFilter {
        DialInfoFilter::all()
    }
}

impl NodeRefOperateTrait for NodeRef {
    fn operate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> T,
    {
        let routing_table = self.routing_table();
        let inner = &*routing_table.inner.read();
        self.entry.with(inner, f)
    }

    fn operate_mut<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> T,
    {
        let routing_table = self.routing_table();
        let inner = &mut *routing_table.inner.write();
        self.entry.with_mut(inner, f)
    }
}

impl NodeRefCommonTrait for NodeRef {}

impl Clone for NodeRef {
    fn clone(&self) -> Self {
        self.entry.ref_count.fetch_add(1u32, Ordering::AcqRel);

        Self {
            registry: self.registry.clone(),
            entry: self.entry.clone(),
            #[cfg(feature = "tracking")]
            track_id: self.entry.write().track(),
        }
    }
}

impl fmt::Display for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.entry.with_inner(|e| e.best_node_id()))
    }
}

impl fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRef")
            .field("node_ids", &self.entry.with_inner(|e| e.node_ids()))
            .finish()
    }
}

impl Drop for NodeRef {
    fn drop(&mut self) {
        #[cfg(feature = "tracking")]
        self.entry.write().untrack(self.track_id);

        // drop the noderef and queue a bucket kick if it was the last one
        let new_ref_count = self.entry.ref_count.fetch_sub(1u32, Ordering::AcqRel) - 1;
        if new_ref_count == 0 {
            // get node ids with inner unlocked because nothing could be referencing this entry now
            // and we don't know when it will get dropped, possibly inside a lock
            let node_ids = self.entry.with_inner(|e| e.node_ids());
            self.routing_table().queue_bucket_kicks(node_ids);
        }
    }
}
