import { botsMap } from "./botsList.js";
import init, { JsInterface } from "./wasm/pkg/wasm.js";

// runs a tournament with the bots

init().then(() => {
    const jsi = JsInterface.new();

    /** @type {Map<string, number>} */
    const elos = new Map();
    /** @type {string[]} */
    const botList = [];

    for (const [botName, _] of botsMap) {
        elos.set(botName, 0);
        botList.push(botName);
    }

    let i = 0;
    while (true) {
        console.log("Running round " + (++i));

        for (const blackPlayer of botList) {
            for (const whitePlayer of botList) {
                botsMap.get(blackPlayer)?.(jsi);
                jsi.set_bot_as_black();

                botsMap.get(whitePlayer)?.(jsi);
                jsi.set_bot_as_white();

                jsi.create_game();
                jsi.bot_run_to_end();

                let whiteScore = jsi.board_count_pieces(true);
                let blackScore = jsi.board_count_pieces(false);

                if (whiteScore > blackScore) {
                    win(elos, whitePlayer, blackPlayer);
                } else if (whiteScore < blackScore) {
                    win(elos, blackPlayer, whitePlayer);
                } else {
                    draw(elos, blackPlayer, whitePlayer);
                }
            }
        }

        let outStr = [];
        for (const [botName, elo] of elos) {
            outStr.push(`${botName}: ${elo}`);
        }
        console.log(outStr.join("\n"));
    }
});

/**
 * @param {Map<string, number>} elos 
 * @param {string} playerA 
 * @param {string} playerB 
 */
function draw(elos, playerA, playerB) {
    /** @type {number} */ // @ts-ignore
    const ratingA = elos.get(playerA);
    /** @type {number} */ // @ts-ignore
    const ratingB = elos.get(playerB);

    const newRatingA = ratingA + 16 * (0.5 - eloExpectedScore(ratingA, ratingB));
    const newRatingB = ratingB + 16 * (0.5 - eloExpectedScore(ratingB, ratingA));

    elos.set(playerA, newRatingA);
    elos.set(playerB, newRatingB);
}

/**
 * @param {Map<string, number>} elos 
 * @param {string} winner
 * @param {string} loser
 */
function win(elos, winner, loser) {
    /** @type {number} */ // @ts-ignore
    const ratingWinner = elos.get(winner);
    /** @type {number} */ // @ts-ignore
    const ratingLoser = elos.get(loser);

    const newRatingWinner = ratingWinner + 16 * (1 - eloExpectedScore(ratingWinner, ratingLoser));
    const newRatingLoser = ratingLoser + 16 * (0 - eloExpectedScore(ratingLoser, ratingWinner));

    elos.set(winner, newRatingWinner);
    elos.set(loser, newRatingLoser);
}

/**
 * @param {number} rating
 * @param {number} opponentRating
 */
function eloExpectedScore(rating, opponentRating) {
    return 1 / (1 + 10 ** ((opponentRating - rating) / 400))
}