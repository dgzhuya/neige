use neige_infra::{
    state::{CallApi, SetApi},
    value::value::LuaValue,
    LUA_RIDX_GLOBALS,
};

use crate::state::LuaState;

impl SetApi for LuaState {
    fn set_table(&mut self, idx: isize) {
        let t = self.stack_get(idx);
        let v = self.stack_pop();
        let k = self.stack_pop();
        self.inline_set_table(t, k, v, false)
    }

    fn set_field(&mut self, idx: isize, key: &str) {
        let t = self.stack_get(idx);
        let v = self.stack_pop();
        self.inline_set_table(t, LuaValue::Str(key.into()), v, false)
    }

    fn set_i(&mut self, idx: isize, i: i64) {
        let t = self.stack_get(idx);
        let v = self.stack_pop();
        self.inline_set_table(t, LuaValue::Integer(i), v, false)
    }

    fn set_meta_table(&mut self, idx: isize) {
        let val = self.stack_get(idx);
        let mt_tb = self.stack_pop();
        match mt_tb {
            LuaValue::Nil => self.inline_set_meta_table(&val, None),
            LuaValue::Table(tb) => self.inline_set_meta_table(&val, Some(tb)),
            _ => panic!("table expected!"),
        }
    }

    fn raw_set(&mut self, idx: isize) {
        let t = self.stack_get(idx);
        let v = self.stack_pop();
        let k = self.stack_pop();
        self.inline_set_table(t, k, v, true)
    }

    fn raw_set_i(&mut self, idx: isize, i: i64) {
        let t = self.stack_get(idx);
        let v = self.stack_pop();
        self.inline_set_table(t, LuaValue::Integer(i), v, true);
    }

    fn set_global(&mut self, name: &str) {
        let t = self.registry_get(&LuaValue::Integer(LUA_RIDX_GLOBALS));
        let v = self.stack_pop();
        self.inline_set_table(t, LuaValue::Str(name.into()), v, false)
    }
}

impl LuaState {
    fn inline_set_table(&mut self, t: LuaValue, k: LuaValue, v: LuaValue, raw: bool) {
        if let LuaValue::Table(tbl) = &t {
            if raw || !tbl.get(&k).is_nil() || !tbl.has_meta_field("__newindex") {
                tbl.put(k, v);
                return;
            }
        }
        if !raw {
            let mf = self.inline_get_meta_field(&t, "__newindex");
            match &mf {
                LuaValue::Table(_) => {
                    self.inline_set_table(mf, k, v, false);
                    return;
                }
                LuaValue::Function(_) => {
                    self.stack_push(mf);
                    self.stack_push(t);
                    self.stack_push(k);
                    self.stack_push(v);
                    self.call(3, 0);
                    return;
                }
                _ => {}
            }
        }
        panic!("not a table!")
    }
}
