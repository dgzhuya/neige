mod api;
mod math;
mod state;
mod vm;

#[cfg(test)]
mod tests {
    use neige_infra::state::LuaVm;

    use crate::state;

    #[test]
    fn test() {
        state::LuaState.fetch();
    }
}
