use pool::MutexPool;
use tonic::transport::Channel;

#[tokio::main]
async fn main() {
    let channel_builder = ChannelBuilder;
    // hold by app or static
    let channel_pool = MutexPool::new(channel_builder, None);

    let addr = "http://127.0.0.1:50000".to_string();
    let channel = channel_pool.get(&addr).await;
    println!("channel: {:?}", channel);
    let channel = channel_pool.get(&addr).await;
    println!("channel: {:?}", channel);
}

#[derive(Debug, Clone)]
struct ChannelBuilder;

#[async_trait::async_trait]
impl pool::PoolItemBuilder for ChannelBuilder {
    type Token = String;
    type Item = Channel;
    type Error = tonic::transport::Error;

    async fn build(&self, addr: &Self::Token) -> Result<Self::Item, Self::Error> {
        println!("Building channel");
        tonic::transport::Endpoint::new(addr.clone())?
            .connect()
            .await
    }
}
