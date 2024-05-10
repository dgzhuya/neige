use neige_infra::state::PushApi;

use crate::state::LuaState;

#[allow(unused_variables)]
impl PushApi for LuaState {
    fn push_nil(&mut self) {
        todo!()
    }

    fn push_boolean(&mut self, b: bool) {
        todo!()
    }

    fn push_integer(&mut self, i: i64) {
        todo!()
    }

    fn push_number(&mut self, f: f64) {
        todo!()
    }

    fn push_string(&mut self, s: String) {
        todo!()
    }

    fn push_rust_fn(&mut self, f: neige_infra::value::closure::RustFn) {}

    fn register(&mut self, name: String, f: neige_infra::value::closure::RustFn) {
        todo!()
    }

    fn push_global_table(&mut self) {
        todo!()
    }

    fn push_rust_closure(&mut self, f: neige_infra::value::closure::RustFn, n: isize) {
        todo!()
    }
}
