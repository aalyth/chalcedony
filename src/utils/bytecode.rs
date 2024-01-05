use crate::lexer::Type;

use super::PtrString;

#[derive(Debug)]
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

    CreateVar(usize),
    DeleteVar(usize),
    GetVar(usize),

    CallFunc(usize),
    Return,

    If(usize), // how much to jump over if the top of the stack is false
    Jmp(isize),

    Assert(Type),
    Print,
    Cast(Type),

    Debug,
}

/*
impl TryInto<Bytecode> for Type {
    type Error = ();
    fn try_into(self) -> Result<Bytecode, Self::Error> {
        match self {
            Type::Int => Ok(Bytecode::ConstI),
            Type::Uint => Ok(Bytecode::ConstU),
            Type::Float => Ok(Bytecode::ConstF),
            Type::Str => Ok(Bytecode::ConstS),
            Type::Bool => Ok(Bytecode::ConstB),
            _ => Err(()),
        }
    }
}

/* converts constant types into actual types */
impl TryInto<Type> for Bytecode {
    type Error = ();

    fn try_into(self) -> Result<Type, Self::Error> {
        match self {
            Bytecode::ConstI => Ok(Type::Int),
            Bytecode::ConstU => Ok(Type::Uint),
            Bytecode::ConstF => Ok(Type::Float),
            Bytecode::ConstS => Ok(Type::Str),
            Bytecode::ConstB => Ok(Type::Bool),
            _ => Err(()),
        }
    }
}
*/
