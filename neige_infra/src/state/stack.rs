pub trait StackApi {
    fn get_top(&self) -> isize;
    fn abs_index(&self, idx: isize) -> isize;
    fn check_stack(&mut self, n: usize) -> bool;
    fn pop(&mut self, n: u32);
    fn copy(&mut self, from_idx: isize, to_idx: isize);
    fn push_value(&mut self, idx: isize);
    fn replace(&mut self, idx: isize);
    fn insert(&mut self, idx: isize);
    fn remove(&mut self, idx: isize);
    fn rotate(&mut self, idx: isize, n: isize);
    fn set_top(&mut self, idx: isize);
}
