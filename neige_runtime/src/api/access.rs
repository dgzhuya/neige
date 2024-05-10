use neige_infra::{
    state::AccessApi,
    value::{closure::RustFn, value::LuaValue},
    LuaType,
};

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
        if self.stack_is_valid(idx) {
            self.stack_get(idx).type_of()
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
        let t = self.ty_id(idx);
        t == LuaType::String || t == LuaType::Number || t == LuaType::Table
    }

    fn is_rust_fn(&self, idx: isize) -> bool {
        let val = self.stack_get(idx);
        if let LuaValue::Function(f) = val {
            if let Some(_) = f.rust_fn {
                return true;
            }
        }
        return false;
    }

    fn to_boolean(&self, idx: isize) -> bool {
        self.stack_get(idx).convert_to_boolean()
    }

    fn to_integer(&self, idx: isize) -> i64 {
        self.to_integer_x(idx).unwrap_or(0)
    }

    fn to_integer_x(&self, idx: isize) -> Option<i64> {
        self.stack_get(idx).convert_to_integer()
    }

    fn to_number(&self, idx: isize) -> f64 {
        self.to_number_x(idx).unwrap_or(0.0)
    }

    fn to_number_x(&self, idx: isize) -> Option<f64> {
        self.stack_get(idx).convert_to_float()
    }

    fn to_string(&self, idx: isize) -> String {
        self.to_string_x(idx).unwrap_or(format!(""))
    }

    fn to_string_x(&self, idx: isize) -> Option<String> {
        let val = self.stack_get(idx);
        match val {
            LuaValue::Integer(i) => {
                let s = format!("{}", i);
                self.stack_set(idx, LuaValue::Str(s.clone()));
                Some(s)
            }
            LuaValue::Number(f) => {
                let s = format!("{}", f);
                self.stack_set(idx, LuaValue::Str(s.clone()));
                Some(s)
            }
            LuaValue::Str(s) => Some(s),
            LuaValue::Table(_) => todo!(),
            _ => None,
        }
    }

    fn to_rust_fn(&self, idx: isize) -> Option<RustFn> {
        let val = self.stack_get(idx);
        if let LuaValue::Function(f) = val {
            if let Some(f) = &f.rust_fn {
                return Some(*f.as_ref());
            }
        }
        None
    }

    fn raw_len(&self, idx: isize) -> usize {
        let val = self.stack_get(idx);
        match val {
            LuaValue::Str(s) => s.len(),
            LuaValue::Table(t) => t.as_ref().arr.borrow().len(),
            _ => 0,
        }
    }
}
