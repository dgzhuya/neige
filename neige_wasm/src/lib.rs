mod node;

use neige_runtime::{api::CallApi, state::LuaState};
use node::LuaNodeLib;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/ts/lib.ts")]
extern "C" {
    pub fn logger(info: &str);
}
#[wasm_bindgen]
pub fn run(data: &[u8], file_name: &str) {
    logger("start");
    let mut state = LuaState::new();
    state.aux_lib();
    state.node_lib();
    state.load(data.to_vec(), file_name, "bt");
    state.call(0, 0);
    logger("end")
}
