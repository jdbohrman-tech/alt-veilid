use super::*;

pub(crate) type LockedNodeRef<'a> = NodeRefLock<'a, NodeRef>;
pub(crate) type LockedFilteredNodeRef<'a> = NodeRefLock<'a, FilteredNodeRef>;

/// Locked reference to a routing table entry
/// For internal use inside the RoutingTable module where you have
/// already locked a RoutingTableInner
/// Keeps entry in the routing table until all references are gone
pub(crate) struct NodeRefLock<
    'a,
    N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone,
> {
    inner: Mutex<&'a RoutingTableInner>,
    nr: N,
}

impl<
        N: NodeRefAccessorsTrait
            + NodeRefOperateTrait
            + VeilidComponentRegistryAccessor
            + fmt::Debug
            + fmt::Display
            + Clone,
    > VeilidComponentRegistryAccessor for NodeRefLock<'_, N>
{
    fn registry(&self) -> VeilidComponentRegistry {
        self.nr.registry()
    }
}

impl<'a, N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    NodeRefLock<'a, N>
{
    pub fn new(inner: &'a RoutingTableInner, nr: N) -> Self {
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

impl<N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    NodeRefAccessorsTrait for NodeRefLock<'_, N>
{
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

impl<N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    NodeRefOperateTrait for NodeRefLock<'_, N>
{
    fn operate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> T,
    {
        let inner = &*self.inner.lock();
        self.nr.entry().with(inner, f)
    }

    fn operate_mut<T, F>(&self, _f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> T,
    {
        panic!("need to locked_mut() for this operation")
    }

    fn with_inner<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner) -> T,
    {
        let inner = &*self.inner.lock();
        f(inner)
    }

    fn with_inner_mut<T, F>(&self, _f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner) -> T,
    {
        panic!("need to locked_mut() for this operation")
    }
}

impl<N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    NodeRefCommonTrait for NodeRefLock<'_, N>
{
}

impl<N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone>
    fmt::Display for NodeRefLock<'_, N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nr)
    }
}

impl<N: NodeRefAccessorsTrait + NodeRefOperateTrait + fmt::Debug + fmt::Display + Clone> fmt::Debug
    for NodeRefLock<'_, N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRefLock").field("nr", &self.nr).finish()
    }
}
