use micro_async_module::{run_async_block_on, AsyncRuntime, Module};
use once_cell::sync::OnceCell;
use sqlx::{Executor, Pool, Sqlite, SqlitePool};
use thiserror::Error;
use tracing::info;

static POLL: OnceCell<Pool<Sqlite>> = OnceCell::new();

const MODULE_NAME: &str = "storage_sqlite";

#[derive(Debug)]
pub struct StorageSqlite;

impl Module for StorageSqlite {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    fn start(&self, runtime: AsyncRuntime) {
        run_async_block_on(init(), runtime);
    }
}

async fn init() {
    let poll = init_db_pool().await;
    POLL.set(poll).expect("Create sqlite error!");
    initial_table().await;
}

async fn init_db_pool() -> Pool<Sqlite> {
    SqlitePool::connect("egccri-storage.sqlite").await.unwrap()
}

async fn initial_table() {
    // create table with flag.
    info!("Initial table with flag.");
    POLL.get().unwrap().execute("select 1");
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("storage sqlite error from {0:?}")]
    SqliteError(#[from] sqlx::Error),
}

/// storage handler
pub trait StorageHandler {
    fn on_add(&self);

    fn on_update(&self);

    fn on_delete(&self);
}
