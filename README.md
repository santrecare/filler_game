# Filler game

Inspired by a 42 subject https://github.com/fpetras/42-subjects/blob/master/filler.en.pdf

### Goal

The aim is to take turns placing a game-supplied piece of random shape and size on a board. To win, you must place more pieces than your opponent.

### Start

To start the project
```
git clone https://github.com/vklaouse/filler_game.git
cd filler_game/
docker network create arena-network
docker-compose up -d
```
Then go on `localhost:3000`

When you hover over the top of the page, there is a box that allows you to start a game and define the players who will play it.

### Players

You can choose from two languages to write your player

- PHP

    `cd php_player_template/`

- Python

    `cd python_player_template/`

Instructions to use them are in their README.md
