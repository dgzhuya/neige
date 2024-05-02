use super::convert::DecodeOrder;

/// 指令集模式
pub enum OpMode {
    IABC(u8, u16, u16),
    IABx(u8, u32),
    IAsBx(u8, i32),
    IAx(u32),
}

impl From<u32> for OpMode {
    fn from(value: u32) -> Self {
        let code = value.get_op();
        match code {
            1 | 2 | 44 => {
                let (a, bx) = value.get_a_bx();
                OpMode::IABx(a, bx)
            }
            30 | 39 | 40 | 42 => {
                let (a, s_bx) = value.get_a_sbx();
                OpMode::IAsBx(a, s_bx)
            }
            46 => OpMode::IAx(value.get_ax()),
            _ => {
                let (a, b, c) = value.get_abc();
                OpMode::IABC(a, b, c)
            }
        }
    }
}
