use super::*;

impl_veilid_log_facility!("registry");

pub(crate) trait AsAnyArcSendSync {
    fn as_any_arc_send_sync(self: Arc<Self>) -> Arc<dyn core::any::Any + Send + Sync>;
}

impl<T: Send + Sync + 'static> AsAnyArcSendSync for T {
    fn as_any_arc_send_sync(self: Arc<Self>) -> Arc<dyn core::any::Any + Send + Sync> {
        self
    }
}

pub(crate) trait VeilidComponent:
    AsAnyArcSendSync + VeilidComponentRegistryAccessor + core::fmt::Debug
{
    fn name(&self) -> &'static str;
    fn init(&self) -> PinBoxFuture<'_, EyreResult<()>>;
    fn post_init(&self) -> PinBoxFuture<'_, EyreResult<()>>;
    fn pre_terminate(&self) -> PinBoxFuture<'_, ()>;
    fn terminate(&self) -> PinBoxFuture<'_, ()>;
}

pub(crate) trait VeilidComponentRegistryAccessor {
    fn registry(&self) -> VeilidComponentRegistry;

    fn config(&self) -> VeilidStartupOptions {
        self.registry().config.clone()
    }
    fn update_callback(&self) -> UpdateCallback {
        self.registry().config.update_callback()
    }
    fn event_bus(&self) -> EventBus {
        self.registry().event_bus.clone()
    }
    fn log_key(&self) -> &'static str {
        self.registry().log_key()
    }
}

pub struct VeilidComponentGuard<'a, T: Send + Sync + 'static> {
    component: Arc<T>,
    _phantom: core::marker::PhantomData<&'a T>,
}

impl<T> core::ops::Deref for VeilidComponentGuard<'_, T>
where
    T: Send + Sync + 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

#[derive(Debug)]
struct VeilidComponentRegistryInner {
    type_map: HashMap<core::any::TypeId, Arc<dyn VeilidComponent + Send + Sync>>,
    init_order: Vec<core::any::TypeId>,
    mock: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct VeilidComponentRegistry {
    inner: Arc<Mutex<VeilidComponentRegistryInner>>,
    config: VeilidStartupOptions,
    namespace: &'static str,
    program_name: &'static str,
    log_key: &'static str,
    event_bus: EventBus,
    init_lock: Arc<AsyncMutex<bool>>,
}

impl VeilidComponentRegistry {
    pub fn new(config: VeilidStartupOptions) -> Self {
        let (namespace, program_name) =
            config.with(|c| (c.namespace.to_static_str(), c.program_name.to_static_str()));

        let log_key = VeilidLayerFilter::make_veilid_log_key(program_name, namespace);

        Self {
            inner: Arc::new(Mutex::new(VeilidComponentRegistryInner {
                type_map: HashMap::new(),
                init_order: Vec::new(),
                mock: false,
            })),
            config,
            namespace,
            program_name,
            log_key,
            event_bus: EventBus::new(),
            init_lock: Arc::new(AsyncMutex::new(false)),
        }
    }

    pub fn enable_mock(&self) {
        let mut inner = self.inner.lock();
        inner.mock = true;
    }

    #[expect(dead_code)]
    pub fn namespace(&self) -> &'static str {
        self.namespace
    }

