pub trait PushApi {
    fn push_nil(&mut self);
    fn push_boolean(&mut self, b: bool);
    fn push_integer(&mut self, i: i64);
    fn push_number(&mut self, f: f64);
    fn push_string(&mut self, s: String);
}
