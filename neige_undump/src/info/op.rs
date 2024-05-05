use neige_infra::code::convert::DecodeOrder;

#[derive(PartialEq)]
pub(super) enum OpArgMode {
    ArgN,
    ArgU,
    ArgR,
    ArgK,
}

pub trait OpName {
    fn get_op_name(&self) -> &str;
    fn get_op_b_mode(&self) -> OpArgMode;
    fn get_op_c_mode(&self) -> OpArgMode;
}

impl OpName for u32 {
    fn get_op_name(&self) -> &str {
        let op = self.get_op();
        match op {
            0 => "MOVE     ",
            1 => "LoadK    ",
            2 => "LoadKX   ",
            3 => "LOADBOOL ",
            4 => "LoadNIL  ",
            5 => "GETUPVAL ",
            6 => "GETTABUP ",
            7 => "GETTABLE ",
            8 => "SETTABUP ",
            9 => "SETTABVAL",
            10 => "SETTABLE ",
            11 => "NEWTABLE ",
            12 => "SELF     ",
            13 => "ADD      ",
            14 => "SUB      ",
            15 => "MUL      ",
            16 => "MOD      ",
            17 => "POW      ",
            18 => "DIV      ",
            19 => "IDIV     ",
            20 => "BAND     ",
            21 => "BOR      ",
            22 => "BXOR     ",
            23 => "SHL      ",
            24 => "SHR      ",
            25 => "UNM      ",
            26 => "BNOT     ",
            27 => "NOT      ",
            28 => "LEN      ",
            29 => "CONCAT   ",
            30 => "JMP      ",
            31 => "EQ       ",
            32 => "LT       ",
            33 => "LE       ",
            34 => "TEST     ",
            35 => "TESTSET  ",
            36 => "CALL     ",
            37 => "TAILCALL ",
            38 => "RETURN   ",
            39 => "FORLOOP  ",
            40 => "FORPREP  ",
            41 => "TFORCALL ",
            42 => "TFORLOOP ",
            43 => "SETLIST  ",
            44 => "CLOSURE  ",
            45 => "VARARG   ",
            46 => "EXTRAARG ",
            _ => "",
        }
    }

    fn get_op_b_mode(&self) -> OpArgMode {
        let op = self.get_op();
        match op {
            2 | 34 | 41 => OpArgMode::ArgN,
            3 | 4 | 5 | 6 | 9 | 11 | 36 | 37 | 38 | 43 | 44 | 45 | 46 => OpArgMode::ArgU,
            0 | 7 | 12 | 25 | 26 | 27 | 28 | 29 | 30 | 35 | 39 | 40 | 42 => OpArgMode::ArgR,
            _ => OpArgMode::ArgK,
        }
    }

    fn get_op_c_mode(&self) -> OpArgMode {
        let op = self.get_op();
        match op {
            29 => OpArgMode::ArgR,
            3 | 11 | 34 | 35 | 36 | 41 | 43 | 46 => OpArgMode::ArgU,
            0 | 1 | 2 | 4 | 5 | 9 | 25 | 26 | 27 | 28 | 30 | 37 | 38 | 39 | 40 | 42 | 44 | 45 => {
                OpArgMode::ArgN
            }
            _ => OpArgMode::ArgK,
        }
    }
}
