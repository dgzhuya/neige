/// 杂项API
///
/// 数量 `5`
pub trait MiscApi {
    fn len(&mut self, idx: isize);
    fn concat(&mut self, n: usize);
    fn next(&mut self, idx: isize);
    fn error(&mut self) -> isize;
    fn pcall(&mut self, n_args: isize, n_results: isize, msg: isize) -> isize;
}
