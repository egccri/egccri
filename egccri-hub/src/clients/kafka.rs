use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;

#[allow(dead_code)]
pub fn create_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("queue.buffering.max.ms", "0")
        .create()
        .expect("Producer create failed")
}
