use std::sync::Arc;
use tokio::net::TcpListener;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::accept_async;
use tokio::sync::Mutex;
use rand::{Rng, thread_rng};

use super::message::{ClientMessage, ClientType};
use super::game_server::GameServer;
use crate::game::{set_game_state, play, Piece};


pub async fn run_server() {
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
                                    // let mut player_turn = 1;
                                    match client_message.message_type.as_str() {
                                        "register" => {
                                            match client_message.client_type {
                                                ClientType::Player => {
                                                    let id = game_server.handle_player_registration(tx.clone(), client_message.client_name).await;
                                                    client_message.client_id = Some(id.clone());
                                                    *client_id.lock().await = Some(id);
                                                },
                                                ClientType::Spectator => {
                                                    let id = game_server.handle_spectator_registration(tx.clone()).await;
                                                    client_message.client_id = Some(id.clone());
                                                    *client_id.lock().await = Some(id);
                                                }
                                            }
                                        },
                                        "start_game" => {
                                            if let ClientType::Spectator = client_message.client_type {
                                                if let Some(game_config) = client_message.data.as_object() {
                                                    let player1 = game_config.get("player1")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("Player1 Unknown");
                                                    let player2 = game_config.get("player2")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("Player2 Unknown");
                                                    let mut game_state = set_game_state(
                                                        player1.to_string().clone(),
                                                        player2.to_string().clone(),
                                                        10,
                                                        5
                                                    );
                                                    GameServer::broadcast_to_spectators(
                                                        &game_server.spectators_clients,
                                                        serde_json::json!({
                                                            "type": "game_state",
                                                            "data": {
                                                                "board": game_state.board,
                                                                "piece": game_state.piece,
                                                                "player_one": game_state.player_one,
                                                                "player_two": game_state.player_two,
                                                                "current_player": game_state.current_player,
                                                            }
                                                        })
                                                    ).await;
                                                    play(&mut game_state);
                                                    game_state.board.display();
                                                    game_state.piece.display();
                                                    let val_to_send = serde_json::json!({
                                                        "type": "game_state",
                                                        "current_player": game_state.current_player,
                                                        "data": game_state.data.clone()
                                                    });
                                                    println!("{}, {}, {}, json {:?} , data {:?}", game_state.player_one, game_state.player_two, game_state.current_player, val_to_send, game_state.data);
                                                    // player_turn = 1;
                                                    GameServer::broadcast_to_player(
                                                        &game_server.players_clients,
                                                        player1.to_string().clone(),
                                                        val_to_send,
                                                    ).await;
                                                    game_server.set_current_game(game_state);
                                                }
                                            }
                                        },
                                        "game_move" => {
                                            if let ClientType::Player = client_message.client_type {
                                                if let Some(game_move) = client_message.data.as_object() {
                                                    let mut game_state = game_server.get_current_game();
                                                    let curr_player = game_move.get("curr_player")
                                                        .and_then(|v| v.as_i64())
                                                        .unwrap_or(0);
                                                    // println!("yolo {}, {}, {}", ((curr_player as i64) != (player_turn as i64)), curr_player, player_turn);

                                                    // if curr_player == player_turn {
                                                    //     return ();
                                                    // }
                                                    // if player_turn == 1 {
                                                    //     player_turn = 2;
                                                    // }
                                                    // else {
                                                    //     player_turn = 1;
                                                    // }
                                                    if let Some(move_value) = game_move.get("move") {
                                                        println!("******************");

                                                        let y = move_value.get(0)
                                                            .and_then(|v| v.as_i64())
                                                            .unwrap_or(0) as isize;
                                                        let x = move_value.get(1)
                                                            .and_then(|v| v.as_i64())
                                                            .unwrap_or(0) as isize;
                                                        println!("test move {}, {}", y, x);
                                                        if game_state.board.is_valid_piece_coord(&game_state.piece, y, x, game_state.current_player.abs()) {
                                                            println!("Good piece {}", game_state.current_player);
                                                            game_state.board.set_piece(
                                                                &game_state.piece, y, x, -1 * game_state.current_player
                                                            );
                                                            GameServer::broadcast_to_spectators(
                                                                &game_server.spectators_clients,
                                                                serde_json::json!({
                                                                    "type": "game_state",
                                                                    "data": {
                                                                        "board": game_state.board,
                                                                        "piece": game_state.piece,
                                                                        "player_one": game_state.player_one,
                                                                        "player_two": game_state.player_two,
                                                                        "current_player": game_state.current_player,
                                                                    }
                                                                })
                                                            ).await;
                                                            game_state.board.set_piece(
                                                                &game_state.piece, y as isize, x as isize, game_state.current_player
                                                            );
                                                        }
                                                        else {
                                                            println!("Bad piece {}", game_state.current_player);
                                                            GameServer::broadcast_to_spectators(
                                                                &game_server.spectators_clients,
                                                                serde_json::json!({
                                                                    "type": "game_state",
                                                                    "data": {
                                                                        "board": game_state.board,
                                                                        "piece": game_state.piece,
                                                                        "player_one": game_state.player_one,
                                                                        "player_two": game_state.player_two,
                                                                        "current_player": game_state.current_player,
                                                                    }
                                                                })
                                                            ).await;
                                                        }
                                                    }
                                                    let mut current_player = String::new();
                                                    if game_state.current_player == 1 {
                                                        current_player = game_state.player_two.clone();
                                                        game_state.current_player = 2
                                                    }
                                                    else {
                                                        current_player = game_state.player_one.clone();
                                                        game_state.current_player = 1
                                                    }
                                                    println!("{}", current_player);
                                                    println!("-------------------");
                                                    let piece_size = game_state.piece.size;
                                                    let mut piece = Piece::new(piece_size, game_state.piece.density);
                                                    piece.generate_piece(
                                                        thread_rng().gen_range(0..piece_size) as usize,
                                                        thread_rng().gen_range(0..piece_size) as usize,
                                                        0
                                                    );
                                                    game_state.piece = piece.clone();
                                                    play(&mut game_state);
                                                    GameServer::broadcast_to_player(
                                                        &game_server.players_clients,
                                                        current_player,
                                                        serde_json::json!({
                                                            "type": "game_state",
                                                            "current_player": game_state.current_player,
                                                            "data": game_state.data
                                                        })
                                                    ).await;
                                                    game_server.set_current_game(game_state.clone());
                                                }
                                            }
                                        },
                                        _ => println!("Unknown message type"),
                                    }
                                },
                                Err(e) => {
                                    eprintln!("Error deserializing message: {}", e);
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

            let send_task = tokio::spawn(async move {
                // println!("Send");
                while let Some(msg) = rx.recv().await {
                    let msg_str = serde_json::to_string(&msg).unwrap();
                    // println!("{}", msg_str);
                    if let Err(e) = write.send(tokio_tungstenite::tungstenite::Message::Text(msg_str)).await {
                        eprintln!("Failed to send message: {}", e);
                        break;
                    }
                }
            });

            tokio::select! {
                _ = receive_task => println!("Receive task completed"),
                _ = send_task => println!("Send task completed"),
            }
        });
    }
}
