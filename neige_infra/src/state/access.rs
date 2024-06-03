use std::rc::Rc;

use crate::{
    constant::LuaType,
    value::{closure::RustFn, table::LuaTable},
};

/// 访问API
///
/// 数量 `19`
pub trait AccessApi {
    fn tp_name(&self, tp: LuaType) -> &str;
    fn ty_id(&self, idx: isize) -> LuaType;
    fn is_none(&self, idx: isize) -> bool;
    fn is_nil(&self, idx: isize) -> bool;
    fn is_none_or_nil(&self, idx: isize) -> bool;
    fn is_boolean(&self, idx: isize) -> bool;
    fn is_integer(&self, idx: isize) -> bool;
    fn is_number(&self, idx: isize) -> bool;
    fn is_string(&self, idx: isize) -> bool;
    fn is_rust_fn(&self, idx: isize) -> bool;
    fn is_lua_tbl(&self, idx: isize) -> bool;
    fn to_boolean(&self, idx: isize) -> bool;
    fn to_integer(&self, idx: isize) -> i64;
    fn to_integer_x(&self, idx: isize) -> Option<i64>;
    fn to_number(&self, idx: isize) -> f64;
    fn to_number_x(&self, idx: isize) -> Option<f64>;
    fn to_string(&mut self, idx: isize) -> String;
    fn to_string_x(&mut self, idx: isize) -> Option<String>;
    fn to_rust_fn(&self, idx: isize) -> Option<RustFn>;
    fn to_lua_tbl(&self, idx: isize) -> Option<Rc<LuaTable>>;
    fn raw_len(&self, idx: isize) -> usize;
}
