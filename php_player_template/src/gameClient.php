<?php
require_once 'vendor/autoload.php';

use WebSocket\Client as WebSocketClient;
require_once 'eval.php';


class GameClient {
    private $clientId;
    private $playerName;
    private $url;
    private $ws;
    const RECONNECT_DELAY = 0.1;
    const MAX_RECONNECT_ATTEMPTS = 5;

    public function __construct($playerName, $host, $port) {
        $this->clientId = null;
        $this->playerName = $playerName;
        $this->url = "ws://$host:$port";
        $this->ws = null;
    }

    public function connect() {
        $attempt = 0;
        while ($attempt < self::MAX_RECONNECT_ATTEMPTS) {
            try {
                $this->ws = new WebSocketClient(
                    $this->url,
                    ['timeout' => 3600]
                );
                echo "Connected to server\n";
                $this->register();
                return;
            } catch (Exception $e) {
                echo "Failed to connect to server\n";
                $attempt++;
                usleep(self::RECONNECT_DELAY * $attempt * 1000000);
            }
        }
        throw new Exception("Max reconnection attempts reached. Exiting...\nUnable to connect to server");
    }

    public function register() {
        $this->sendMessage([
            'message_type' => 'register',
            'client_type' => 'Player',
            'client_name' => $this->playerName,
            'client_id' => $this->clientId,
            'data' => new stdClass()
        ]);

        $response = $this->receiveMessage();
        if (isset($response['type']) && $response['type'] === 'registration_success') {
            $this->clientId = $response['client_id'];
            echo "Registered successfully. ID: {$this->clientId}\n";
        }
    }

    public function makeMove($gameInfos, $currentPlayer, $gameId, $piece, $board) {
        $move = null;
        if (count($gameInfos['playable_coordinates'])) {
            $move = eval_move($gameInfos, $board, $piece, $currentPlayer);
        }
        $this->sendMessage([
            'message_type' => 'game_move',
            'client_type' => 'Player',
            'client_name' => $this->playerName,
            'client_id' => $this->clientId,
            'data' => [
                'move' => $move,
                'curr_player' => $currentPlayer,
                'game_id' => $gameId,
            ]
        ]);
    }

    public function gameLoop() {
        echo "Starting game loop\n";
        while (true) {
            try {
                while (true) {
                    $message = $this->receiveMessage();
                    if ($message && isset($message['type']) && $message['type'] === 'game_state') {
                        $this->makeMove($message['data'], $message['current_player'], $message['game_id'], $message['piece'], $message['board']);
                    }
                    usleep(self::RECONNECT_DELAY * 1000000);
                }
            } catch (Exception $e) {
                echo "Connection to server lost. Attempting to reconnect...\n";
                $this->retryConnection();
            }
        }
    }

    private function receiveMessage() {
        $message = $this->ws->receive();
        return json_decode($message, true);
    }

    private function sendMessage($message) {
        $this->ws->send(json_encode($message));
    }

    private function retryConnection() {
        usleep(self::RECONNECT_DELAY * 1000000);
        $this->connect();
    }

    public function close() {
        if ($this->ws) {
            $this->ws->close();
            echo "Connection closed\n";
        }
    }
}
?>
