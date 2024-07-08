const socket = new WebSocket('ws://localhost:8765');
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
let arenaState;

const cellColors = {
    null: '#2a2a2a',
    '*': '#f2f2f2',
    '1': '#8e008e',
    '2': '#8e8e00',
    '-1': '#f2008d',
    '-2': '#8ef200',
}

window.addEventListener('resize', updateArena);

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
    // marginLeft = mLeft ? (width - cellSize * grid.length) / 2 : 0
    // marginTop = mTop ? (height - cellSize * grid.length) : 0
    for (let y = 0; y < grid.length; y++) {
        for (let x = 0; x < grid[y].length; x++) {
            drawCell(
                x * cellSize + marginLeft,
                y * cellSize + marginTop,
                cellSize - 1,
                cellSize - 1,
                cellColors[grid[y][x]],
            );
        }
    }
}

function updateArena() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    const playerInfoSize = Math.floor(canvas.height * 0.1);
    drawBackground();

    const arenaCellSize = Math.floor(
        (canvas.height - playerInfoSize) / arenaState['board'].length);
    drawGrid(
        grid=arenaState['board'],
        width=canvas.width,
        height=canvas.height - playerInfoSize,
        cellSize=arenaCellSize,
        marginLeft=(canvas.width - arenaCellSize * arenaState['board'].length) / 2);

    const pieceCellSize = Math.floor(
        (canvas.height - arenaCellSize * arenaState['board'].length) / arenaState['piece'].length);
    drawGrid(
        grid=arenaState['piece'],
        width=canvas.width,
        height=canvas.height,
        cellSize=pieceCellSize,
        marginLeft=(canvas.width - pieceCellSize * arenaState['piece'].length) / 2,
        marginTop=(height - pieceCellSize * arenaState['piece'].length));

    drawText(
        text=arenaState['player1']['name'] + ' - ' + arenaState['player1']['score'],
        x=canvas.width / 5,
        y=canvas.height - playerInfoSize,
        fontSize=playerInfoSize * 0.7);
    drawText(
        text=arenaState['player2']['score'] + ' - ' + arenaState['player2']['name'],
        x=(canvas.width / 5) * 4,
        y=canvas.height - playerInfoSize,
        fontSize=playerInfoSize * 0.7);
}

socket.onmessage = function(event) {
    arenaState = JSON.parse(event.data);
    updateArena();
};

updateArena()
