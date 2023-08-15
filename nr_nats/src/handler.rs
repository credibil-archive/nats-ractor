use anyhow::{anyhow, Result};
use async_nats::Message;
use bytes::Bytes;
use cg_nano::{decode, encode, SubscriptionHandler};
use nr_actors::{Greeting, Greeter, GreeterMessage};
use ractor::{Actor, ActorRef, concurrency::Duration};
use ractor_cluster::NodeServerMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExampleRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExampleResponse {
    pub message: String,
}

#[derive(Clone)]
pub struct Handler {
    pub node_server_ref: ActorRef<NodeServerMessage>,
    pub remote_actor_port: u16,
}

#[async_trait::async_trait]
impl SubscriptionHandler for Handler {
    async fn handle(&self, msg: Message) -> Result<Bytes> {

        let req = match decode::<ExampleRequest>(&msg.payload) {
            Ok(req) => req,
            Err(e) => {
                println!("failed to decode request: {:?}", e);
                return Err(anyhow!("failed to decode request".to_string()));
            }
        };
        println!("Greeting handler received request with name: {}", req.name);

        let (greeter, _) = Actor::spawn(None, Greeter, ()).await?;
        println!("Greeter actor spawned");

        // Send a message to the remote actor
        ractor_cluster::node::client::connect(
            &self.node_server_ref,
            format!("127.0.0.1:{}", self.remote_actor_port),
        ).await?;
        println!("Connected to remote node {}", self.remote_actor_port);

        let actor_res = greeter
            .call(
                |tx| {
                    GreeterMessage::Greet(
                        Greeting {
                            name: req.name,
                        },
                        tx,
                    )
                },
                Some(Duration::from_millis(100)),
            )
            .await?;
        if !actor_res.is_success() {
            println!("actor call failed");
            return Err(anyhow!("actor call failed".to_string()));
        }

        let msg = actor_res.unwrap();
        println!("actor response: {}", msg);

        let res = ExampleResponse {
            message: msg,
        };

        let reply = encode::<ExampleResponse>(&res)?;
        Ok(reply)
    }
}
