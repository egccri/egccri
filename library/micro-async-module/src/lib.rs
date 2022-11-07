//! Support async runtime and config app.
//!
//! Support a module to run and context store.
//!
//! Use metamsg or memory channel to send and recv message.

use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime::{Builder, Runtime};
use tokio::task;

///
pub trait Module {
    fn config(&self) -> Config;

    async fn start();

    fn context(&self);
}

pub struct Config {
    name: String,
    max_threads: usize,
}

pub fn run_block_on<F, M>(spawn_closure: F, async_module: M)
where
    F: Future<Output = ()> + Send + 'static,
    M: Module,
{
    Builder::new_multi_thread()
        .worker_threads(async_module.config().max_threads)
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("{}-worker-{}", async_module.config().name, id)
        })
        .enable_all()
        .build()
        .unwrap()
        .block_on(spawn_closure);
}

pub fn shutdown_graceful<M>(async_module: M) {}
