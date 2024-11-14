use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;
use serde_json::Value;

use crate::game::GameState;


#[derive(Debug, Clone)]
pub struct PlayerClient {
    pub player_name: String,
    sender: mpsc::Sender<Value>,
}

#[derive(Debug, Clone)]
pub struct GameServer {
    pub players_clients: Arc<Mutex<HashMap<String, PlayerClient>>>,
    pub spectators_clients: Arc<Mutex<HashMap<String, mpsc::Sender<Value>>>>,
    pub current_game: Arc<RwLock<Option<GameState>>>,
}


impl GameServer {
    pub fn new() -> Self {
        GameServer {
            players_clients: Arc::new(Mutex::new(HashMap::new())),
            spectators_clients: Arc::new(Mutex::new(HashMap::new())),
            current_game: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn handle_player_registration(
        &self,
        player_sender: mpsc::Sender<Value>,
        player_name: String,
    ) -> String {
        let player_id = Self::generate_client_id();
        let player = PlayerClient {
            player_name: player_name.clone(),
            sender: player_sender.clone(),
        };
        let mut players = self.players_clients.lock().await;
        players.insert(player_id.clone(), player);
        if let Err(e) = player_sender.send(serde_json::json!({
                "type": "registration_success",
                "client_id": player_id.clone()
            })).await {
            eprintln!("Failed to send player id: {}", e);
        }

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
        Self::broadcast_to_spectators(&self.spectators_clients, players_list).await;
        player_id
    }

    pub async fn handle_spectator_registration(
        &self,
        spectator_sender: mpsc::Sender<Value>,
    ) -> String {
        let spectator_id = Self::generate_client_id();
        let mut spectators = self.spectators_clients.lock().await;
        spectators.insert(spectator_id.clone(), spectator_sender.clone());

        let players = self.players_clients.lock().await;
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

    pub async fn handle_disconnect(&self, client_id: &str) {
        let mut players = self.players_clients.lock().await;
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
            Self::broadcast_to_spectators(&self.spectators_clients, players_list).await;
        }
        let mut spectators = self.spectators_clients.lock().await;
        if spectators.remove(client_id).is_some() {
            println!("Spectator {} removed", client_id);
        }

    }

    pub fn set_current_game(&self, game_state: GameState) {
        let mut current_game = self.current_game.write().expect("Lock was poisoned");
        *current_game = Some(game_state);
    }

    pub fn get_current_game(&self) -> GameState {
        let mut current_game = self.current_game.read().expect("Lock was poisoned");
        current_game.as_ref().expect("No game in progress").clone()
    }

    pub async fn broadcast_to_spectators(
        spectators: &Arc<Mutex<HashMap<String, mpsc::Sender<Value>>>>,
        message: Value,
    ) {
        let spectators = spectators.lock().await;
        for (_, spectator_sender) in spectators.iter() {
            if let Err(e) = spectator_sender.send(message.clone()).await {
                eprintln!("Failed to send message to spectator: {}", e);
            }
        }
    }

    pub async fn broadcast_to_player(
        players: &Arc<Mutex<HashMap<String, PlayerClient>>>,
        player_id: String,
        message: Value,
    ) {
        let players = players.lock().await;

        if let Err(e) = players[&player_id].sender.send(message.clone()).await {
            eprintln!("Failed to send message to spectator: {}", e);
        }
    }

    fn generate_client_id() -> String {
        Uuid::new_v4().to_string()
    }
}
