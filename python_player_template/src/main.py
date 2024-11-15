import argparse
from os import environ
from .game_client.game_client import GameClient


def parse_arguments():
    """Parse and return command-line arguments."""
    parser = argparse.ArgumentParser(description='Python player client')
    parser.add_argument(
        '--player_name',
        type=str,
        default=environ.get('PLAYER_NAME', 'PythonPlayer'),
    )
    parser.add_argument(
        '--host',
        type=str,
        default=environ.get('ARENA_HOST', 'arena'),
    )
    parser.add_argument(
        '--port',
        type=int,
        default=environ.get('ARENA_PORT', 8080),
    )
    return parser.parse_args()


def run_client(player_name, host, port):
    """Run the game client with the provided parameters."""
    client = GameClient(player_name=player_name, host=host, port=port)
    try:
        print('Attempting to connect...')
        client.connect()
        client.game_loop()
    except KeyboardInterrupt:
        print('Client shutdown...')
    finally:
        client.close()


def main():
    args = parse_arguments()
    run_client(args.player_name, args.host, args.port)


if __name__ == '__main__':
    main()
