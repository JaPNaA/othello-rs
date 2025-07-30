import init, { JsInterface } from "./wasm/pkg/wasm.js";
import { Elm } from "./elements.js";

/** @type {HTMLDivElement} */ // @ts-ignore
const game = document.getElementById("game");
/** @type {Elm} */ // @ts-ignore
const history = new Elm(document.getElementById("history"));

/** @type {HTMLButtonElement} */ // @ts-ignore
const rerunButton = document.getElementById("rerunButton");
/** @type {HTMLDivElement} */ // @ts-ignore
const sideElm = document.getElementById("side");

/** @type {HTMLSelectElement} */ // @ts-ignore
const blackBotSelect = document.getElementById("blackBotSelect");
/** @type {HTMLSelectElement} */ // @ts-ignore
const whiteBotSelect = document.getElementById("whiteBotSelect");
/** @type {HTMLInputElement} */ // @ts-ignore
const numRoundsInput = document.getElementById("numRounds");

/** @type {Elm[][]} */
const boardCellContents = [];

const lastBoardContents = [];

/** @type {((x: number, y: number) => void)[]} */
const clickListeners = [];

const whiteScore = new Elm("span").class("score");
const blackScore = new Elm("span").class("score");
const gameOverDisplay = new Elm("div");
const gameTableContainer = new Elm("div").class("gameTableContainer");

/** @type {Map<string, (jsi: JsInterface) => void>} */
const bots = new Map([
    ["Human", (_) => { }],
    ["Deep Negative Heuristic Bot", (jsi) => jsi.create_deep_negative_heuristic_bot()],
    ["Top Left Bot", (jsi) => jsi.create_new_first_valid_move_bot()],
    ["Bottom Right Bot", (jsi) => jsi.create_new_last_valid_move_bot()],
    ["Random Bot", (jsi) => jsi.create_new_random_bot()],
    ["Center Bot", (jsi) => jsi.create_new_center_bot()],
    ["Edge Bot", (jsi) => jsi.create_new_edge_bot()],
    ["Edge Exclusive Bot", (jsi) => jsi.create_new_edge_exclusive_bot()],
    ["Shallow Score Bot", (jsi) => jsi.create_shallow_score_bot()],
    ["Minmax Score Bot", (jsi) => jsi.create_new_minmax_score_bot()],
    ["Deep Score Bot", (jsi) => jsi.create_deep_score_bot()],
    ["Deep Heuristic Bot", (jsi) => jsi.create_deep_heuristic_bot()],
]);

const gameState = {
    whiteIsHuman: true,
    blackIsHuman: true,
    gameActive: false,
    turn: false
};

function initAll() {
    initBotSelector();
    initBoard();
    initExplainOthelloBoards();

    init().then(() => {
        const jsInterface = JsInterface.new();
        runGame(jsInterface);

        rerunButton.addEventListener("click", () => {
            runGame(jsInterface);
        });

        clickListeners.push((x, y) => {
            humanInput(jsInterface, x, y);
            renderBoard(jsInterface);
        });
    });
}

function initBotSelector() {
    for (const bot of bots.keys()) {
        new Elm("option").append(bot).attribute("value", bot).appendTo(whiteBotSelect);
        new Elm("option").append(bot).attribute("value", bot).appendTo(blackBotSelect);
    }
}

function initBoard() {
    while (game.lastChild) {
        game.removeChild(game.lastChild);
    }

    const table = new Elm("table").class("gameTable").appendTo(gameTableContainer.appendTo(game));
    const tbody = new Elm("tbody").appendTo(table);

    for (let y = 0; y < 8; y++) {
        const boardRow = [];
        const tr = new Elm("tr").appendTo(tbody);

        for (let x = 0; x < 8; x++) {
            const cell = new Elm("td")
                .class("gameCell")
                .on("click", () => dispatchCellClicked(x, y))
                .appendTo(tr);

            const content = new Elm().class("gameCellContent", "blank").appendTo(cell);

            boardRow.push(content);
        }

        boardCellContents.push(boardRow);
        lastBoardContents.push([0, 0, 0, 0, 0, 0, 0, 0]);
    }

    new Elm().class("status").append(
        new Elm().append("Score -- black: ", blackScore, ", white: ", whiteScore),
        new Elm().append(gameOverDisplay)
    ).appendTo(game);

    // attach resize event handlers
    updateGameCellSize();
    addEventListener("resize", () => onResize());
}

