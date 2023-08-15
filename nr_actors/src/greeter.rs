use ractor::{Actor, RpcReplyPort, BytesConvertable, ActorRef, ActorProcessingErr};
use ractor_cluster::RactorClusterMessage;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Greeting {
    pub name: String,
}

impl BytesConvertable for Greeting {
    /// Serialize this type to a vector of bytes. Panics are acceptable
    fn into_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
    /// Deserialize this type from a vector of bytes. Panics are acceptable
    fn from_bytes(bytes: Vec<u8>) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }
}

#[derive(RactorClusterMessage)]
pub enum GreeterMessage {
    #[rpc]
    Greet(Greeting, RpcReplyPort<String>),
}

pub struct Greeter;

#[async_trait::async_trait]
impl Actor for Greeter {
    type State = ();
    type Msg = GreeterMessage;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _arguments: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            GreeterMessage::Greet(greeting, reply_port) => {
                if reply_port.send(format!("Hello, {}!", greeting.name)).is_err() {
                    println!("Failed to send reply");
                }
            },
        }
        Ok(())
    }
}
