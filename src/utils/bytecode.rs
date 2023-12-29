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

    OpCallFunc = 31, // <pos: u64> -> calls the function at the given position
    OpReturn = 32,   // terminate the current function's execution

    OpIf = 35, // <len: u64> <body> -> if the top of the stack is true continue, else jump over the body
    OpJmp = 36, // <distance: i64> -> jumps forward the given distance (goes back if negative)

    OpAssertType = 50, // <type: u8> -> asserts the top of the stack is of given type
    OpPrint = 51,      // prints the value at the top of the stack
    OpCast = 52,       // <type: u8> -> attempts to cast the top of the stack to the given value

    OpDebug = 200,    // prints debug info for the CVM
    OpStartLn = 201,  // <ln: u64> -> sets the start line
    OpStartCol = 202, // <col: u64> -> sets the start column
    OpEndLn = 203,    // <ln: u64> -> sets the end line
    OpEndCol = 204,   // <col: u64> -> sets the end column
    OpSetSpan = 205,  // <id: u16> -> sets the current span's id
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
