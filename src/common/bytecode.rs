use crate::utils::PtrString;

#[derive(Debug, Clone)]
pub enum Bytecode {
    Nop,

    ConstI(i64),
    ConstU(u64),
    ConstF(f64),
    ConstS(PtrString),
    ConstB(bool),

    ConstL, /* builds a new empty list */

    /* converts uint -> int */
    CastI,
    /* converts uint/int -> float */
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

    SetGlobal(usize), /* global id */
    GetGlobal(usize), /* global id */
    SetArg(usize),    /* arg id */
    GetArg(usize),    /* arg id */
    SetLocal(usize),  /* local id */
    GetLocal(usize),  /* local id */

    CreateFunc(usize), /* arg count */
    CallFunc(usize),   /* func id */
    Return,
    ReturnVoid,

    If(usize), /* how much to jump over if the top of the stack is false */
    Jmp(isize),

    LInsert(isize),
    LPop(usize),

    Print,
    Assert, /* asserts the top 2 values on the stack are equal */
}
