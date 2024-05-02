use super::MAX_ARG_S_BX;

pub trait DecodeOrder {
    fn get_op(&self) -> u8;
    fn get_abc(&self) -> (u8, u16, u16);
    fn get_a_bx(&self) -> (u8, u32);

    fn get_ax(&self) -> u32;

    fn get_a_sbx(&self) -> (u8, i32) {
        let (a, bx) = self.get_a_bx();
        (a, (bx as i32) - MAX_ARG_S_BX)
    }
}

impl DecodeOrder for u32 {
    fn get_op(&self) -> u8 {
        (self.clone() as u8) & 0x3f
    }

    fn get_abc(&self) -> (u8, u16, u16) {
        let a = (self >> 6) as u8 & 0xff;
        let c = (self >> 14) as u16 & 0x1ff;
        let b = (self >> 23) as u16 & 0x1ff;
        (a, b, c)
    }

    fn get_a_bx(&self) -> (u8, u32) {
        let a = (self >> 6) as u8 & 0xff;
        let bx = self >> 14;
        (a, bx)
    }

    fn get_ax(&self) -> u32 {
        self >> 6
    }
}
