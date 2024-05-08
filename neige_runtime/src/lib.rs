mod api;
mod math;
mod state;
mod vm;

#[cfg(test)]
mod tests {
    use crate::state::LuaState;

    #[test]
    fn test_lua_state() {
        let state = LuaState::new();
        println!("{:#?}", state)
    }
}
