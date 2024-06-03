use std::{fs::File, io::BufReader, path::PathBuf};

use neige_infra::state::{CallApi, LuaApi, PushApi};
use neige_runtime::state::LuaState;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/js/lib.ts")]
extern "C" {
    pub fn genFontCode(val: &JsValue);
    pub fn log(info: &str);
}

fn gen_font_code(ls: &mut dyn LuaApi) -> usize {
    let n_args = ls.get_top() - 1;
    genFontCode(&JsValue::from_f64(n_args as f64));
    0
}

#[wasm_bindgen]
pub fn run(file_name: &str) {
    let path: PathBuf = file_name.into();
    if path.exists() {
        log(file_name);
        let file = File::open(path).unwrap();
        let data = BufReader::new(file);
        let mut state = LuaState::new();
        state.aux_lib();
        state.register("genFontCode", gen_font_code);
        state.load(data, file_name, "bt");
        state.call(0, 0)
    } else {
        log(&format!("{} not exists", file_name))
    }
}
