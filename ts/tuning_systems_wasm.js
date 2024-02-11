import * as wasm from "./tuning_systems_wasm_bg.wasm";
import { __wbg_set_wasm } from "./tuning_systems_wasm_bg.js";
__wbg_set_wasm(wasm);
export * from "./tuning_systems_wasm_bg.js";
