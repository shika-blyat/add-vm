#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Reg(Register),
    Int(i32),
}
#[derive(Clone, Debug, PartialEq)]
pub struct Register {
    pub idx: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    name: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Instr {
    Label(Val),
    RegAssign(Register, Box<Instr>),
    Deref(Val),
    DerefAssign(Val, Val),
    Jump(Label),
    JumpGreater(Val, Val, Label),
    JumpGreaterEqual(Val, Val, Label),
    Greater(Val, Val),
    GreaterEqual(Val, Val),
    Add(Val, Val),
    Mul(Val, Val),
    Div(Val, Val),
    Pow(Val, Val),
    Sqr(Val, Val),
    Sqrt(Val, Val),
    Val(Val),
}

pub type Instructions = Vec<Instr>;
