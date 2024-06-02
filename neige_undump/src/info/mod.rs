use neige_infra::{code::inst::Instruction, Constant, Prototype};

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
            print_oprands(code, &self.constants);
            println!()
        }
    }

    fn print_detail(&self) {
        println!("constants ({}):", self.constants.len());
        for (i, k) in self.constants.iter().enumerate() {
            println!("\t{}\t{}", i + 1, k.to_string())
        }
        println!("locals ({}):", self.loc_vars.len());
        for (i, v) in self.loc_vars.iter().enumerate() {
            println!(
                "\t{}\t{}\t{}\t{}",
                i + 1,
                v.var_name,
                v.start_pc + 1,
                v.end_pc + 1
            )
        }
        println!("upvalues ({}):", self.upvalues.len());
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

fn print_oprands(op: &Instruction, consts: &Vec<Constant>) {
    match op {
        Instruction::Move(a, b, c) => {
            println!("MOVE     {}\t{}\t{}\t", a, b, c);
        }
        Instruction::LoadK(_, _) => {
            println!("LoadK    ");
        }
        Instruction::LoadKx(_, _) => {
            println!("LoadKx   ");
        }
        Instruction::LoadBool(_, _, _) => {
            println!("LoadBool ");
        }
        Instruction::LoadNil(_, _, _) => {
            println!("LoadNil  ");
        }
        Instruction::GetUpVal(_, _, _) => {
            println!("GetUpVal ");
        }
        Instruction::GetTabUp(_, _, _) => {
            println!("GetTabUp ");
        }
        Instruction::GetTable(_, _, _) => {
            println!("GetTable ");
        }
        Instruction::SetTabUp(_, _, _) => {
            println!("SetTabUp ");
        }
        Instruction::SetUpVal(_, _, _) => {
            println!("SetUpVal ");
        }
        Instruction::SetTable(_, _, _) => {
            println!("SetTable ");
        }
        Instruction::NetTable(_, _, _) => {
            println!("NetTable ");
        }
        Instruction::_Self(_, _, _) => {
            println!("Self     ");
        }
        Instruction::Add(_, _, _) => {
            println!("Add      ");
        }
        Instruction::Sub(_, _, _) => {
            println!("Sub      ");
        }
        Instruction::Mul(_, _, _) => {
            println!("Mul      ");
        }
        Instruction::Mod(_, _, _) => {
            println!("Mod      ");
        }
        Instruction::Pow(_, _, _) => {
            println!("Pow      ");
        }
        Instruction::Div(_, _, _) => {
            println!("Div      ");
        }
        Instruction::IDiv(_, _, _) => {
            println!("IDiv    ");
        }
        Instruction::BAnd(_, _, _) => {
            println!("BAnd     ");
        }
        Instruction::Bor(_, _, _) => {
            println!("Bor      ");
        }
        Instruction::BXor(_, _, _) => {
            println!("BXor     ");
        }
        Instruction::Shl(_, _, _) => {
            println!("Shl      ");
        }
        Instruction::Shr(_, _, _) => {
            println!("Shr      ");
        }
        Instruction::Unm(_, _, _) => {
            println!("Unm      ");
        }
        Instruction::BNot(_, _, _) => {
            println!("BNot     ");
        }
        Instruction::Not(_, _, _) => {
            println!("Not      ");
        }
        Instruction::Length(_, _, _) => {
            println!("Length   ");
        }
        Instruction::Concat(_, _, _) => {
            println!("Concat   ");
        }
        Instruction::Jmp(_, _) => {
            println!("Jmp      ");
        }
        Instruction::Eq(_, _, _) => {
            println!("Eq       ");
        }
        Instruction::Lt(_, _, _) => {
            println!("Lt       ");
        }
        Instruction::Le(_, _, _) => {
            println!("Le       ");
        }
        Instruction::Test(_, _, _) => {
            println!("Test     ");
        }
        Instruction::TestSet(_, _, _) => {
            println!("TestSet  ");
        }
        Instruction::Call(_, _, _) => {
            println!("Call     ");
        }
        Instruction::TailCall(_, _, _) => {
            println!("TailCall ");
        }
        Instruction::Return(_, _, _) => {
            println!("Return   ");
        }
        Instruction::ForLoop(_, _) => {
            println!("ForLoop  ");
        }
        Instruction::ForPrep(_, _) => {
            println!("ForPrep  ");
        }
        Instruction::TForCall(_, _, _) => {
            println!("TForCall ");
        }
        Instruction::TForLoop(_, _) => {
            println!("TForLoop ");
        }
        Instruction::SetList(_, _, _) => {
            println!("SetList  ");
        }
        Instruction::Closure(_, _) => {
            println!("Closure  ");
        }
        Instruction::Vararg(_, _, _) => {
            println!("Vararg   ");
        }
        Instruction::ExtraArg(_) => {
            println!("ExtraArg ");
        }
    }
}
