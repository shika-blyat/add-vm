mod instructions;
use instructions::{Instr, Instructions, Register, Val};
use std::ops::Deref;

struct Vm {
    registers: [i32; 16],
    curr_instr: usize,
    instrs: Instructions,
}
impl Vm {
    #[inline(always)]
    fn assign_to_register(&mut self, idx: usize, val: i32) {
        self.registers[idx] = val;
    }
    pub fn eval_instrs(&mut self) {
        while self.curr_instr < self.instrs.len() {
            self.eval_instr(&self.current_instr().clone());
            self.advance_instr();
        }
    }
    fn eval_instr(&mut self, instr: &Instr) {
        match instr {
            Instr::RegAssign(reg, val) => {
                let val = self.eval_return_instr(val.deref());
                self.assign_to_register(reg.idx, val);
            }
            Instr::Add(val1, val2) => {
                let val1 = match val1 {
                    Val::Int(int) => *int,
                    Val::Reg(reg) => self.registers[reg.idx],
                };
                let val2 = match val2 {
                    Val::Int(int) => *int,
                    Val::Reg(reg) => self.registers[reg.idx],
                };
                println!("{}", val1 + val2);
            }
            _ => unreachable!(),
        }
    }
    #[inline]
    fn eval_return_instr(&mut self, instr: &Instr) -> i32 {
        match instr {
            Instr::Val(val) => match val {
                Val::Reg(reg) => self.registers[reg.idx],
                Val::Int(int) => *int,
            },
            _ => unreachable!(),
        }
    }
    fn current_instr(&self) -> &Instr {
        &self.instrs[self.curr_instr]
    }
    fn advance_instr(&mut self) {
        self.curr_instr += 1;
    }
}

fn main() {
    let mut vm = Vm {
        registers: [0; 16],
        instrs: vec![
            Instr::RegAssign(Register { idx: 0 }, Box::new(Instr::Val(Val::Int(15)))),
            Instr::Add(Val::Reg(Register { idx: 0 }), Val::Int(5)),
        ],
        curr_instr: 0,
    };
    vm.eval_instrs();
    println!("Hello, world!");
}
