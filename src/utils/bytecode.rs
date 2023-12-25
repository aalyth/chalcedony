use crate::lexer::Type;

#[repr(u8)]
pub enum Bytecode {
    OpConstI = 1,
    OpConstU = 2,
    OpConstF = 3,
    OpConstS = 4,
    OpConstB = 5,

    OpAdd = 10,
    OpSub = 11,
    OpMul = 12,
    OpDiv = 13,
    OpMod = 14,

    OpAnd = 15,
    OpOr = 16,
    OpLt = 17,
    OpGt = 18,
    OpEq = 19,
    OpLtEq = 20,
    OpGtEq = 21,

    OpNeg = 22,
    OpNot = 23,

    OpCreateVar = 25, // <name> -> creates a variable with value the top of the stack
    OpDeleteVar = 26, // <name>
    OpGetVar = 27,    // <name> -> pushes the given variable's value on the top of the stack

    OpCreateFunc = 30, // <name: str> <len: u64> <body> -> len marks the number of bytes as body
    OpCallFunc = 31,   // <name> -> calls the function with given name
    OpReturn = 32,     // terminate the current function's execution

    OpSetReg = 35, // <N> -> sets the register N's value to the top of the stack
    OpGetReg = 36, // <N> -> pushes register N's value to the top of the stack

    OpAssertType = 50, // <type> -> asserts the top of the stack is of given type

    OpDebug = 200, // prints debug info for the CVM
}

impl TryInto<Bytecode> for Type {
    type Error = ();
    fn try_into(self) -> Result<Bytecode, Self::Error> {
        match self {
            Type::Int => Ok(Bytecode::OpConstI),
            Type::Uint => Ok(Bytecode::OpConstU),
            Type::Float => Ok(Bytecode::OpConstF),
            Type::Str => Ok(Bytecode::OpConstS),
            Type::Bool => Ok(Bytecode::OpConstB),
            _ => Err(()),
        }
    }
}

/* converts constant types into actual types */
impl TryInto<Type> for Bytecode {
    type Error = ();

    fn try_into(self) -> Result<Type, Self::Error> {
        match self {
            Bytecode::OpConstI => Ok(Type::Int),
            Bytecode::OpConstU => Ok(Type::Uint),
            Bytecode::OpConstF => Ok(Type::Float),
            Bytecode::OpConstS => Ok(Type::Str),
            Bytecode::OpConstB => Ok(Type::Bool),
            _ => Err(()),
        }
    }
}
