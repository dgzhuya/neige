use gloo_utils::format::JsValueSerdeExt;
use neige_runtime::{
    api::{GetApi, LuaApi, PushApi, SetApi},
    state::LuaState,
    LuaValue,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/ts/lib.ts")]
extern "C" {
    pub fn genFontCode(name: &str, val: &JsValue);
}

fn gen_font_code(ls: &mut dyn LuaApi) -> usize {
    let info = ls.to_string(-2);
    let route = if let Some(info) = ls.to_lua_tbl(-1) {
        if let Ok(val) = JsValue::from_serde(&LuaValue::Table(info)) {
            val
        } else {
            JsValue::null()
        }
    } else {
        JsValue::null()
    };
    genFontCode(&info, &route);
    0
}

pub trait LuaNodeLib {
    fn node_lib(&mut self);
}

impl LuaNodeLib for LuaState {
    fn node_lib(&mut self) {
        self.create_table(0, 1);
        self.push_rust_fn(gen_font_code);
        self.set_field(-2, "genFontCode");
        self.set_global("NodeJs");
    }
}
