const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
let arenaState;

const cellColors = {
    null: '#2a2a2a',
    '0': '#2a2a2a',
    '*': '#f2f2f2',
    '1': '#8e008e',
    '2': '#8e8e00',
    '-1': '#f2008d',
    '-2': '#8ef200',
}

window.addEventListener('resize', updateArena);
class SpectatorClient {
    constructor() {
        this.ws = null;
        this.players = [];
        this.currentGame = null;
        this.selectedPlayer1 = null;
        this.selectedPlayer2 = null;

        document.getElementById('player1-dropdown').addEventListener('change', (e) => {
            this.selectPlayer(1, e.target.value);
        });
        document.getElementById('player2-dropdown').addEventListener('change', (e) => {
            this.selectPlayer(2, e.target.value);
        });
    }

    connect() {
        this.ws = new WebSocket('ws://0.0.0.0:8080');

        this.ws.onopen = () => {
            console.log('Connected to server');
            this.register();
        };

        this.ws.onmessage = (event) => {
            const message = JSON.parse(event.data);
            this.handleMessage(message);
        };

        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
        };

        this.ws.onclose = () => {
            console.log('Disconnected from server');
            setTimeout(() => this.connect(), 5000);
        };
    }

    register() {
        const registerMessage = {
            message_type: 'register',
            client_type: 'Spectator',
            client_name: 'Spectator',
            client_id: null,
            data: {}
        };
        this.ws.send(JSON.stringify(registerMessage));
    }

    handleMessage(message) {
        switch(message.type) {
            case 'players_list':
                this.players = message.players.map(player => ({
                    client_id: player.client_id,
                    player_name: player.player_name
                }));

                this.updateDropdowns();
                break;
            case 'game_state':
                arenaState = message.data
                this.updateGameState(message.game_state);
                break;
        }
    }

    updateDropdowns() {
        const player1Dropdown = document.getElementById('player1-dropdown');
        const player2Dropdown = document.getElementById('player2-dropdown');

        while (player1Dropdown.options.length > 1) {
            player1Dropdown.remove(1);
        }
        while (player2Dropdown.options.length > 1) {
            player2Dropdown.remove(1);
        }

        this.players.forEach(player => {
            const option1 = new Option(`${player.player_name}`, player.client_id);
            player1Dropdown.add(option1);

            const option2 = new Option(`${player.player_name}`, player.client_id);
            player2Dropdown.add(option2);
        });

        if (!this.players.includes(this.selectedPlayer1)) {
            this.selectedPlayer1 = null;
            player1Dropdown.value = "";
        }
        if (!this.players.includes(this.selectedPlayer2)) {
            this.selectedPlayer2 = null;
            player2Dropdown.value = "";
        }

        this.updateStartButton();
    }

    selectPlayer(playerNumber, playerId) {
        if (playerNumber === 1) {
            this.selectedPlayer1 = playerId || null;
        } else {
            this.selectedPlayer2 = playerId || null;
        }
        this.updateStartButton();
    }

    updateStartButton() {
        const startButton = document.getElementById('start-game-btn');
        startButton.disabled = !(this.selectedPlayer1 && this.selectedPlayer2);
        startButton.onclick = () => {
            if (this.selectedPlayer1 && this.selectedPlayer2) {
                this.startGame(this.selectedPlayer1, this.selectedPlayer2);
            }
        };
    }

    updateGameState(gameState) {
        this.currentGame = gameState;
        this.renderGame();
    }

    startGame(player1Id, player2Id) {
        const startGameMessage = {
            message_type: 'start_game',
            client_type: 'Spectator',
            client_name: 'Spectator',
            client_id: null,
            data: {
                player1: player1Id,
                player2: player2Id
            }
        };
        this.ws.send(JSON.stringify(startGameMessage));
    }

    renderGame() {
        updateArena()
    }
}

function drawCell(x, y, width, height, color) {
    ctx.fillStyle = color;
    ctx.fillRect(x, y, width, height);
}

function calcCellSize(height, cellCount) {
    return Math.floor(height / cellCount);
}

function drawText(text, x, y, fontSize) {
    ctx.fillStyle = '#fff';
    ctx.font = fontSize + 'px trebuchet ms';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    ctx.fillText(text, x, y);
}

function drawBackground() {
    ctx.fillStyle = '#151515';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

function drawGrid(grid, width, height, cellSize, marginLeft=0, marginTop=0) {
    for (let y = 0; y < grid.length; y++) {
        for (let x = 0; x < grid[y].length; x++) {
            let cell = null;
            if (grid[y][x] != null)
                cell = grid[y][x].toString();
            drawCell(
                x * cellSize + marginLeft,
                y * cellSize + marginTop,
                cellSize - 1,
                cellSize - 1,
                cellColors[cell],
            );
        }
    }
}

function updateArena() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    const playerInfoSize = Math.floor(canvas.height * 0.1);
    drawBackground();
    if (!arenaState)
        return;
    const arenaCellSize = Math.floor(
        (canvas.height - playerInfoSize) / arenaState['board']['size']);
    drawGrid(
        grid=arenaState['board']['board'],
        width=canvas.width,
        height=canvas.height - playerInfoSize,
        cellSize=arenaCellSize,
        marginLeft=(canvas.width - arenaCellSize * arenaState['board']['size']) / 2);

    const pieceCellSize = Math.floor(
        (canvas.height - arenaCellSize * arenaState['board']['size']) / arenaState['piece']['size']);
    drawGrid(
        grid=arenaState['piece']['piece'],
        width=canvas.width,
        height=canvas.height,
        cellSize=pieceCellSize,
        marginLeft=(canvas.width - pieceCellSize * arenaState['piece']['size']) / 2,
        marginTop=(height - pieceCellSize * arenaState['piece']['size']));

    // drawText(
    //     text=arenaState['player1']['name'] + ' - ' + arenaState['player1']['score'],
    //     x=canvas.width / 5,
    //     y=canvas.height - playerInfoSize,
    //     fontSize=playerInfoSize * 0.7);
    // drawText(
    //     text=arenaState['player2']['score'] + ' - ' + arenaState['player2']['name'],
    //     x=(canvas.width / 5) * 4,
    //     y=canvas.height - playerInfoSize,
    //     fontSize=playerInfoSize * 0.7);
}

const spectator = new SpectatorClient();
updateArena()
spectator.connect();
