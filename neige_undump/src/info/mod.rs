use neige_infra::{code::inst::Instruction, Prototype};

pub trait ProtoPrint {
    fn list_proto(&self);
    fn print_header(&self);
    fn print_code(&self);
    fn print_detail(&self);
}

impl ProtoPrint for Prototype {
    fn list_proto(&self) {
        self.print_header();
        self.print_code();
        self.print_detail();
        for proto in self.protos.iter() {
            proto.list_proto()
        }
    }

    fn print_header(&self) {
        let func_type = if self.line_defined > 0 {
            "function"
        } else {
            "main"
        };

        let vararg_flag = if self.is_vararg > 0 { "+" } else { "" };
        let source = if let Some(s) = &self.source { s } else { "" };

        print!(
            "\n{} <{}:{}, {}> ({} instructions)\n",
            func_type,
            source,
            self.line_defined,
            self.last_line_defined,
            self.code.len()
        );
        print!(
            "{}{} params, {} slots, {} upvalues, {} locals, {} constants, {} functions\n",
            self.num_params,
            vararg_flag,
            self.max_stack_size,
            self.upvalues.len(),
            self.loc_vars.len(),
            self.constants.len(),
            self.protos.len()
        )
    }

    fn print_code(&self) {
        for (i, code) in self.code.iter().enumerate() {
            let line = if self.line_info.len() > 0 {
                self.line_info[i].to_string()
            } else {
                "-".to_string()
            };
            print!("\t{}\t[{}]\t", i + 1, line);
            print_oprands(code);
        }
    }

    fn print_detail(&self) {
        println!("constants  ({}):", self.constants.len());
        for (i, k) in self.constants.iter().enumerate() {
            println!("\t{}\t{}", i + 1, k.to_string())
        }
        println!("locals  ({}):", self.loc_vars.len());
        for (i, v) in self.loc_vars.iter().enumerate() {
            println!(
                "\t{}\t{}\t{}\t{}",
                i + 1,
                v.var_name,
                v.start_pc + 1,
                v.end_pc + 1
            )
        }
        println!("upvalues  ({}):", self.upvalues.len());
        for (i, u) in self.upvalues.iter().enumerate() {
            let name = if self.upvalue_names.len() > 0 {
                self.upvalue_names[i].clone()
            } else {
                format!("-")
            };
            println!("\t{}\t{}\t{}\t{}", i, name, u.in_stack, u.idx)
        }
    }
}

fn print_oprands(op: &Instruction) {
    let op_arg = |n: &u16| {
        if *n > 0xff {
            format!("{}", -1 - ((*n & 0xff) as isize))
        } else {
            format!(" {}", *n as isize)
        }
    };
    let op_arg_sbx = |n: &i32| {
        if *n < 0 {
            format!("{}", n)
        } else {
            format!(" {}", n)
        }
    };
    match op {
        Instruction::Move(a, b, c) => {
            println!("MOVE      {}  {}  {}", a, b, c);
        }
        Instruction::LoadK(a, bx) => {
            println!("LOADK     {}  {}", a, -1 - (*bx as isize));
        }
        Instruction::LoadKx(a, _) => {
            println!("LOADKX    {}", a);
        }
        Instruction::LoadBool(a, b, c) => {
            println!("LOADBOOL  {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::LoadNil(a, b, _) => {
            println!("LOADNIL   {}  {}", a, op_arg(b));
        }
        Instruction::GetUpVal(a, b, _) => {
            println!("GETUPVAL  {}  {}", a, op_arg(b));
        }
        Instruction::GetTabUp(a, b, c) => {
            println!("GETTABUP  {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::GetTable(a, b, c) => {
            println!("GETTABLE  {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::SetTabUp(a, b, c) => {
            println!("SETTABUP  {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::SetUpVal(a, b, _) => {
            println!("SETUPVAL  {}  {}", a, op_arg(b));
        }
        Instruction::SetTable(a, b, c) => {
            println!("SETTABLE  {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::NetTable(a, b, c) => {
            println!("NETTABLE  {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::_Self(a, b, c) => {
            println!("SELF      {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Add(a, b, c) => {
            println!("ADD       {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Sub(a, b, c) => {
            println!("SUB       {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Mul(a, b, c) => {
            println!("MUL       {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Mod(a, b, c) => {
            println!("MOD       {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Pow(a, b, c) => {
            println!("POW       {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Div(a, b, c) => {
            println!("DIV       {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::IDiv(a, b, c) => {
            println!("IDIV      {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::BAnd(a, b, c) => {
            println!("BAND      {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Bor(a, b, c) => {
            println!("BOR       {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::BXor(a, b, c) => {
            println!("BXOR      {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Shl(a, b, c) => {
            println!("SHL       {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Shr(a, b, c) => {
            println!("SHR       {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Unm(a, b, _) => {
            println!("UNM       {}  {}", a, op_arg(b));
        }
        Instruction::BNot(a, b, _) => {
            println!("BNOT      {}  {}", a, op_arg(b));
        }
        Instruction::Not(a, b, _) => {
            println!("NOT       {}  {}", a, op_arg(b));
        }
        Instruction::Length(a, b, _) => {
            println!("LENGTH    {}  {}", a, op_arg(b));
        }
        Instruction::Concat(a, b, c) => {
            println!("CONCAT    {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Jmp(a, sbx) => {
            println!("JMP       {}  {}", a, op_arg_sbx(sbx));
        }
        Instruction::Eq(a, b, c) => {
            println!("EQ        {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Lt(a, b, c) => {
            println!("LT        {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Le(a, b, c) => {
            println!("LE        {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Test(a, _, c) => {
            println!("TEST      {}  {}", a, op_arg(c));
        }
        Instruction::TestSet(a, b, c) => {
            println!("TESTSET   {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Call(a, b, c) => {
            println!("CALL      {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::TailCall(a, b, _) => {
            println!("TAILCALL  {}  {}", a, op_arg(b));
        }
        Instruction::Return(a, b, _) => {
            println!("RETURN    {}  {}", a, op_arg(b));
        }
        Instruction::ForLoop(a, sbx) => {
            println!("FORLOOP   {}  {}", a, op_arg_sbx(sbx));
        }
        Instruction::ForPrep(a, sbx) => {
            println!("FORPREP   {}  {}", a, op_arg_sbx(sbx));
        }
        Instruction::TForCall(a, _, c) => {
            println!("TFORCALL  {}  {}", a, op_arg(c));
        }
        Instruction::TForLoop(a, sbx) => {
            println!("TFORLOOP  {}  {}", a, op_arg_sbx(sbx));
        }
        Instruction::SetList(a, b, c) => {
            println!("SETLIST   {}  {}  {}", a, op_arg(b), op_arg(c));
        }
        Instruction::Closure(a, bx) => {
            println!("CLOSURE   {}   {}", a, bx);
        }
        Instruction::Vararg(a, b, _) => {
            println!("VARARG    {}  {}", a, op_arg(b));
        }
        Instruction::ExtraArg(ax) => {
            println!("EXTRAARG  {}", -1 - (*ax as isize));
        }
    }
}
