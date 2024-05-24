use std::rc::Rc;

use neige_infra::{
    state::CallApi,
    value::{table::LuaTable, value::LuaValue},
};

use super::LuaState;

impl LuaState {
    fn call_meta_method(&mut self, a: LuaValue, b: LuaValue, mm_name: String) -> Option<LuaValue> {
        let mm = match self.get_meta_field(&a, &mm_name) {
            LuaValue::Nil => match self.get_meta_field(&b, &mm_name) {
                LuaValue::Nil => return None,
                val => val,
            },
            val => val,
        };
        self.stack_check(4);
        self.stack_push(mm);
        self.stack_push(a);
        self.stack_push(b);
        self.call(2, 1);
        Some(self.stack_pop())
    }

    fn get_meta_field(&self, val: &LuaValue, field_name: &str) -> LuaValue {
        if let Some(tbl) = self.get_meta_table(val) {
            tbl.get(&LuaValue::Str(format!("{}", field_name)))
        } else {
            LuaValue::Nil
        }
    }

    fn get_meta_table(&self, val: &LuaValue) -> Option<Rc<LuaTable>> {
        if let LuaValue::Table(tbl) = val {
            let m_tbl = tbl.meta_table.borrow().clone();
            if let Some(m_tbl) = m_tbl {
                return Some(m_tbl.clone());
            }
        }
        let key = LuaValue::Str(format!("_MT{}", val.type_of()));
        if let LuaValue::Table(tbl) = self.registry_get(&key) {
            let m_tbl = tbl.meta_table.borrow().clone();
            if let Some(m_tbl) = m_tbl {
                return Some(m_tbl.clone());
            }
        }
        None
    }

    fn set_meta_table(&self, val: &LuaValue, mt: Rc<LuaTable>) {
        if let LuaValue::Table(tbl) = val {
            let mut m_tb = tbl.meta_table.borrow_mut();
            *m_tb = Some(mt);
            return;
        }
        let key = LuaValue::Str(format!("_MT{}", val.type_of()));
        self.registry_set(key, LuaValue::Table(mt));
    }
}
