use neige_infra::state::CallApi;

use crate::state::LuaState;

impl CallApi for LuaState {
    fn load(&mut self, chunk: Vec<u8>, chunk_name: &str, mode: &str) {
        todo!()
    }

    fn call(&mut self, n_args: isize, n_result: isize) {
        todo!()
    }
}
