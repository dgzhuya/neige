use neige_infra::{state::CallApi, value::value::LuaValue};

use crate::state::LuaState;

#[allow(unused_variables)]
impl CallApi for LuaState {
    fn load(&mut self, chunk: Vec<u8>, chunk_name: &str, mode: &str) {
        todo!()
    }

    fn call(&mut self, n_args: isize, n_result: isize) {
        let val = self.stack_get(-(n_args + 1));
        if let LuaValue::Function(c) = val {
            // println!("{:?}", c)
        }
    }
}