    #[allow(dead_code)]
    pub fn program_name(&self) -> &'static str {
        self.program_name
    }

    pub fn log_key(&self) -> &'static str {
        self.log_key
    }

    pub fn register<
        T: VeilidComponent + Send + Sync + 'static,
        F: FnOnce(VeilidComponentRegistry) -> T,
    >(
        &self,
        component_constructor: F,
    ) {
        let component = Arc::new(component_constructor(self.clone()));
        let component_type_id = core::any::TypeId::of::<T>();

        let mut inner = self.inner.lock();
        assert!(
            inner
                .type_map
                .insert(component_type_id, component)
                .is_none(),
            "should not register same component twice"
        );
        inner.init_order.push(component_type_id);
    }

    pub fn register_with_context<
        C,
        T: VeilidComponent + Send + Sync + 'static,
        F: FnOnce(VeilidComponentRegistry, C) -> T,
    >(
        &self,
        component_constructor: F,
        context: C,
    ) {
        let component = Arc::new(component_constructor(self.clone(), context));
        let component_type_id = core::any::TypeId::of::<T>();

        let mut inner = self.inner.lock();
        assert!(
            inner
                .type_map
                .insert(component_type_id, component)
                .is_none(),
            "should not register same component twice"
        );
        inner.init_order.push(component_type_id);
    }

    pub async fn init(&self) -> EyreResult<()> {
        let Some(mut _init_guard) = asyncmutex_try_lock!(self.init_lock) else {
            bail!("init should only happen one at a time");
        };
        if *_init_guard {
            bail!("already initialized");
        }

        // Event bus starts up early
        self.event_bus.startup()?;

        // Process components in initialization order
        let init_order = self.get_init_order();
        let mut initialized = vec![];
        for component in init_order {
            if let Err(e) = component.init().await {
                self.terminate_inner(initialized).await;
                self.event_bus.shutdown().await;
                return Err(e);
            }
            initialized.push(component);
        }

        *_init_guard = true;
        Ok(())
    }

    pub async fn post_init(&self) -> EyreResult<()> {
        let Some(mut _init_guard) = asyncmutex_try_lock!(self.init_lock) else {
            bail!("init should only happen one at a time");
        };
        if !*_init_guard {
            bail!("not initialized");
        }

        let init_order = self.get_init_order();
        let mut post_initialized = vec![];
        for component in init_order {
            if let Err(e) = component.post_init().await {
                self.pre_terminate_inner(post_initialized).await;
                return Err(e);
            }
            post_initialized.push(component)
        }
        Ok(())
    }

    pub async fn pre_terminate(&self) {
        let Some(mut _init_guard) = asyncmutex_try_lock!(self.init_lock) else {
            panic!("terminate should only happen one at a time");
        };
        if !*_init_guard {
            panic!("not initialized");
        }

        let init_order = self.get_init_order();
        self.pre_terminate_inner(init_order).await;
    }

    pub async fn terminate(&self) {
        let Some(mut _init_guard) = asyncmutex_try_lock!(self.init_lock) else {
            panic!("terminate should only happen one at a time");
        };
        if !*_init_guard {
            panic!("not initialized");
        }

        // Terminate components in reverse initialization order
        let init_order = self.get_init_order();
        self.terminate_inner(init_order).await;

        // Event bus shuts down last
        self.event_bus.shutdown().await;

        *_init_guard = false;
    }

    async fn pre_terminate_inner(
        &self,
        pre_initialized: Vec<Arc<dyn VeilidComponent + Send + Sync>>,
    ) {
        for component in pre_initialized.iter().rev() {
            component.pre_terminate().await;
        }
    }
    async fn terminate_inner(&self, initialized: Vec<Arc<dyn VeilidComponent + Send + Sync>>) {
        for component in initialized.iter().rev() {
            let refs = Arc::strong_count(component);
            if refs > 2 {
                veilid_log!(self warn
                    "Terminating component '{}' while still referenced ({} extra references)",
                    component.name(),
                    refs - 2
                );
            }
            component.terminate().await;
        }
    }

    fn get_init_order(&self) -> Vec<Arc<dyn VeilidComponent + Send + Sync>> {
        let inner = self.inner.lock();
        inner
            .init_order
            .iter()
            .map(|id| inner.type_map.get(id).unwrap().clone())
            .collect::<Vec<_>>()
    }

    //////////////////////////////////////////////////////////////

    pub fn lookup<'a, T: VeilidComponent + Send + Sync + 'static>(
        &self,
    ) -> Option<VeilidComponentGuard<'a, T>> {
        let inner = self.inner.lock();
        let component_type_id = core::any::TypeId::of::<T>();
        let component_dyn = inner.type_map.get(&component_type_id)?.clone();
        let component = component_dyn
            .as_any_arc_send_sync()
            .downcast::<T>()
            .unwrap();
        Some(VeilidComponentGuard {
            component,
            _phantom: core::marker::PhantomData {},
        })
    }
}

impl VeilidComponentRegistryAccessor for VeilidComponentRegistry {
    fn registry(&self) -> VeilidComponentRegistry {
        self.clone()
    }
}

////////////////////////////////////////////////////////////////////

macro_rules! impl_veilid_component_registry_accessor {
    ($struct_name:ty) => {
        impl VeilidComponentRegistryAccessor for $struct_name {
            fn registry(&self) -> VeilidComponentRegistry {
                self.registry.clone()
            }
        }
    };
}

pub(crate) use impl_veilid_component_registry_accessor;

/////////////////////////////////////////////////////////////////////

macro_rules! impl_veilid_component {
    ($component_name:ty) => {
        impl_veilid_component_registry_accessor!($component_name);

        impl VeilidComponent for $component_name {
            fn name(&self) -> &'static str {
                stringify!($component_name)
            }

            fn init(&self) -> PinBoxFuture<'_, EyreResult<()>> {
                Box::pin(async { self.init_async().await })
            }

            fn post_init(&self) -> PinBoxFuture<'_, EyreResult<()>> {
                Box::pin(async { self.post_init_async().await })
            }

            fn pre_terminate(&self) -> PinBoxFuture<'_, ()> {
                Box::pin(async { self.pre_terminate_async().await })
            }

            fn terminate(&self) -> PinBoxFuture<'_, ()> {
                Box::pin(async { self.terminate_async().await })
            }
        }
    };
}

pub(crate) use impl_veilid_component;

/////////////////////////////////////////////////////////////////////

// Utility macro for setting up a background TickTask
// Should be called during new/construction of a component with background tasks
// and before any post-init 'tick' operations are started
macro_rules! impl_setup_task {
    ($this:expr, $this_type:ty, $task_name:ident, $task_routine:ident ) => {{
        let registry = $this.registry();
        $this.$task_name.set_routine(move |s, l, t| {
            let registry = registry.clone();
            Box::pin(async move {
                let this = registry.lookup::<$this_type>().unwrap();
                this.$task_routine(s, Timestamp::new(l), Timestamp::new(t))
                    .await
            })
        });
    }};
}

pub(crate) use impl_setup_task;

// Utility macro for setting up an event bus handler
// Should be called after init, during post-init or later
// Subscription should be unsubscribed before termination
macro_rules! impl_subscribe_event_bus {
    ($this:expr, $this_type:ty, $event_handler:ident ) => {{
        let registry = $this.registry();
        $this.event_bus().subscribe(move |evt| {
            let registry = registry.clone();
            Box::pin(async move {
                let this = registry.lookup::<$this_type>().unwrap();
                this.$event_handler(evt);
            })
        })
    }};
}

pub(crate) use impl_subscribe_event_bus;

// macro_rules! impl_subscribe_event_bus_async {
//     ($this:expr, $this_type:ty, $event_handler:ident ) => {{
//         let registry = $this.registry();
//         $this.event_bus().subscribe(move |evt| {
//             let registry = registry.clone();
//             Box::pin(async move {
//                 let this = registry.lookup::<$this_type>().unwrap();
//                 this.$event_handler(evt).await;
//             })
//         })
//     }};
// }

// pub(crate) use impl_subscribe_event_bus_async;
