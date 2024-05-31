mod api;
mod libs;
mod state;
mod vm;

#[cfg(test)]
mod tests {

    use crate::state::LuaState;

    #[test]
    fn test_lua_state() {
        #[allow(unused_variables)]
        let state = LuaState::new();
    }
}
