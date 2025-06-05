import init, { Board } from "./wasm/pkg/wasm.js";

/** @type {HTMLDivElement} */ // @ts-ignore
const game = document.getElementById("game");

/** @type {HTMLDivElement[][]} */
const boardCellContents = [];

const lastBoardContents = [];

/** @type {((x: number, y: number) => void)[]} */
const clickListeners = [];

initBoard();

init().then(() => {
    const board = Board.new();
    renderBoard(board);

    let currTurn = false;

    clickListeners.push((x, y) => {
        if (board.try_place_chip(x, y, currTurn)) {
            currTurn = !currTurn;
        }

        renderBoard(board);
    });
});

function initBoard() {
    while (game.lastChild) {
        game.removeChild(game.lastChild);
    }

    const table = document.createElement("table");
    table.classList.add("gameTable");
    const tbody = document.createElement("tbody");

    for (let y = 0; y < 8; y++) {
        const boardRow = [];
        const tr = document.createElement("tr");

        for (let x = 0; x < 8; x++) {
            const cell = document.createElement("td");
            cell.classList.add("gameCell");
            cell.addEventListener("click", () => dispatchCellClicked(x, y));

            const content = document.createElement("div");
            content.classList.add("gameCellContent", "blank");
            cell.appendChild(content);

            tr.appendChild(cell);
            boardRow.push(content);
        }

        tbody.appendChild(tr);
        boardCellContents.push(boardRow);
        lastBoardContents.push([0, 0, 0, 0, 0, 0, 0, 0]);
    }

    table.appendChild(tbody);
    game.appendChild(table);
}

/**
 * @param {Board} board 
 */
function renderBoard(board) {
    const jsRep = convertBoardToJsRep(board);

    for (let y = 0; y < 8; y++) {
        for (let x = 0; x < 8; x++) {
            if (jsRep[y][x] !== lastBoardContents[y][x]) {
                const elm = boardCellContents[y][x];
                switch (jsRep[y][x]) {
                    case 0n:
                        elm.classList.remove("black", "white");
                        elm.classList.add("blank");
                        elm.innerText = "";
                        break;
                    case 1n:
                        elm.classList.remove("blank", "black");
                        elm.classList.add("white");
                        elm.innerText = "W";
                        break;
                    case -1n:
                        elm.classList.remove("blank", "white");
                        elm.classList.add("black");
                        elm.innerText = "B";
                        break;
                }
            }
        }
    }
}

/**
 * @param {number} x 
 * @param {number} y 
 */
function dispatchCellClicked(x, y) {
    for (const listener of clickListeners) {
        listener(x, y);
    }
}

/** @param {Board} board */
function convertBoardToJsRep(board) {
    let filled = board.filled;
    let color = board.color;

    const arr = [];
    for (let i = 0; i < 8; i++) {
        arr.push([
            (filled & 1n) * (((color & 1n) << 1n) - 1n),
            (filled >> 1n & 1n) * (((color >> 1n & 1n) << 1n) - 1n),
            (filled >> 2n & 1n) * (((color >> 2n & 1n) << 1n) - 1n),
            (filled >> 3n & 1n) * (((color >> 3n & 1n) << 1n) - 1n),
            (filled >> 4n & 1n) * (((color >> 4n & 1n) << 1n) - 1n),
            (filled >> 5n & 1n) * (((color >> 5n & 1n) << 1n) - 1n),
            (filled >> 6n & 1n) * (((color >> 6n & 1n) << 1n) - 1n),
            (filled >> 7n & 1n) * (((color >> 7n & 1n) << 1n) - 1n),
        ]);
        filled >>= 8n;
        color >>= 8n;
    }

    return arr;
}