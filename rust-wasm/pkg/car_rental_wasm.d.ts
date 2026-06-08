/* tslint:disable */
/* eslint-disable */

export function close_checkout(): void;

export function confirm_booking(): void;

export function init_app(): void;

export function load_preset(preset_key: string): void;

export function on_field_input(): void;

export function on_search(event: Event): void;

export function on_ui_field_input(ui_type: string, value: string): void;

export function on_ui_font_input(value: string): void;

export function open_checkout(car_index: number): void;

export function scroll_to_top(): void;

export function set_accent_color(hex: string): void;

export function show_toast(text: string): void;

export function toggle_drawer(open: boolean): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly close_checkout: () => void;
    readonly confirm_booking: () => void;
    readonly init_app: () => void;
    readonly load_preset: (a: number, b: number) => void;
    readonly on_field_input: () => void;
    readonly on_search: (a: any) => void;
    readonly on_ui_field_input: (a: number, b: number, c: number, d: number) => void;
    readonly on_ui_font_input: (a: number, b: number) => void;
    readonly open_checkout: (a: number) => void;
    readonly scroll_to_top: () => void;
    readonly set_accent_color: (a: number, b: number) => void;
    readonly show_toast: (a: number, b: number) => void;
    readonly toggle_drawer: (a: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h33e44da0d0f9814e: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
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
