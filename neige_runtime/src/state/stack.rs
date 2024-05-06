use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use neige_infra::value::{closure::Closure, value::LuaValue};

struct LuaStack {
    slots: Vec<LuaValue>,
    top: isize,
    prev: Option<Rc<RefCell<LuaStack>>>,
    closure: Option<Rc<RefCell<Closure>>>,
    varargs: Option<Vec<LuaValue>>,
}
