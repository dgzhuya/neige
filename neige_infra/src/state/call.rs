pub trait CallApi {
    fn load(&mut self, chunk: Vec<u8>, chunk_name: &str, mode: &str);
    fn call(&mut self, n_args: isize, n_result: isize);
}
