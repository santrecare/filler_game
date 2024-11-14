use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::accept_async;
use serde::{Serialize, Deserialize};
use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
enum ClientType {
    Player,
    Spectator,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ClientMessage {
    message_type: String,
    client_type: ClientType,
    client_name: String,
    client_id: Option<String>,
    data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GameState {
    board: Vec<Vec<i32>>,
    current_player: String,
}

#[derive(Clone, Debug)]
struct Player {
    player_name: String,
    sender: mpsc::Sender<Value>,
}

struct GameServer {
    players: Arc<Mutex<HashMap<String, Player>>>,
    spectators: Arc<Mutex<HashMap<String, mpsc::Sender<Value>>>>,
    current_game: Option<GameState>,
}










impl GameServer {
    fn new() -> Self {
        GameServer {
            players: Arc::new(Mutex::new(HashMap::new())),
            spectators: Arc::new(Mutex::new(HashMap::new())),
            current_game: None,
        }
    }

    async fn handle_player_registration(
        &self,
        player_sender: mpsc::Sender<Value>,
        player_name: String,
    ) -> String {
        let player_id = Self::generate_client_id();
        let player = Player {
            player_name: player_name.clone(),
            sender: player_sender.clone(),
        };
        let mut players = self.players.lock().await;
        players.insert(player_id.clone(), player);
        if let Err(e) = player_sender
            .send(serde_json::json!({
                "type": "registration_success",
                "client_id": player_id.clone()
            }))
            .await
        {
            eprintln!("Failed to send player id: {}", e);
        }

        // let players = players.lock().await;
        let players_infos = players
            .iter()
            .map(|(id, player)| serde_json::json!({
                "client_id": id,
                "player_name": player.player_name
            }))
            .collect::<Vec<_>>();
        let players_list = serde_json::json!({
            "type": "players_list",
            "players": players_infos
        });
        Self::broadcast_to_spectators(&self.spectators, players_list).await;
        player_id
    }

    async fn handle_spectator_registration(
        &self,
        spectator_sender: mpsc::Sender<Value>,
    ) -> String {
        let spectator_id = Self::generate_client_id();
        let mut spectators = self.spectators.lock().await;
        spectators.insert(spectator_id.clone(), spectator_sender.clone());

        let players = self.players.lock().await;
        let players_infos = players
            .iter()
            .map(|(id, player)| serde_json::json!({
                "client_id": id,
                "player_name": player.player_name
            }))
            .collect::<Vec<_>>();
        let players_list = serde_json::json!({
            "type": "players_list",
            "players": players_infos
        });
        for (_, spectator_sender) in spectators.iter() {
            if let Err(e) = spectator_sender.send(players_list.clone()).await {
                eprintln!("Failed to send message to spectator: {}", e);
            }
        }
        spectator_id
    }

    async fn handle_disconnect(&self, client_id: &str) {
        let mut players = self.players.lock().await;
        if players.remove(client_id).is_some() {
            let players_infos = players
                .iter()
                .map(|(id, player)| serde_json::json!({
                    "client_id": id,
                    "player_name": player.player_name
                }))
                .collect::<Vec<_>>();
            let players_list = serde_json::json!({
                "type": "players_list",
                "players": players_infos
            });
            Self::broadcast_to_spectators(&self.spectators, players_list).await;
        }
        let mut spectators = self.spectators.lock().await;
        if spectators.remove(client_id).is_some() {
            println!("Spectator {} removed", client_id);
        }

    }

    async fn broadcast_to_spectators(
        spectators: &Arc<Mutex<HashMap<String, mpsc::Sender<Value>>>>,
        message: Value,
    ) {
        let spectators = spectators.lock().await;
        for (tmp, spectator_sender) in spectators.iter() {
            println!("broads {}", tmp);
            if let Err(e) = spectator_sender.send(message.clone()).await {
                eprintln!("Failed to send message to spectator: {}", e);
            }
        }
    }

    fn generate_client_id() -> String {
        Uuid::new_v4().to_string()
    }

}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await.expect("Can't bind to address");
    println!("Server listening on: {}", addr);

    let game_server = Arc::new(GameServer::new());

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.expect("Failed to accept");
        let (write, read) = ws_stream.split();
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);

        let game_server = game_server.clone();
        let read = Arc::new(Mutex::new(read));
        tokio::spawn(async move {
            let mut write = write;
            let client_id = Arc::new(Mutex::new(None));

            let receive_task = {
                let client_id = Arc::clone(&client_id);
                let game_server = game_server.clone();
                let read = Arc::clone(&read);
                tokio::spawn(async move {
                    while let Some(msg) = {
                        let mut read_lock = read.lock().await;
                        read_lock.next().await
                    } {
                        if let Ok(msg) = msg {
                            let text = msg.to_text().unwrap();
                            match serde_json::from_str::<ClientMessage>(&text) {
                                Ok(mut client_message) => {
                                    match client_message.message_type.as_str() {
                                        "register" => {
                                            let mut id = String::new();
                                            match client_message.client_type {
                                                ClientType::Player => {
                                                    id = game_server.handle_player_registration(tx.clone(), client_message.client_name).await;
                                                },
                                                ClientType::Spectator => {
                                                    id = game_server.handle_spectator_registration(tx.clone()).await;
                                                }
                                            }
                                            client_message.client_id = Some(id.clone());
                                            *client_id.lock().await = Some(id);
                                        },
                                        "start_game" => {
                                            if let ClientType::Spectator = client_message.client_type {
                                                if let Some(game_config) = client_message.data.as_object() {
                                                    println!("Starting new game with config: {:?}", game_config);
                                                }
                                            }
                                        },
                                        "game_move" => {
                                            if let ClientType::Player = client_message.client_type {
                                                println!("GAME MOVE");
                                            }
                                        },
                                        _ => println!("Unknown message type"),
                                    }
                                },
                                Err(e) => {
                                    eprintln!("Error deserializing message: {}", e);
                                    // Vérifier si le client s'est déconnecté
                                    if e.to_string().contains("EOF") {
                                        println!("Client {} has disconnected", client_id.lock().await.as_ref().unwrap());
                                        game_server.handle_disconnect(client_id.lock().await.as_ref().unwrap()).await;
                                        break;

                                    }
                                },
                            }
                        }
                    }
                })
            };

            // Gestion des messages sortants
            let send_task = tokio::spawn(async move {
                println!("Send");
                while let Some(msg) = rx.recv().await {
                    let msg_str = serde_json::to_string(&msg).unwrap();
                    if let Err(e) = write.send(tokio_tungstenite::tungstenite::Message::Text(msg_str)).await {
                        eprintln!("Failed to send message: {}", e);
                        break;
                    }
                }
            });

            // Attendre que l'une des tâches se termine
            tokio::select! {
                _ = receive_task => println!("Receive task completed"),
                _ = send_task => println!("Send task completed"),
            }
        });
    }
}
