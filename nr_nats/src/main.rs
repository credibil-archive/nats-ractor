use anyhow::Result;
use cg_nano::{ServerBuilder, SubscriptionEndpoint};
use ractor::{Actor, concurrency::{sleep, Duration}};
use std::sync::Arc;

mod handler;

/// NATS-based microservice. Uses the Credibil cg-nano crate.
#[tokio::main]
async fn main() -> Result<()>{

    // TCP port this ractor cluster node will listen on
    let port: u16 = 9001;
    // TCP port of another node in the cluster that will be used to run the
    // actor.
    let remote_port: u16 = 9000;
    let cookie = "cookie".to_string();
    let host = "localhost".to_string();
    let node_name = "nodeB".to_string();

    // Create a new node server
    let server = ractor_cluster::NodeServer::new(port, cookie, node_name.clone(), host, None, None);
    let (server_actor, server_handle) = Actor::spawn(None, server, ()).await?;
    println!("Ractor node server {} started on port {}", node_name.clone(), port);

    // Delay to allow the server to start
    sleep(Duration::from_millis(2000)).await;

    // Handler for the "greetings" subject. Add node and remote server info.
    let endpoint = SubscriptionEndpoint {
        is_request: true,
        subject: "greetings".to_string(),
        handler: Arc::new(handler::Handler {
            node_server_ref: server_actor.clone(),
            remote_actor_port: remote_port,
        }),
    };

    // Configure NATS client
    let mut svr = ServerBuilder::new()
        .connection("127.0.0.1:4222", "", "")
        .subscription(endpoint)
        .build();

    // Connect and serve
    match svr.connect().await {
        Ok(_) => println!("Connected to NATS server"),
        Err(e) => println!("Failed to connect to NATS server: {:?}", e),
    }
    svr.serve().await?;

    // Wait for ctrl-c then clean up
    tokio::signal::ctrl_c().await?;
    server_actor.stop(None);
    server_handle.await?;

    Ok(())
}
