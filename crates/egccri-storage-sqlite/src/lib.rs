mod device_storage;

use lazy_static::lazy_static;
use micro_async_module::run_block_on;
use micro_async_module::{Config, Module};
use sqlx::{Pool, Sqlite, SqlitePool};
use thiserror::Error;

lazy_static! {
    pub static ref pool: Pool<Sqlite> = SqlitePool::connect("egccri-storage.db").await?;
}

struct StorageSqlite;

impl Module for StorageSqlite {
    fn config(&self) -> Config {
        todo!()
    }

    async fn start() {
        initial_table().await
    }

    fn context(&self) {
        todo!()
    }
}

async fn initial_table() {
    // create table with flag.
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("storage sqlite error from {0:?}")]
    SqliteError(#[from] sqlx::Error),
}
