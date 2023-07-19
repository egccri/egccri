use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};

// FIXME support concurrent items per token and retry build.
// FIXME Add check item alive policy.
#[async_trait::async_trait]
pub trait PoolItemBuilder: Clone {
    type Token;
    type Item;
    type Error;

    async fn build(&self, token: &Self::Token) -> Result<Self::Item, Self::Error>;

    /// When check is false, pool remove item.
    async fn check(&self) -> Result<bool, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct MutexPool<ItemBuilder: PoolItemBuilder + Debug> {
    inner: Arc<Mutex<HashMap<ItemBuilder::Token, ItemBuilder::Item>>>,

    item_builder: ItemBuilder,

    retry_build_config: Option<RetryConfig>,
}

#[derive(Debug, Clone)]
pub struct RwLockPool<ItemBuilder: PoolItemBuilder + Debug> {
    inner: Arc<RwLock<HashMap<ItemBuilder::Token, ItemBuilder::Item>>>,

    item_builder: ItemBuilder,

    retry_build_config: Option<RetryConfig>,
}

impl<ItemBuilder> MutexPool<ItemBuilder>
where
    ItemBuilder: PoolItemBuilder + Debug,
    ItemBuilder::Token: Clone + Eq + Hash + Send + Debug,
    ItemBuilder::Item: Clone + Sync + Send + Debug,
    ItemBuilder::Error: Sync + Debug,
{
    pub fn new(item_builder: ItemBuilder, retry_build_config: Option<RetryConfig>) -> Self {
        MutexPool {
            inner: Arc::new(Mutex::new(HashMap::new())),
            item_builder,
            retry_build_config,
        }
    }

    pub async fn get(
        &self,
        token: &ItemBuilder::Token,
    ) -> Result<ItemBuilder::Item, ItemBuilder::Error> {
        if let Some(item) = self.inner.lock().await.get(token) {
            return Ok(item.clone());
        }

        return match self.item_builder.build(token).await {
            Ok(item) => {
                let mut lock = self.inner.lock().await;
                lock.insert(token.clone(), item.clone());
                Ok(item)
            }
            Err(err) => Err(err),
        };
    }
}

#[derive(Debug, Clone)]
pub enum RetryPolicy {
    FixedRetry,
    Forever,
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    retry_times: u32,
    initial_retry_interval: Duration,
    retry_policy: RetryPolicy,
}
