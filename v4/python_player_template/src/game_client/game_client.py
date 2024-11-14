import time
import argparse
from os import environ
from websocket import create_connection, WebSocketConnectionClosedException
from json import dumps, loads
from ..eval import eval

class GameClient:
    RECONNECT_DELAY = 0.1
    MAX_RECONNECT_ATTEMPTS = 5

    def __init__(self, player_name, host, port):
        self.client_id = None
        self.player_name = player_name
        self.url = f'ws://{host}:{port}'
        self.ws = None

    def connect(self):
        """Establish a WebSocket connection and register the client."""
        attempt = 0
        while attempt < self.MAX_RECONNECT_ATTEMPTS:
            try:
                self.ws = create_connection(self.url)
                print('Connected to server')
                self.register()
                return
            except WebSocketConnectionClosedException:
                print('Failed to connect to server')
                attempt += 1
                time.sleep(self.RECONNECT_DELAY * attempt)
        raise WebSocketConnectionClosedException(
            "Max reconnection attempts reached. Exiting...\n"
            "Unable to connect to server")

    def register(self):
        """Send registration data to the server and handle the response."""
        self.send_message({
            'message_type': 'register',
            'client_type': 'Player',
            'client_name': self.player_name,
            'client_id': self.client_id,
            'data': {}
        })
        response = self.receive_message()
        if response.get('type') == 'registration_success':
            self.client_id = response['client_id']
            print(f'Registered successfully. ID: {self.client_id}')

    def make_move(self, game_infos, current_player, game_id):
        """Evaluate and send a move based on the current game infos."""
        move = None
        if len(game_infos):
            move = eval(game_infos)
        self.send_message({
            'message_type': 'game_move',
            'client_type': 'Player',
            'client_name': self.player_name,
            'client_id': self.client_id,
            'data': {'move': move, 'curr_player': current_player, 'game_id': game_id}
        })
        print(f'Move played: {move}')

    def game_loop(self):
        """Continuously receive and handle game state updates from the server,
        with automatic reconnection and restart on connection loss.
        """
        print('Starting game loop')
        while True:
            try:
                while True:
                    message = self.receive_message()
                    if message and message.get('type') and message['type'] == 'game_state':
                        # print(message)
                        self.make_move(message['data'], message['current_player'], message['game_id'])
                    time.sleep(self.RECONNECT_DELAY)
            except WebSocketConnectionClosedException:
                print('Connection to server lost. Attempting to reconnect...')
                self.retry_connection()
            except KeyboardInterrupt:
                print('Client shutdown...')
                break
        self.close()

    def receive_message(self):
        """Receive and parse a JSON message from the server."""
        return loads(self.ws.recv())

    def send_message(self, message):
        """Send JSON message to the server."""
        self.ws.send(dumps(message))

    def retry_connection(self):
        """Retry the connection in case of a WebSocket disconnection."""
        time.sleep(self.RECONNECT_DELAY)
        self.connect()

    def close(self):
        """Close the WebSocket connection if it is open."""
        if self.ws:
            self.ws.close()
            print('Connection closed')
