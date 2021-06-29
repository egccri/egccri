#[tokio::main]
async fn main() {
    log4rs::init_file("egccri-core/log4rs.yaml", Default::default()).unwrap();

    println!("Hello, world!");
}
