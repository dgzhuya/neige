use neige_infra::LUA_REGISTRY_INDEX;

pub(super) fn lua_upvalue_index(n: isize) -> isize {
    LUA_REGISTRY_INDEX - n
}

pub(super) fn u8_isize(n: &u8) -> isize {
    *n as isize
}

pub(super) fn u16_isize(n: &u16) -> isize {
    *n as isize
}

pub(super) fn u32_isize(n: &u32) -> isize {
    *n as isize
}

pub(super) fn i32_isize(n: &i32) -> isize {
    *n as isize
}
