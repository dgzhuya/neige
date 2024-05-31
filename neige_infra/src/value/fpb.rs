pub fn fb_2_isize(x: isize) -> usize {
    (if x < 8 {
        x
    } else {
        ((x & 7) + 8) << (((x >> 3) - 1) as usize)
    }) as usize
}
