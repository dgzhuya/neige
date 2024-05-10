use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
struct LuaNode {
    stack: Option<Rc<RefCell<LuaStack>>>,
    val: u8,
}

impl LuaNode {
    fn new() -> Rc<RefCell<LuaNode>> {
        let state = Rc::new(RefCell::new(LuaNode {
            stack: None,
            val: 1,
        }));
        let stack = LuaStack::new(&state.clone(), 2);
        state.borrow_mut().stack = Some(stack);
        state
    }

    fn push_stack(&mut self, stack: Rc<RefCell<LuaStack>>) {
        stack.borrow_mut().prev = self.stack.clone();
        self.stack = Some(stack);
    }

    fn pop_stack(&mut self) {
        if let Some(stack) = self.stack.clone() {
            self.stack = stack.borrow().prev.clone();
            stack.borrow_mut().prev = None
        }
    }

    fn display(&self) {
        println!("state {}", self.val);
        if let Some(s) = &self.stack {
            s.borrow_mut().display();
        }
    }
}

#[derive(Debug, Clone)]
struct LuaStack {
    prev: Option<Rc<RefCell<LuaStack>>>,
    state: Weak<RefCell<LuaNode>>,
    val: u8,
}

impl LuaStack {
    fn new(state: &Rc<RefCell<LuaNode>>, val: u8) -> Rc<RefCell<LuaStack>> {
        Rc::new(RefCell::new(Self {
            val,
            prev: None,
            state: Rc::downgrade(state),
        }))
    }

    fn display(&mut self) {
        println!("cur {}", self.val);
        if let Some(p) = &self.prev {
            p.borrow_mut().display();
        };
    }

    fn get_state(&mut self) -> Rc<RefCell<LuaNode>> {
        if let Some(node) = self.state.upgrade() {
            node.clone()
        } else {
            panic!("stack have not state")
        }
    }
}

fn main() {
    let state = LuaNode::new();
    let stack = LuaStack::new(&state, 3);
    stack.borrow_mut().get_state();
    state.borrow_mut().push_stack(stack);
    let stack = LuaStack::new(&state, 4);
    state.borrow_mut().push_stack(stack);
    let stack = LuaStack::new(&state, 5);
    state.borrow_mut().push_stack(stack);
    state.borrow().display();
    state.borrow_mut().pop_stack();
    println!("pop");
    state.borrow().display();
    state.borrow_mut().pop_stack();
    println!("pop");
    state.borrow().display();
    state.borrow_mut().pop_stack();
    println!("pop");
    state.borrow().display();
}