/**
 * @param {JsInterface} jsi
 * @param {number} x
 * @param {number} y
 */
function humanInput(jsi, x, y) {
    if (!gameState.gameActive) { return; }
    if (!(
        gameState.turn === true && gameState.whiteIsHuman ||
        gameState.turn === false && gameState.blackIsHuman
    )) {
        return;
    }

    if (jsi.board_try_place(x, y, gameState.turn)) {
        addHistory(gameState.turn, x, y);
        gameState.turn = !gameState.turn;
        checkGameState(jsi);

        scheduleBotMoveIfShould(jsi);
    }
}

/** @param {JsInterface} jsi */
function runBotMove(jsi) {
    let moved = false;
    let move;
    if (gameState.turn === true && !gameState.whiteIsHuman) {
        move = jsi.bot_run_white();
        moved = true;
    } else if (gameState.turn === false && !gameState.blackIsHuman) {
        move = jsi.bot_run_black();
        moved = true;
    }

    if (!moved) { return; }

    if (move) {
        addHistory(gameState.turn, move[0], move[1]);
        renderBoard(jsi);
    } else {
        addHistorySkipped(gameState.turn);
    }
    gameState.turn = !gameState.turn;


    checkGameState(jsi);
    scheduleBotMoveIfShould(jsi);
}

/**
 * Checks the state of the game for skips or game ends.
 * @param {JsInterface} jsi
 */
function scheduleBotMoveIfShould(jsi) {
    if (!gameState.gameActive) { return; }
    if (
        gameState.turn === true && !gameState.whiteIsHuman ||
        gameState.turn === false && !gameState.blackIsHuman
    ) {
        setTimeout(() => {
            runBotMove(jsi);
        }, 300);
    }
}

/**
 * Checks the state of the game for skips or game ends.
 * @param {JsInterface} jsi
 */
function checkGameState(jsi) {
    if (!jsi.board_has_valid_move(gameState.turn)) {
        addHistorySkipped(gameState.turn);
        gameState.turn = !gameState.turn;

        if (!jsi.board_has_valid_move(gameState.turn)) {
            // game end
            gameState.gameActive = false;
            return;
        }
    }
}

const letters = "abcdefgh";

/**
 * @param {boolean} color 
 * @param {number} x 
 * @param {number} y 
 */
function addHistory(color, x, y) {
    _addHistory(color, `${letters[x]}${8 - y}`);
}

/** @param {boolean} color */
function addHistorySkipped(color) {
    _addHistory(color, `Skip`);
}

/**
 * @param {boolean} color 
 * @param {string} text 
 */
function _addHistory(color, text) {
    const elm = new Elm("li").class("item");
    if (color) { elm.class("white"); } else { elm.class("black"); }
    elm.append(`${color ? "W" : "B"}: `, text);
    elm.appendTo(history);
    history.elm.scrollTop = history.elm.scrollHeight;
}

/**
 * @param {JsInterface} jsInterface 
 */
function runGame(jsInterface) {
    resetGameState();
    bots.get(blackBotSelect.value)?.(jsInterface);
    gameState.blackIsHuman = blackBotSelect.value === "Human";
    jsInterface.set_bot_as_black();

    bots.get(whiteBotSelect.value)?.(jsInterface);
    gameState.whiteIsHuman = whiteBotSelect.value === "Human";
    jsInterface.set_bot_as_white();

    jsInterface.create_game();

    // const numRounds = parseInt(numRoundsInput.value);
    // console.log(jsInterface.bot_run_to_end_times(numRounds > 0 ? numRounds : 1));

    renderBoard(jsInterface);
    gameState.gameActive = true;

    // in case bot is first, run bot move
    runBotMove(jsInterface);
}

function resetGameState() {
    gameState.turn = false;
    gameState.gameActive = false;
    gameState.blackIsHuman = true;
    gameState.whiteIsHuman = true;
    history.clear();
}

/**
 * @param {JsInterface} jsInterface
 */
function renderBoard(jsInterface) {
    const jsRep = getBoard(jsInterface);

    for (let y = 0; y < 8; y++) {
        for (let x = 0; x < 8; x++) {
            if (jsRep[y][x] !== lastBoardContents[y][x]) {
                const elm = boardCellContents[y][x];
                renderGameCell(elm, jsRep[y][x]);
            }
        }
    }

    const whites = jsInterface.board_count_pieces(true);
    const blacks = jsInterface.board_count_pieces(false);
    whiteScore.replaceContents(whites.toString());
    blackScore.replaceContents(blacks.toString());

    if (whites > blacks) {
        whiteScore.class("leading");
        blackScore.removeClass("leading");
    } else if (blacks > whites) {
        whiteScore.removeClass("leading");
        blackScore.class("leading");
    } else {
        whiteScore.removeClass("leading");
        blackScore.removeClass("leading");
    }
}

