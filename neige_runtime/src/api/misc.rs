use neige_infra::state::MiscApi;

use crate::state::LuaState;

impl MiscApi for LuaState {
    fn len(&mut self, idx: isize) {
        todo!()
    }

    fn concat(&mut self, n: usize) {
        todo!()
    }

    fn next(&mut self, idx: isize) {
        todo!()
    }

    fn error(&mut self) {
        todo!()
    }

    fn pcall(&mut self, n_args: isize, n_results: isize, msg: isize) -> isize {
        todo!()
    }
}
