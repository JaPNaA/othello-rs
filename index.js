import init, { JsInterface } from "./wasm/pkg/wasm.js";
import { Elm } from "./elements.js";

/** @type {HTMLDivElement} */ // @ts-ignore
const game = document.getElementById("game");

/** @type {HTMLButtonElement} */ // @ts-ignore
const rerunButton = document.getElementById("rerunButton");


/** @type {HTMLSelectElement} */ // @ts-ignore
const blackBotSelect = document.getElementById("blackBotSelect");
/** @type {HTMLSelectElement} */ // @ts-ignore
const whiteBotSelect = document.getElementById("whiteBotSelect");
/** @type {HTMLInputElement} */ // @ts-ignore
const numRoundsInput = document.getElementById("numRounds")

/** @type {Elm[][]} */
const boardCellContents = [];

const lastBoardContents = [];

/** @type {((x: number, y: number) => void)[]} */
const clickListeners = [];

const whiteScore = new Elm("span").class("score");
const blackScore = new Elm("span").class("score");
const gameOverDisplay = new Elm("div");

/** @type {Map<string, (jsi: JsInterface) => void} */
const bots = new Map([
    ["Human", (jsi) => { }],
    ["Center Bot", (jsi) => jsi.create_new_center_bot()],
    ["Deep Minmax Bot", (jsi) => jsi.create_deep_minmax_bot()],
    ["Deep Minmax Bot without Alpha-Beta pruning (debug)", (jsi) => jsi.create_deep_minmax_pre_alpha_beta_bot()],
    ["Edge Bot", (jsi) => jsi.create_new_edge_bot()],
    ["Edge Exclusive Bot", (jsi) => jsi.create_new_edge_exclusive_bot()],
    ["Top Left Bot", (jsi) => jsi.create_new_first_valid_move_bot()],
    ["Bottom Right Bot", (jsi) => jsi.create_new_last_valid_move_bot()],
    ["Minmax Score Bot", (jsi) => jsi.create_new_minmax_score_bot()],
    ["Random Bot", (jsi) => jsi.create_new_random_bot()],
    ["Shallow Score Bot", (jsi) => jsi.create_shallow_score_bot()],
]);

initBotSelector();
initBoard();

init().then(() => {
    const jsInterface = JsInterface.new();
    runGame(jsInterface);

    rerunButton.addEventListener("click", () => {
        runGame(jsInterface);
    });

    clickListeners.push((x, y) => {
        if (jsInterface.board_try_place(x, y, false)) {
            // currTurn = !currTurn;
            setTimeout(() => {
                jsInterface.bot_run_white();
                renderBoard(jsInterface);
            }, 300);
        }

        renderBoard(jsInterface);
    });
});

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

    const table = new Elm("table").class("gameTable").appendTo(game);
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
}

/**
 * @param {JsInterface} jsInterface 
 */
function runGame(jsInterface) {
    bots.get(blackBotSelect.value)?.(jsInterface);
    jsInterface.set_bot_as_black();

    bots.get(whiteBotSelect.value)?.(jsInterface);
    jsInterface.set_bot_as_white();

    jsInterface.create_game();

    const numRounds = parseInt(numRoundsInput.value);
    console.log(jsInterface.bot_run_to_end_times(numRounds > 0 ? numRounds : 1));

    renderBoard(jsInterface);
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
                switch (jsRep[y][x]) {
                    case 0n:
                        elm.removeClass("black")
                        elm.removeClass("white");
                        elm.class("blank");
                        elm.clear();
                        break;
                    case 1n:
                        elm.removeClass("black")
                        elm.class("white");
                        elm.removeClass("blank");
                        elm.replaceContents("W");
                        break;
                    case -1n:
                        elm.class("black")
                        elm.removeClass("white");
                        elm.removeClass("blank");
                        elm.replaceContents("B");
                        break;
                }
            }
        }
    }

    const whites = jsInterface.board_count_pieces(true);
    const blacks = jsInterface.board_count_pieces(false);
    whiteScore.replaceContents(whites);
    blackScore.replaceContents(blacks);

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