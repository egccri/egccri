mod clients;
mod config;
mod context;
mod message;
mod server;

use crate::config::config::{ClientProtocol, EdgeHubConfig, ServerProtocol};
use clap::builder::TypedValueParser;
use clap::{Arg, Command};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    log4rs::init_file("egccri-hub/log4rs.yaml", Default::default()).unwrap();
    let matches = Command::new("Edge-Hub, a edge hub for edges.")
        .version("0.0.0")
        .about("Yang <zifeng.1024@gmail.com>")
        .arg(
            Arg::new("server-protocol")
                .short('s')
                .long("server-protocol")
                .value_name("SERVER_PROTOCOL")
                .default_value("GRPC")
                .help("The server protocol that use between edge-core and edge-hub"),
        )
        .arg(
            Arg::new("client-protocol")
                .short('c')
                .long("client-protocol")
                .value_name("CLIENT_PROTOCOL")
                .default_value("KAFKA")
                .help("The client protocol that use between edge-hub and user mq"),
        )
        .get_matches();

    let server_protocol = matches.get_one::<String>("server-protocol").unwrap();
    let server_protocol: ServerProtocol =
        ServerProtocol::from_str(server_protocol.as_str()).unwrap();
    let client_protocol = matches.get_one::<String>("client-protocol").unwrap();
    let client_protocol: ClientProtocol =
        ClientProtocol::from_str(client_protocol.as_str()).unwrap();
    let edge_hub_config = EdgeHubConfig::new(server_protocol, client_protocol);
    server::start(&edge_hub_config).await;
    clients::start(&edge_hub_config);
}

#[cfg(test)]
mod tests {
    use egccri_api::helloworld::greeter_client::GreeterClient;
    use egccri_api::helloworld::HelloRequest;

    #[tokio::test]
    async fn test_client() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = GreeterClient::connect("http://[::1]:50051").await?;

        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });

        let response = client.say_hello(request).await?;

        println!("RESPONSE={:?}", response);

        Ok(())
    }
}
