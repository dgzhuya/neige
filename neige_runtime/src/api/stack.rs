use neige_infra::{state::StackApi, value::value::LuaValue};

use crate::state::LuaState;

impl StackApi for LuaState {
    fn get_top(&self) -> isize {
        let node = self.get_node();
        let stack = node.get_stack();
        stack.top as isize
    }

    fn abs_index(&self, idx: isize) -> isize {
        self.stack_abs_index(idx)
    }

    fn check_stack(&mut self, n: usize) -> bool {
        self.stack_check(n);
        true
    }

    fn pop(&mut self, n: isize) {
        self.set_top(n - 1)
    }

    fn copy(&mut self, from_idx: isize, to_idx: isize) {
        let val = self.stack_get(from_idx);
        self.stack_set(to_idx, val)
    }

    fn push_value(&mut self, idx: isize) {
        let val = self.stack_get(idx);
        self.stack_push(val)
    }

    fn replace(&mut self, idx: isize) {
        let val = self.stack_pop();
        self.stack_set(idx, val);
    }

    fn insert(&mut self, idx: isize) {
        self.rotate(idx, 1)
    }

    fn remove(&mut self, idx: isize) {
        self.rotate(idx, -1);
        self.pop(1)
    }

    fn rotate(&mut self, idx: isize, n: isize) {
        let t = self.get_top() - 1;
        let p = self.abs_index(idx) - 1;
        let m = if n > 0 { t - n } else { p - n - 1 };
        self.stack_reverse(p, m);
        self.stack_reverse(m + 1, t);
        self.stack_reverse(p, t);
    }

    fn set_top(&mut self, idx: isize) {
        let new_top = self.stack_abs_index(idx);
        if new_top < 0 {
            panic!("stack overflow")
        }
        let n = self.get_top() - new_top;
        if n > 0 {
            self.stack_pop_n(n as usize);
        } else {
            for _ in n..0 {
                self.stack_push(LuaValue::Nil)
            }
        }
    }
}
