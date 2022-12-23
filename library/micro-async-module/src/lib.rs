//! Support async runtime and config app.
//!
//! Support a module to run and context store.
//!
//! Use metamsg or memory channel to send and recv message.

use std::fmt::Debug;
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io;
use tokio::runtime::{Builder, Runtime};

#[derive(Debug)]
pub struct App {
    modules: Vec<Box<dyn Module>>,
    runtime: AsyncRuntime,
}

#[derive(Debug, Clone)]
pub struct AsyncRuntime {
    inner: Arc<Runtime>,
    max_threads: usize,
    max_blocking_threads: usize,
}

impl App {
    pub fn new(app_name: String, max_threads: usize, max_blocking_threads: usize) -> App {
        let app = App {
            modules: Vec::new(),
            runtime: AsyncRuntime {
                inner: Arc::new(
                    multi_thread_runtime(app_name, max_threads, max_blocking_threads).unwrap(),
                ),
                max_threads,
                max_blocking_threads,
            },
        };
        app
    }

    pub fn on_startup<F>(self, f: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        f();
        self
    }

    pub fn add_module<T>(mut self, module: T) -> Self
    where
        T: Module + 'static,
    {
        let box_module = Box::new(module);
        self.modules.push(box_module);
        self
    }

    pub fn run(self) {
        self.modules
            .into_iter()
            .for_each(|module| module.start(self.runtime.clone()))
    }

    pub fn get_async_thread(&self) -> usize {
        self.runtime.max_threads
    }

    pub fn get_blocking_thread(&self) -> usize {
        self.runtime.max_blocking_threads
    }
}

/// micro async module.
pub trait Module: Debug {
    fn name(&self) -> &str;

    fn start(&self, runtime: AsyncRuntime);

    fn send(&self) {}

    fn recv(&self) {}
}

pub fn run_async_block_on<F>(spawn_closure: F, runtime: AsyncRuntime)
where
    F: Future<Output = ()> + Send + 'static,
{
    runtime.inner.block_on(spawn_closure);
}

fn run_on_block_thread<F, R>(f: F, runtime: AsyncRuntime)
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    runtime.inner.spawn_blocking(f);
}

pub fn shutdown_graceful(_module: Box<dyn Module>) {}

fn multi_thread_runtime(
    app_name: String,
    max_threads: usize,
    max_blocking_thread: usize,
) -> io::Result<Runtime> {
    Builder::new_multi_thread()
        .worker_threads(max_threads)
        .max_blocking_threads(max_blocking_thread)
        .thread_name_fn(move || {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("{}-worker-{}", app_name, id)
        })
        .enable_all()
        .build()
}
