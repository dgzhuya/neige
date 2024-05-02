use std::rc::Rc;

use neige_infra::{Constant, LocVar, Prototype, Upvalue};

use crate::binary::{
    CINT_SIZE, CSIZET_SIZE, INSTRUCTION_SIZE, LUAC_DATA, LUAC_FORMAT, LUAC_INT, LUAC_NUM,
    LUAC_VERSION, LUA_INTEGER_SIZE, LUA_NUMBER_SIZE, LUA_SIGNATURE,
};

pub struct Reader {
    data: Vec<u8>,
    pos: usize,
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, pos: 0 }
    }

    pub fn read_byte(&mut self) -> u8 {
        let b = self.data[self.pos];
        self.pos += 1;
        b
    }

    fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.read_byte())
        }
        v
    }

    fn read_u32(&mut self) -> u32 {
        let a = self.read_byte() as u32;
        let b = self.read_byte() as u32;
        let c = self.read_byte() as u32;
        let d = self.read_byte() as u32;
        d << 24 | c << 16 | b << 8 | a
    }

    fn read_u64(&mut self) -> u64 {
        let a = self.read_u32() as u64;
        let b = self.read_u32() as u64;
        b << 32 | a
    }

    fn read_vec<T, F>(&mut self, f: F) -> Vec<T>
    where
        F: Fn(&mut Self) -> T,
    {
        let n = self.read_u32() as usize;
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(f(self))
        }
        v
    }
}

impl Reader {
    fn read_lua_integer(&mut self) -> i64 {
        self.read_u64() as i64
    }

    fn read_lua_number(&mut self) -> f64 {
        use std::f64;
        f64::from_bits(self.read_u64())
    }

    fn read_string(&mut self) -> String {
        match self.read_byte() as usize {
            0 => String::new(),
            mut x => {
                if x == 0xff {
                    x = self.read_u64() as usize
                }
                let bytes = self.read_bytes(x - 1);
                String::from_utf8(bytes).unwrap_or(String::new())
            }
        }
    }

    pub fn check_header(&mut self) {
        assert_eq!(
            self.read_bytes(4),
            LUA_SIGNATURE,
            "not a precompiled chunk!"
        );
        assert_eq!(self.read_byte(), LUAC_VERSION, "version mismatch!");
        assert_eq!(self.read_byte(), LUAC_FORMAT, "format mismatch!");
        assert_eq!(self.read_bytes(6), LUAC_DATA, "corrupted!");
        assert_eq!(self.read_byte(), CINT_SIZE, "int size mismatch!");
        assert_eq!(self.read_byte(), CSIZET_SIZE, "size_t size mismatch!");
        assert_eq!(
            self.read_byte(),
            INSTRUCTION_SIZE,
            "instruction size mismatch!"
        );
        assert_eq!(
            self.read_byte(),
            LUA_INTEGER_SIZE,
            "lua_Integer size mismatch!"
        );
        assert_eq!(
            self.read_byte(),
            LUA_NUMBER_SIZE,
            "lua_Number size mismatch!"
        );
        assert_eq!(self.read_lua_integer(), LUAC_INT, "endianness mismatch!");
        assert_eq!(self.read_lua_number(), LUAC_NUM, "float format mismatch!");
    }
}

impl Reader {
    pub fn read_proto(&mut self, parent_source: Option<String>) -> Rc<Prototype> {
        let s = self.read_string();
        let source = if s.len() > 0 { Some(s) } else { parent_source };

        Rc::new(Prototype {
            source: source.clone(),
            line_defined: self.read_u32(),
            last_line_defined: self.read_u32(),
            num_params: self.read_byte(),
            is_vararg: self.read_byte(),
            max_stack_size: self.read_byte(),
            code: self.read_vec(|r| r.read_u32().into()),
            constants: self.read_vec(|r| r.read_constant()),
            upvalues: self.read_vec(|r| r.read_upvalue()),
            protos: self.read_vec(|r| r.read_proto(source.clone())),
            line_info: self.read_vec(|r| r.read_u32()),
            loc_vars: self.read_vec(|r| r.read_loc_var()),
            upvalue_names: self.read_vec(|r| r.read_string()),
        })
    }

    fn read_constant(&mut self) -> Constant {
        let tag = self.read_byte();
        match tag {
            0 => Constant::Nil,
            1 => Constant::Boolean(self.read_byte() != 0),
            3 => Constant::Number(self.read_lua_number()),
            4 => Constant::Str(self.read_string()),
            19 => Constant::Integer(self.read_lua_integer()),
            20 => Constant::Str(self.read_string()),
            _ => panic!("error"),
        }
    }

    fn read_upvalue(&mut self) -> Upvalue {
        Upvalue {
            in_stack: self.read_byte(),
            idx: self.read_byte(),
        }
    }

    fn read_loc_var(&mut self) -> LocVar {
        LocVar {
            var_name: self.read_string(),
            start_pc: self.read_u32(),
            end_pc: self.read_u32(),
        }
    }
}
