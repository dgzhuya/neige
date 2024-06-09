pub mod fpb;

pub fn random() -> usize {
    let ptr = Box::into_raw(Box::new(123));
    ptr as usize
}

pub fn float_to_integer(n: f64) -> Option<i64> {
    let i = n as i64;
    if i as f64 == n {
        Some(i)
    } else {
        None
    }
}

pub fn float_quote_to_integer(n: &f64) -> i64 {
    let i = *n as i64;
    if i as f64 == *n {
        i
    } else {
        panic!("{} can not change to number", n)
    }
}
