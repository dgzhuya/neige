use crate::{
    api::{AccessApi, LuaVm, PushApi, StackApi},
    state::LuaState,
};

impl LuaState {
    pub(super) fn pop_result(&mut self, a: isize, c: isize) {
        if c > 1 {
            let mut i = a + c - 2;
            while i >= a {
                self.replace(i);
                i -= 1;
            }
        } else if c < 1 {
            self.check_stack(1);
            self.push_integer(a as i64)
        }
    }

    pub(super) fn push_func_and_args(&mut self, a: isize, b: isize) -> isize {
        if b >= 1 {
            self.check_stack(b as usize);
            for i in a..a + b {
                self.push_value(i)
            }
            b - 1
        } else {
            self.fix_stack(a);
            self.get_top() - self.register_count() as isize - 1
        }
    }

    pub(super) fn fix_stack(&mut self, a: isize) {
        let x = self.to_integer(-1) as isize;
        self.pop(1);
        if x > a {
            self.check_stack((x - a) as usize);
        }
        for i in a..x {
            self.push_value(i)
        }
        self.rotate(self.register_count() as isize + 1, x - a)
    }
}
