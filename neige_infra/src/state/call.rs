/// 函数调用trait
///
/// 数量 `2`
pub trait CallApi {
    fn load(&mut self, chunk: Vec<u8>, chunk_name: &str, mode: &str);
    fn call(&mut self, n_args: isize, n_results: isize);
}
