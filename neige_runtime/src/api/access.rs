use neige_infra::{state::AccessApi, value::value::LuaValue, LuaType};

use crate::state::LuaState;

#[allow(unused_variables)]
impl AccessApi for LuaState {
    /// 获取类型名
    fn tp_name(&self, tp: LuaType) -> &str {
        match tp {
            LuaType::None => "no value",
            LuaType::Nil => "nil",
            LuaType::Boolean => "boolean",
            LuaType::LightUserData => "light user data",
            LuaType::Number => "number",
            LuaType::String => "string",
            LuaType::Table => "table",
            LuaType::Function => "function",
            LuaType::UserData => "user data",
            LuaType::Thread => "thread",
            LuaType::Integer => "integer",
        }
    }

    fn ty_id(&self, idx: isize) -> neige_infra::LuaType {
        let node = self.get_node();
        let stack = node.get_stack();
        if stack.is_valid(idx) {
            let val = stack.get(idx);
            val.type_of()
        } else {
            LuaType::None
        }
    }

    fn is_none(&self, idx: isize) -> bool {
        self.ty_id(idx) == LuaType::None
    }

    fn is_nil(&self, idx: isize) -> bool {
        self.ty_id(idx) == LuaType::Nil
    }

    fn is_none_or_nil(&self, idx: isize) -> bool {
        self.is_nil(idx) || self.is_none(idx)
    }

    fn is_boolean(&self, idx: isize) -> bool {
        self.ty_id(idx) == LuaType::Boolean
    }

    fn is_integer(&self, idx: isize) -> bool {
        self.ty_id(idx) == LuaType::Integer
    }

    fn is_number(&self, idx: isize) -> bool {
        self.ty_id(idx) == LuaType::Number
    }

    fn is_string(&self, idx: isize) -> bool {
        self.ty_id(idx) == LuaType::String
    }

    fn is_rust_fn(&self, idx: isize) -> bool {
        let node = self.get_node();
        let stack = node.get_stack();
        if let LuaValue::Function(f) = stack.get(idx) {
            if let Some(_) = f.rust_fn {
                return true;
            }
        }
        return false;
    }

    fn to_boolean(&self, idx: isize) -> bool {
        let node = self.get_node();
        let stack = node.get_stack();

        let val = stack.get(idx);
        convert_to_boolean(&val)
    }

    fn to_integer(&self, idx: isize) -> i64 {
        todo!()
    }

    fn to_integer_x(&self, idx: isize) -> (i64, bool) {
        todo!()
    }

    fn to_number(&self, idx: isize) -> f64 {
        todo!()
    }

    fn to_number_x(&self, idx: isize) -> (f64, bool) {
        todo!()
    }

    fn to_string(&self, idx: isize) -> String {
        todo!()
    }

    fn to_string_x(&self, idx: isize) -> (String, bool) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        let val = stack.get(idx);
        match val {
            LuaValue::Integer(i) => {
                let s = format!("{}", i);
                stack.set(idx, LuaValue::Str(s.clone()));
                (s, true)
            }
            LuaValue::Number(f) => {
                let s = format!("{}", f);
                stack.set(idx, LuaValue::Str(s.clone()));
                (s, true)
            }
            LuaValue::Str(s) => (s, true),
            LuaValue::Table(_) => todo!(),
            _ => (format!(""), false),
        }
    }

    fn to_rust_fn(&self, idx: isize) -> neige_infra::value::closure::RustFn {
        todo!()
    }

    fn raw_len(&self, idx: isize) -> usize {
        todo!()
    }
}

fn convert_to_boolean(val: &LuaValue) -> bool {
    match val {
        LuaValue::Nil => false,
        LuaValue::Boolean(b) => *b,
        _ => true,
    }
}
