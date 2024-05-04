pub trait OpName {
    #[allow(dead_code)]
    fn get_op_name(&self) -> &str;
}

impl OpName for u8 {
    fn get_op_name(&self) -> &str {
        match self {
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
}
