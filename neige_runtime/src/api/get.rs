use neige_infra::{
    state::{CallApi, GetApi},
    value::value::LuaValue,
    LuaType, LUA_RIDX_GLOBALS,
};

use crate::state::LuaState;

impl GetApi for LuaState {
    fn new_table(&mut self) {
        self.create_table(0, 0)
    }

    fn create_table(&mut self, n_arr: usize, n_rec: usize) {
        let tb = LuaValue::new_table(n_arr, n_rec);
        self.stack_push(tb);
    }

    fn get_table(&mut self, idx: isize) -> LuaType {
        let t = self.stack_get(idx);
        let k = self.stack_pop();
        self.inline_get_table(t, k, false)
    }

    fn get_field(&mut self, idx: isize, key: String) -> LuaType {
        let t = self.stack_get(idx);
        self.inline_get_table(t, LuaValue::Str(key), false)
    }

    fn get_i(&mut self, idx: isize, i: i64) -> LuaType {
        let t = self.stack_get(idx);
        self.inline_get_table(t, LuaValue::Integer(i), false)
    }

    fn get_meta_table(&mut self, idx: isize) -> bool {
        let val = self.stack_get(idx);
        if let Some(mt) = self.inline_get_meta_table(&val) {
            self.stack_push(LuaValue::Table(mt));
            true
        } else {
            false
        }
    }

    fn raw_get(&mut self, idx: isize) -> LuaType {
        let t = self.stack_get(idx);
        let k = self.stack_pop();
        self.inline_get_table(t, k, true)
    }

    fn raw_get_i(&mut self, idx: isize, i: i64) -> LuaType {
        let t = self.stack_get(idx);
        self.inline_get_table(t, LuaValue::Integer(i), true)
    }

    fn get_global(&mut self, name: String) -> LuaType {
        let t = self.registry_get(&LuaValue::Integer(LUA_RIDX_GLOBALS));
        self.inline_get_table(t, LuaValue::Str(name), true)
    }
}

impl LuaState {
    fn inline_get_table(&mut self, t: LuaValue, k: LuaValue, raw: bool) -> LuaType {
        if let LuaValue::Table(tbl) = &t {
            let v = tbl.get(&k);
            let v_type = v.type_of();
            if raw || !v.is_nil() || !tbl.has_meta_field("__index") {
                self.stack_push(v);
                return v_type;
            }
        }

        if !raw {
            let mf = self.inline_get_meta_field(&t, "__index");
            match mf {
                LuaValue::Table(_) => {
                    return self.inline_get_table(mf, k, false);
                }
                LuaValue::Function(_) => {
                    self.stack_push(mf);
                    self.stack_push(t);
                    self.stack_push(k);
                    self.call(2, 1);
                    return self.stack_get(-1).type_of();
                }
                _ => {}
            }
        }

        panic!("not a lua table!")
    }
}
