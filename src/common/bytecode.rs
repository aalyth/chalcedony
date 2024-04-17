use crate::utils::PtrString;

#[derive(Debug, Clone)]
pub enum Bytecode {
    Nop,

    ConstI(i64),
    ConstU(u64),
    ConstF(f64),
    ConstS(PtrString),
    ConstB(bool),
    ThrowException,

    // converts uint -> int
    CastI,
    // converts uint/int -> float
    CastF,

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

    SetGlobal(usize),
    GetGlobal(usize),
    SetArg(usize),
    GetArg(usize),
    SetLocal(usize),
    GetLocal(usize),

    CreateFunc(usize), // arg count
    CallFunc(usize),
    Return,
    ReturnVoid,

    If(usize), // how much to jump over if the top of the stack is false
    Jmp(isize),

    /* the length of the try-catch scope */
    TryScope(usize),
    /* jumping over the `catch` body, terminating the `try` block */
    CatchJmp(usize),

    Print,
    Assert, // asserts the top 2 values on the stack are equal
}
