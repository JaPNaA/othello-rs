/** @type {Map<string, (jsi: import("./wasm/pkg/wasm").JsInterface) => void>} */
export const botsMap = new Map([
    ["Deep Negative Heuristic Bot", (jsi) => jsi.create_deep_negative_heuristic_bot()],
    ["Center Bot", (jsi) => jsi.create_new_center_bot()],
    ["Bottom Right Bot", (jsi) => jsi.create_new_last_valid_move_bot()],
    ["Top Left Bot", (jsi) => jsi.create_new_first_valid_move_bot()],
    ["Random Bot", (jsi) => jsi.create_new_random_bot()],
    ["Shallow Score Bot", (jsi) => jsi.create_shallow_score_bot()],
    ["Edge Bot", (jsi) => jsi.create_new_edge_bot()],
    ["Edge Exclusive Bot", (jsi) => jsi.create_new_edge_exclusive_bot()],
    ["Minmax Score Bot", (jsi) => jsi.create_new_minmax_score_bot()],
    ["Deep Score Bot", (jsi) => jsi.create_deep_score_bot()],
    ["Deep Heuristic Bot", (jsi) => jsi.create_deep_heuristic_bot()],
]);