/**
 * @param {Elm} cell 
 * @param {bigint} rep 
 */
function renderGameCell(cell, rep) {
    switch (rep) {
        case 0n:
            cell.removeClass("black")
            cell.removeClass("white");
            cell.class("blank");
            cell.clear();
            break;
        case 1n:
            cell.removeClass("black")
            cell.class("white");
            cell.removeClass("blank");
            cell.replaceContents("W");
            break;
        case -1n:
            cell.class("black")
            cell.removeClass("white");
            cell.removeClass("blank");
            cell.replaceContents("B");
            break;
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

/** @param {JsInterface} jsInterface */
function getBoard(jsInterface) {
    let filled = jsInterface.get_board_filled();
    let color = jsInterface.get_board_color();

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

let lastWidth = innerWidth;
let lastHeight = innerHeight;
let resizeAttempts = 0;

function onResize() {
    // can't test, but remember iOS resize events (infamously) needing a workaround
    if (lastWidth === innerWidth && lastHeight === innerHeight) {
        resizeAttempts = 0;
        onResizeTick();
    } else {
        updateGameCellSize();
    }
}

function onResizeTick() {
    if (lastWidth === innerWidth && lastHeight === innerHeight) {
        if (resizeAttempts < 10) {
            resizeAttempts++;
            setTimeout(() => { onResizeTick(); }, 100);
        }
    } else {
        updateGameCellSize();
    }
}

function estimateAvailableWidth() {
    const innerWidthMinusPadding = innerWidth - 16;

    if (innerWidth > 640) {
        return innerWidthMinusPadding - sideElm.clientWidth - 8; // 8: margin with sideElm
    } else {
        return innerWidthMinusPadding;
    }
}

function updateGameCellSize() {
    lastWidth = innerWidth;
    lastHeight = innerHeight;

    let availableSize = Math.min(estimateAvailableWidth(), innerHeight * 0.8) - 1; // 16: padding, 1: cell border width

    const spacePerCell = availableSize / 8;
    const cellSize = spacePerCell - 1; // 1: cell border width

    const cellContentSize = cellSize * 0.8;

    document.body.style.setProperty("--game-cell-width", cellSize + "px");
    document.body.style.setProperty("--game-cell-content-width", cellContentSize + "px"); // 8: cell padding
    document.body.style.setProperty("--game-cell-content-padding", (cellSize - cellContentSize) / 2 + "px"); // 8: cell padding
}

initAll();

function initExplainOthelloBoards() {
    const boards = document.querySelectorAll("code.language-othello");
    for (const board of boards) {
        const text = board.innerHTML;
        let elm = board;

        if (board.parentElement?.tagName === "PRE") {
            elm = board.parentElement;
        }

        elm.replaceWith(createExplainOthelloBoard(text));
    }
}

/**
 * @param {string} text
 */
function createExplainOthelloBoard(text) {
    const table = new Elm("table").class("gameTable", "readonly").attribute("style", "--game-cell-width: 32px; --game-cell-content-width: 25.6px; --game-cell-content-padding: 3.2px;");
    const tbody = new Elm("tbody").appendTo(table);

    const textArr = text.split("\n").filter(x => x).map(x => x.split(" ").filter(x => x));
    const mappings = {
        'x': -1n,
        'X': -1n,
        'o': 1n,
        'O': 1n,
        '_': 0n
    };

    for (let y = 0; y < textArr.length; y++) {
        const boardRow = [];
        const tr = new Elm("tr").appendTo(tbody);

        for (let x = 0; x < textArr[y].length; x++) {
            const cell = new Elm("td")
                .class("gameCell")
                .appendTo(tr);

            const content = new Elm().class("gameCellContent", "blank").appendTo(cell);

            renderGameCell(content, mappings[textArr[y][x]] ?? 0n);

            if (['X', 'O'].includes(textArr[y][x])) {
                content.class("highlighted");
            }

            boardRow.push(content);
        }

        boardCellContents.push(boardRow);
        lastBoardContents.push([0, 0, 0, 0, 0, 0, 0, 0]);
    }

    return new Elm("p").append(table).elm;
}