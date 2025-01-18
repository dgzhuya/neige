pub mod fpb;

/// 随机数生成器
pub fn random() -> usize {
    let ptr = Box::into_raw(Box::new(123));
    ptr as usize
}

/// 浮点数转整数
///
/// @pram n 浮点数
///
/// @return Option<i64> 整数
pub fn float_to_integer(n: f64) -> Option<i64> {
    let i = n as i64;
    if i as f64 == n {
        Some(i)
    } else {
        None
    }
}

/// 浮点数转整数
///
/// @pram n 浮点数
///
/// @return i64 整数
pub fn float_quote_to_integer(n: &f64) -> i64 {
    let i = *n as i64;
    if i as f64 == *n {
        i
    } else {
        panic!("{} can not change to number", n)
    }
}
