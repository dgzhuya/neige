mod op;

use neige_infra::{code::inst_mode::OpMode, Prototype};

use crate::info::op::{OpArgMode, OpName};

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
            print!("\t{}\t[{}]\t{}\t", i + 1, line, code.get_op_name());
            print_oprands(*code);
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

fn print_oprands(op: u32) {
    let op_mode: OpMode = op.into();
    match op_mode {
        OpMode::IABC(a, b, c) => {
            print!("{}", a);
            if op.get_op_b_mode() != OpArgMode::ArgN {
                let b = if b > 0xff {
                    -1 - ((b as i32) & 0xff)
                } else {
                    b.into()
                };
                print!(" {}", b);
            }
            if op.get_op_c_mode() != OpArgMode::ArgN {
                let c = if c > 0xff {
                    -1 - ((c as i32) & 0xff)
                } else {
                    c.into()
                };
                print!(" {}", c);
            }
        }
        OpMode::IABx(a, bx) => {
            print!("{}", a);
            match op.get_op_b_mode() {
                OpArgMode::ArgU => print!(" {}", -1 - (bx as i32)),
                OpArgMode::ArgK => print!(" {}", bx),
                _ => {}
            }
        }
        OpMode::IAsBx(a, s_bx) => print!("{} {}", a, s_bx),
        OpMode::IAx(ax) => print!(" {}", -1 - (ax as i32)),
    }
}
