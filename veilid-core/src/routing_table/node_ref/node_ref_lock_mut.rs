use super::*;

pub type LockedMutNodeRef<'a> = NodeRefLockMut<'a, NodeRef>;
pub type LockedMutFilteredNodeRef<'a> = NodeRefLockMut<'a, FilteredNodeRef>;

/// Mutable locked reference to a routing table entry
/// For internal use inside the RoutingTable module where you have
/// already locked a RoutingTableInner
/// Keeps entry in the routing table until all references are gone
pub struct NodeRefLockMut<
    'a,
    N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone,
> {
    inner: Mutex<&'a mut RoutingTableInner>,
    nr: N,
}

impl<'a, N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    NodeRefLockMut<'a, N>
{
    pub fn new(inner: &'a mut RoutingTableInner, nr: N) -> Self {
        Self {
            inner: Mutex::new(inner),
            nr,
        }
    }

    #[expect(dead_code)]
    pub fn unlocked(&self) -> N {
        self.nr.clone()
    }
}

impl<'a, N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    NodeRefAccessorsTrait for NodeRefLockMut<'a, N>
{
    fn routing_table(&self) -> RoutingTable {
        self.nr.routing_table()
    }
    fn entry(&self) -> Arc<BucketEntry> {
        self.nr.entry()
    }

    fn sequencing(&self) -> Sequencing {
        self.nr.sequencing()
    }

    fn routing_domain_set(&self) -> RoutingDomainSet {
        self.nr.routing_domain_set()
    }

    fn filter(&self) -> NodeRefFilter {
        self.nr.filter()
    }

    fn take_filter(&mut self) -> NodeRefFilter {
        self.nr.take_filter()
    }

    fn dial_info_filter(&self) -> DialInfoFilter {
        self.nr.dial_info_filter()
    }
}

impl<'a, N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    NodeRefOperateTrait for NodeRefLockMut<'a, N>
{
    fn operate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> T,
    {
        let inner = &*self.inner.lock();
        self.nr.entry().with(inner, f)
    }

    fn operate_mut<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> T,
    {
        let inner = &mut *self.inner.lock();
        self.nr.entry().with_mut(inner, f)
    }
}

impl<'a, N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    NodeRefCommonTrait for NodeRefLockMut<'a, N>
{
}

impl<'a, N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    fmt::Display for NodeRefLockMut<'a, N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nr)
    }
}

impl<'a, N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    fmt::Debug for NodeRefLockMut<'a, N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRefLockMut")
            .field("nr", &self.nr)
            .finish()
    }
}
