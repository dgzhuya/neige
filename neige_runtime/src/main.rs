use std::env;
use std::io::BufReader;

use neige_infra::read_file;
use neige_infra::state::CallApi;
use neige_runtime::state::LuaState;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file = read_file(&args[1]).unwrap();
        let data = BufReader::new(file);
        let mut state = LuaState::new();
        state.aux_lib();
        state.load(data, "test.out", "bt");
        state.call(0, 0);
    }
}
