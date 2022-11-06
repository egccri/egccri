mod device_storage;

use lazy_static::lazy_static;
use sqlx::{Pool, Sqlite, SqlitePool};
use thiserror::Error;

lazy_static! {
    pub static ref popl: Pool<Sqlite> = initial_store();
}

fn start() {
    // create table with flag.
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("storage sqlite error from {0:?}")]
    SqliteError(#[from] sqlx::Error),
}

async fn initial_store() -> Pool<Sqlite> {
    let pool = SqlitePool::connect("egccri-storage.db").await?;
    pool
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
