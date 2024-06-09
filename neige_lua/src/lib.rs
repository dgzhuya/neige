pub mod api;
pub mod libs;
pub mod state;
mod value;
mod vm;

pub use value::value::LuaValue;

#[cfg(test)]
#[cfg(not(feature = "wasm"))]
mod tests {

    use std::io::BufReader;

    use neige_infra::{read_file, state::CallApi};

    use crate::state::LuaState;

    #[test]
    fn test_lua_state() {
        let file = read_file("example/test.out").unwrap();
        let data = BufReader::new(file);
        let mut state = LuaState::new();
        state.aux_lib();
        state.load(data, "test.out", "bt");
        state.call(0, 0);
    }
}
