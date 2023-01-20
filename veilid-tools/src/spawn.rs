use super::*;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use async_executors::{Bindgen, LocalSpawnHandleExt, SpawnHandleExt};

        pub fn spawn<Out>(future: impl Future<Output = Out> + Send + 'static) -> MustJoinHandle<Out>
        where
            Out: Send + 'static,
        {
            MustJoinHandle::new(
                Bindgen
                    .spawn_handle(future)
                    .expect("wasm-bindgen-futures spawn_handle_local should never error out"),
            )
        }

        pub fn spawn_local<Out>(future: impl Future<Output = Out> + 'static) -> MustJoinHandle<Out>
        where
            Out: 'static,
        {
            MustJoinHandle::new(
                Bindgen
                    .spawn_handle_local(future)
                    .expect("wasm-bindgen-futures spawn_handle_local should never error out"),
            )
        }

        pub fn spawn_detached<Out>(future: impl Future<Output = Out> + Send + 'static)
        where
            Out: Send + 'static,
        {
            Bindgen
                .spawn_handle_local(future)
                .expect("wasm-bindgen-futures spawn_handle_local should never error out")
                .detach()
        }
        pub fn spawn_detached_local<Out>(future: impl Future<Output = Out> + 'static)
        where
            Out: 'static,
        {
            Bindgen
                .spawn_handle_local(future)
                .expect("wasm-bindgen-futures spawn_handle_local should never error out")
                .detach()
        }

    } else {

        pub fn spawn<Out>(future: impl Future<Output = Out> + Send + 'static) -> MustJoinHandle<Out>
        where
            Out: Send + 'static,
        {
            cfg_if! {
                if #[cfg(feature="rt-async-std")] {
                    MustJoinHandle::new(async_std::task::spawn(future))
                } else if #[cfg(feature="rt-tokio")] {
                    MustJoinHandle::new(tokio::task::spawn(future))
                }
            }
        }

        pub fn spawn_local<Out>(future: impl Future<Output = Out> + 'static) -> MustJoinHandle<Out>
        where
            Out: 'static,
        {
            cfg_if! {
                if #[cfg(feature="rt-async-std")] {
                    MustJoinHandle::new(async_std::task::spawn_local(future))
                } else if #[cfg(feature="rt-tokio")] {
                    MustJoinHandle::new(tokio::task::spawn_local(future))
                }
            }
        }

        pub fn spawn_detached<Out>(future: impl Future<Output = Out> + Send + 'static)
        where
            Out: Send + 'static,
        {
            cfg_if! {
                if #[cfg(feature="rt-async-std")] {
                    drop(async_std::task::spawn(future));
                } else if #[cfg(feature="rt-tokio")] {
                    drop(tokio::task::spawn(future));
                }
            }
        }

        pub fn spawn_detached_local<Out>(future: impl Future<Output = Out> + 'static)
        where
            Out: 'static,
        {
            cfg_if! {
                if #[cfg(feature="rt-async-std")] {
                    drop(async_std::task::spawn_local(future));
                } else if #[cfg(feature="rt-tokio")] {
                    drop(tokio::task::spawn_local(future));
                }
            }
        }

        #[allow(unused_variables)]
        pub async fn blocking_wrapper<F, R>(blocking_task: F, err_result: R) -> R
        where
            F: FnOnce() -> R + Send + 'static,
            R: Send + 'static,
        {
            // run blocking stuff in blocking thread
            cfg_if! {
                if #[cfg(feature="rt-async-std")] {
                    async_std::task::spawn_blocking(blocking_task).await
                } else if #[cfg(feature="rt-tokio")] {
                    tokio::task::spawn_blocking(blocking_task).await.unwrap_or(err_result)
                } else {
                    #[compile_error("must use an executor")]
                }
            }
        }
    }
}
