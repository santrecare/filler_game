<?php
require_once 'vendor/autoload.php'; // Ajoutez cette ligne pour autoloader les classes via Composer

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
        // Établir une connexion WebSocket et enregistrer le client
        $attempt = 0;
        while ($attempt < self::MAX_RECONNECT_ATTEMPTS) {
            try {
                $this->ws = new WebSocketClient($this->url); // Remplace WebSocketClient par la classe de ton client WebSocket
                echo "Connecté au serveur\n";
                $this->register();
                return;
            } catch (Exception $e) {
                echo "Échec de connexion au serveur\n";
                $attempt++;
                usleep(self::RECONNECT_DELAY * $attempt * 1000000);
            }
        }
        throw new Exception("Nombre maximal de tentatives de reconnexion atteint. Fin du programme.\nImpossible de se connecter au serveur.");
    }

    public function register() {
        // Envoie les données d'inscription au serveur et gère la réponse
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
            echo "Inscription réussie. ID: {$this->clientId}\n";
        }
    }

    public function makeMove($gameInfos, $currentPlayer) {
        // Évalue et envoie un mouvement basé sur les infos de jeu actuelles
        $move = null;
        if (count($gameInfos)) {
            $move = eval_move($gameInfos);
        }
        $this->sendMessage([
            'message_type' => 'game_move',
            'client_type' => 'Player',
            'client_name' => $this->playerName,
            'client_id' => $this->clientId,
            'data' => ['move' => $move, 'curr_player' => $currentPlayer]
        ]);
        echo "Mouvement joué: $move\n";
    }

    public function gameLoop() {
        // Boucle de réception continue et gestion des mises à jour de l'état de jeu
        echo "Démarrage de la boucle de jeu\n";
        while (true) {
            try {
                while (true) {
                    $message = $this->receiveMessage();
                    if ($message && isset($message['type']) && $message['type'] === 'game_state') {
                        echo json_encode($message) . "\n";
                        $this->makeMove($message['data'], $message['current_player']);
                    }
                    usleep(self::RECONNECT_DELAY * 1000000);
                }
            } catch (Exception $e) {
                echo $e;
                echo "Connexion au serveur perdue. Tentative de reconnexion...\n";
                $this->retryConnection();
            }
        }
    }

    private function receiveMessage() {
        // Reçoit et analyse un message JSON du serveur
        $message = $this->ws->receive();
        return json_decode($message, true);
    }

    private function sendMessage($message) {
        // Envoie un message JSON au serveur
        $this->ws->send(json_encode($message));
    }

    private function retryConnection() {
        // Réessaye la connexion en cas de déconnexion WebSocket
        usleep(self::RECONNECT_DELAY * 1000000);
        $this->connect();
    }

    public function close() {
        // Ferme la connexion WebSocket si elle est ouverte
        if ($this->ws) {
            $this->ws->close();
            echo "Connexion fermée\n";
        }
    }
}
?>
