/* tslint:disable */
/* eslint-disable */

export class WasmGame {
    free(): void;
    [Symbol.dispose](): void;
    attempt_move_tableau_to_foundation(from: number, idx: number): boolean;
    attempt_move_tableau_to_tableau(from: number, to: number, count: number): boolean;
    attempt_move_waste_to_foundation(idx: number): boolean;
    attempt_move_waste_to_tableau(col: number): boolean;
    auto_move_tableau_to_foundation(col: number): boolean;
    auto_move_tableau_to_tableau(col: number, count: number): boolean;
    auto_move_waste_to_foundation(): boolean;
    auto_move_waste_to_tableau(): boolean;
    can_undo(): boolean;
    flush_waste(): boolean;
    from_deck_to_waste(): boolean;
    get_state(): any;
    is_game_won(): boolean;
    constructor();
    undo(): boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_wasmgame_free: (a: number, b: number) => void;
    readonly wasmgame_attempt_move_tableau_to_foundation: (a: number, b: number, c: number) => number;
    readonly wasmgame_attempt_move_tableau_to_tableau: (a: number, b: number, c: number, d: number) => number;
    readonly wasmgame_attempt_move_waste_to_foundation: (a: number, b: number) => number;
    readonly wasmgame_attempt_move_waste_to_tableau: (a: number, b: number) => number;
    readonly wasmgame_auto_move_tableau_to_foundation: (a: number, b: number) => number;
    readonly wasmgame_auto_move_tableau_to_tableau: (a: number, b: number, c: number) => number;
    readonly wasmgame_auto_move_waste_to_foundation: (a: number) => number;
    readonly wasmgame_auto_move_waste_to_tableau: (a: number) => number;
    readonly wasmgame_can_undo: (a: number) => number;
    readonly wasmgame_flush_waste: (a: number) => number;
    readonly wasmgame_from_deck_to_waste: (a: number) => number;
    readonly wasmgame_get_state: (a: number) => any;
    readonly wasmgame_is_game_won: (a: number) => number;
    readonly wasmgame_new: () => number;
    readonly wasmgame_undo: (a: number) => number;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
