mod operators;

use operators::{BinaryOperatorType, UnaryOperatorType};

pub enum VariableType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Str,
    Auto, // might remove later
    Custom(String), // an user-defined type (ex. struct or enum)
}

pub enum NodeValue {
    Int(i64),
    UInt(u64),
    Float(f64),
    Str(String),
}

pub struct NodeVariableCall {
    r#type: VariableType,
    value: Box<NodeAST>,
}

pub struct NodeVariableDefinition {
    r#type: VariableType,
    name: String,
    value: Option<Box<NodeValue>>,
}

pub struct NodeFunctionDefinition {
    name: String,
    args: Vec<(String, VariableType)>,
    return_type: VariableType,
    body: Vec<Box<NodeAST>>,
}

pub struct NodeFunctionDeclaration {
    name: String,
    args: Vec<(String, VariableType)>,
    return_type: VariableType,
}

pub struct NodeFunctionCall {
    name: String,
    args: Vec<NodeAST>,
}

pub struct NodeBinaryExpression {
    left: Box<NodeAST>,
    right: Box<NodeAST>,
    operator: BinaryOperatorType,
}

pub struct NodeUnaryExpression {
    operand: Box<NodeAST>,
    Operator: UnaryOperatorType,
}

pub enum NodeExpression {
    BinaryExpression(NodeBinaryExpression),
    UnaryExpression(NodeUnaryExpression),
}

pub struct NodeIfStatement {
    condition: Box<NodeExpression>,
    body: Vec<Box<NodeAST>>,
}

pub struct NodeWhileLoop {
    condition: Box<NodeExpression>,
    body: Vec<Box<NodeAST>>,
}

/*
pub struct NodeForLoop {

}
*/

pub enum NodeAST {
    Value(NodeValue), // might not be needed
    VariableCall(NodeVariableCall),
    VariableDefinition(NodeVariableDefinition),
    FunctionDefinition(NodeFunctionDefinition),
    FunctionDeclaration(NodeFunctionDeclaration),
    FunctionCall(NodeFunctionCall),
    Expression(NodeExpression),
    IfStatement(NodeIfStatement),
    WhileLoop(NodeWhileLoop),
    Error(String),
//    ForLoop(NodeForLoop),
}
