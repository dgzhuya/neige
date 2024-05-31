use neige_infra::state::{LuaApi, PushApi};

use crate::state::LuaState;

fn pcall(ls: &mut dyn LuaApi) -> usize {
    let n_args = ls.get_top() - 1;
    let status = ls.pcall(n_args, -1, 0);
    ls.push_boolean(status == 0);
    ls.insert(1);
    ls.get_top() as usize
}

fn error(ls: &mut dyn LuaApi) -> usize {
    ls.error() as usize
}

fn get_meta_table(ls: &mut dyn LuaApi) -> usize {
    if !ls.get_meta_table(1) {
        ls.push_nil();
    }
    1
}

fn set_meta_table(ls: &mut dyn LuaApi) -> usize {
    ls.set_meta_table(1);
    1
}

fn lua_print(ls: &mut dyn LuaApi) -> usize {
    let n_args = ls.get_top();
    for i in 1..=n_args {
        if ls.is_boolean(i) {
            print!("{}", ls.to_boolean(i))
        } else if ls.is_string(i) {
            print!("{}", ls.to_string(i))
        } else {
            print!("{}", ls.tp_name(ls.ty_id(i)));
        }
        if i < n_args {
            print!("\t")
        }
    }
    println!();
    0
}

#[allow(dead_code)]
impl LuaState {
    pub(crate) fn aux_lib(&mut self) {
        self.register("print", lua_print);
        self.register("getmetatable", get_meta_table);
        self.register("setmetatable", set_meta_table);
        self.register("error", error);
        self.register("pcall", pcall);
        self.register("print", lua_print);
    }
}
