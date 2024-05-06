use neige_infra::state::AccessApi;

use crate::state::LuaState;

#[allow(unused_variables)]
impl AccessApi for LuaState {
    fn tp_name(&self, tp: i8) -> &str {
        todo!()
    }

    fn ty_id(&self, idx: isize) -> neige_infra::LuaType {
        todo!()
    }

    fn is_none(&self, idx: isize) -> bool {
        todo!()
    }

    fn is_none_or_nil(&self, idx: isize) -> bool {
        todo!()
    }

    fn is_boolean(&self, idx: isize) -> bool {
        todo!()
    }

    fn is_integer(&self, idx: isize) -> bool {
        todo!()
    }

    fn is_number(&self, idx: isize) -> bool {
        todo!()
    }

    fn is_string(&self, idx: isize) -> bool {
        todo!()
    }

    fn is_rust_fn(&self, idx: isize) -> bool {
        todo!()
    }

    fn to_boolean(&self, idx: isize) -> bool {
        todo!()
    }

    fn to_integer(&self, idx: isize) -> i64 {
        todo!()
    }

    fn to_integer_x(&self, idx: isize) -> (i64, bool) {
        todo!()
    }

    fn to_number(&self, idx: isize) -> f64 {
        todo!()
    }

    fn to_number_x(&self, idx: isize) -> (f64, bool) {
        todo!()
    }

    fn to_string(&self, idx: isize) -> String {
        todo!()
    }

    fn to_string_x(&self, idx: isize) -> (String, bool) {
        todo!()
    }

    fn to_rust_fn(&self, idx: isize) -> neige_infra::value::closure::RustFn {
        todo!()
    }

    fn raw_len(&self, idx: isize) -> usize {
        todo!()
    }
}
