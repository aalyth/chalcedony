use crate::utils::PtrString;

#[derive(Debug, Clone)]
pub enum Bytecode {
    ConstI(i64),
    ConstU(u64),
    ConstF(f64),
    ConstS(PtrString),
    ConstB(bool),

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    And,
    Or,
    Lt,
    Gt,
    Eq,
    LtEq,
    GtEq,

    Neg,
    Not,

    CastInt,
    CastUint,
    CastFloat,
    CastStr,
    CastBool,

    SetGlobal(usize),
    GetGlobal(usize),
    // SetArg(usize),
    GetArg(usize),
    SetLocal(usize),
    GetLocal(usize),

    // arg count, locals count
    CreateFunc(usize, usize),
    CallFunc(usize),
    Return,
    // TODO: add a TailCallReturn operation
    If(usize), // how much to jump over if the top of the stack is false
    Jmp(isize),

    Print,

    Debug,
}
