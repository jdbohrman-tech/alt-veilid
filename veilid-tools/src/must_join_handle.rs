use super::*;

use core::task::{Context, Poll};

#[derive(Debug)]
pub struct MustJoinHandle<T> {
    join_handle: Option<LowLevelJoinHandle<T>>,
    completed: bool,
}

impl<T> MustJoinHandle<T> {
    pub fn new(join_handle: LowLevelJoinHandle<T>) -> Self {
        Self {
            join_handle: Some(join_handle),
            completed: false,
        }
    }

    pub fn detach(mut self) {
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                self.join_handle = None;
            } else if #[cfg(feature="rt-tokio")] {
                self.join_handle = None;
            } else if #[cfg(target_arch = "wasm32")] {
                if let Some(jh) = self.join_handle.take() {
                    jh.detach();
                }
            } else {
                compile_error!("needs executor implementation")
            }
        }
        self.completed = true;
    }

    #[allow(unused_mut)]
    pub async fn abort(mut self) {
        if !self.completed {
            cfg_if! {
                if #[cfg(feature="rt-async-std")] {
                    if let Some(jh) = self.join_handle.take() {
                        jh.cancel().await;
                        self.completed = true;
                    }
                } else if #[cfg(feature="rt-tokio")] {
                    if let Some(jh) = self.join_handle.take() {
                        jh.abort();
                        let _ = jh.await;
                        self.completed = true;
                    }
                } else if #[cfg(target_arch = "wasm32")] {
                    drop(self.join_handle.take());
                    self.completed = true;
                } else {
                    compile_error!("needs executor implementation")
                }

            }
        }
    }
}

impl<T> Drop for MustJoinHandle<T> {
    fn drop(&mut self) {
        // panic if we haven't completed
        if !self.completed {
            panic!("MustJoinHandle was not completed upon drop. Add cooperative cancellation where appropriate to ensure this is completed before drop.")
        }
    }
}

impl<T: 'static> Future for MustJoinHandle<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(self.join_handle.as_mut().unwrap()).poll(cx) {
            Poll::Ready(t) => {
                if self.completed {
                    panic!("should not poll completed join handle");
                }
                self.completed = true;
                cfg_if! {
                    if #[cfg(feature="rt-async-std")] {
                        Poll::Ready(t)
                    } else if #[cfg(feature="rt-tokio")] {
                        match t {
                            Ok(t) => Poll::Ready(t),
                            Err(e) => {
                                if e.is_panic() {
                                    // Resume the panic on the main task
                                    std::panic::resume_unwind(e.into_panic());
                                } else {
                                    panic!("join error was not a panic, should not poll after abort");
                                }
                            }
                        }
                    } else if #[cfg(target_arch = "wasm32")] {
                        Poll::Ready(t)
                    } else {
                        compile_error!("needs executor implementation")
                    }
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
