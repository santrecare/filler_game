<?php

require_once 'gameClient.php';

function parseArguments() {
    $playerName = getenv('PLAYER_NAME') ?: 'PHPPlayer';
    $host = getenv('ARENA_HOST') ?: 'arena';
    $port = getenv('ARENA_PORT') ?: 8080;
    return [$playerName, $host, $port];
}

function runClient($playerName, $host, $port) {
    $client = new GameClient($playerName, $host, $port);
    try {
        echo "Tentative de connexion...\n";
        $client->connect();
        $client->gameLoop();
    } catch (Exception $e) {
        echo "Fermeture du client...\n";
    } finally {
        $client->close();
    }
}

list($playerName, $host, $port) = parseArguments();
runClient($playerName, $host, $port);
?>
