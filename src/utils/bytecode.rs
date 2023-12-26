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

    OpCreateFunc = 30, // <name: u64> <len: u64> <body> -> len marks the number of bytes as body
    OpCallFunc = 31,   // <name: u64> -> calls the function with given name
    OpReturn = 32,     // terminate the current function's execution

    OpIf = 35, // <len: u64> <body> -> if the top of the stack is true continue, else jump over the body
    OpJmp = 36, // <distance: i64> -> jumps forward the given distance (goes back if negative)

    OpAssertType = 50, // <type: u8> -> asserts the top of the stack is of given type
    OpPrint = 51,      // prints the value at the top of the stack

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
