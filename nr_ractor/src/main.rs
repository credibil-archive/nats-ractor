use anyhow::Result;
use ractor::Actor;
use ractor_cluster::NodeServer;

/// Using "current_thread" flavor so that tokio runs async in a single thread
/// to explore the idea of running such a service in a WASM container.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {

    // TCP port this ractor cluster node will listen on
    let port: u16 = 9000;
    let cookie = "cookie".to_string();
    let host = "localhost".to_string();
    let node_name = "nodeA".to_string();

    // Create a new node server
    let server = NodeServer::new(port, cookie, node_name.clone(), host, None, None);
    let (server_actor, server_handle) = Actor::spawn(None, server, ()).await?;
    println!("Ractor node server {} started on port {}", node_name.clone(), port);

    // Use ctrl-c to stop and then clean up
    tokio::signal::ctrl_c().await?;
    server_actor.stop(None);
    server_handle.await?;

    Ok(())
}
