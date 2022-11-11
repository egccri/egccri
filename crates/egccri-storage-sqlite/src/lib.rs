mod device_storage;

use lazy_static::lazy_static;
use micro_async_module::run_block_on;
use micro_async_module::{Config, Module};
use sqlx::{Pool, Sqlite, SqlitePool};
use thiserror::Error;
use tracing::info;

lazy_static! {
    pub static ref pool: Pool<Sqlite> = SqlitePool::connect("egccri-storage.db").await?;
}

pub struct StorageSqlite;

impl Module for StorageSqlite {
    fn config(&self) -> Config {
        Config {
            name: "Storage sqlite".to_string(),
            max_threads: 2,
        }
    }

    fn start(&self) {
        run_block_on(initial_table(), self.config());
    }

    fn context(&self) {
        todo!()
    }
}

async fn initial_table() {
    // create table with flag.
    info!("Initial table with flag.")
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("storage sqlite error from {0:?}")]
    SqliteError(#[from] sqlx::Error),
}
