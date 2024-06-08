use crate::value::closure::RustFn;

/// 传入API
///
/// 数量 `8`
pub trait PushApi {
    fn push_nil(&mut self);
    fn push_boolean(&mut self, b: bool);
    fn push_integer(&mut self, i: i64);
    fn push_number(&mut self, f: f64);
    fn push_string(&mut self, s: &str);
    fn push_rust_fn(&mut self, f: RustFn);
    fn register(&mut self, name: &str, f: RustFn);
    fn push_global_table(&mut self);
    fn push_rust_closure(&mut self, f: RustFn, n: usize);
}
