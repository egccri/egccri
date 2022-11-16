use lazy_static::lazy_static;
use micro_async_module::run_block_on;
use micro_async_module::{Config, Module};
use once_cell::sync::OnceCell;
use sqlx::{Executor, Pool, Sqlite, SqlitePool};
use thiserror::Error;
use tracing::info;

static POLL: OnceCell<Pool<Sqlite>> = OnceCell::new();

pub struct StorageSqlite;

impl Module for StorageSqlite {
    fn config(&self) -> Config {
        Config {
            name: "Storage sqlite".to_string(),
            max_threads: 2,
        }
    }

    fn start(&self) {
        run_block_on(init(), self.config());
    }

    fn context(&self) {
        todo!()
    }
}

async fn init() {
    let poll = init_db_pool().await;
    CONNS.set(poll).expect("Create sqlite error!");
    initial_table();
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
