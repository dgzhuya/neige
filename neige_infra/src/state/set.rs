/// 设置表API
///
/// 数量 `7`
pub trait SetApi {
    fn set_table(&mut self, idx: isize);
    fn set_field(&mut self, idx: isize, key: String);
    fn set_i(&mut self, idx: isize, i: i64);
    fn set_meta_table(&mut self, idx: isize);
    fn raw_set(&mut self, idx: isize);
    fn raw_set_i(&mut self, idx: isize, i: i64);
    fn set_global(&mut self, name: String);
}
