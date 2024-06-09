use neige_infra::LuaType;

use crate::api::LuaApi;

pub(super) fn next(ls: &mut dyn LuaApi) -> usize {
    ls.set_top(2);
    if ls.next(1) {
        2
    } else {
        ls.push_nil();
        1
    }
}

pub(super) fn pairs(ls: &mut dyn LuaApi) -> usize {
    ls.push_rust_fn(next);
    ls.push_value(1);
    ls.push_nil();
    1
}

pub(super) fn ipairs(ls: &mut dyn LuaApi) -> usize {
    ls.push_rust_fn(ipairs_aux);
    ls.push_value(1);
    ls.push_integer(0);
    3
}

fn ipairs_aux(ls: &mut dyn LuaApi) -> usize {
    let i = ls.to_integer(2) + 1;
    ls.push_integer(i);
    if ls.get_i(1, i) == LuaType::Nil {
        1
    } else {
        2
    }
}
