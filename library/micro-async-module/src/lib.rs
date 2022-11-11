//! Support async runtime and config app.
//!
//! Support a module to run and context store.
//!
//! Use metamsg or memory channel to send and recv message.

use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime::{Builder, Runtime};
use tokio::task;

/// micro async module.
pub trait Module {
    fn config(&self) -> Config;

    fn start(&self);

    fn context(&self);
}

pub struct Config {
    pub name: String,
    pub max_threads: usize,
}

pub fn run_block_on<F>(spawn_closure: F, async_module_config: Config)
where
    F: Future<Output = ()> + Send + 'static,
{
    let module_name = async_module_config.name.clone();
    Builder::new_multi_thread()
        .worker_threads(async_module_config.max_threads)
        .thread_name_fn(move || {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("{}-worker-{}", module_name, id)
        })
        .enable_all()
        .build()
        .unwrap()
        .block_on(spawn_closure);
}

pub fn shutdown_graceful<M>(async_module: M) {}
