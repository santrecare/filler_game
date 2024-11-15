use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientType {
    Player,
    Spectator,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessage {
    pub message_type: String,
    pub client_type: ClientType,
    pub client_name: String,
    pub client_id: Option<String>,
    pub data: Value,
}
