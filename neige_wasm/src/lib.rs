use gloo_utils::format::JsValueSerdeExt;
use neige_infra::{
    state::{CallApi, LuaApi, PushApi},
    value::value::LuaValue,
};
use neige_runtime::state::LuaState;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/js/lib.ts")]
extern "C" {
    pub fn genFontCode(val: &JsValue);
    pub fn log(info: &str);
}

fn gen_font_code(ls: &mut dyn LuaApi) -> usize {
    let info = ls.to_lua_tbl(-1);
    if let Some(info) = info {
        if let Ok(val) = JsValue::from_serde(&LuaValue::Table(info)) {
            genFontCode(&val);
        }
    }
    0
}

#[wasm_bindgen]
pub fn run(data: &[u8], file_name: &str) {
    let mut state = LuaState::new();
    state.aux_lib();
    state.register("genFontCode", gen_font_code);
    state.load(data.to_vec(), file_name, "bt");
    state.call(0, 0)
}
