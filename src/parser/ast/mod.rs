mod operators;

use operators::{BinaryOperatorType, UnaryOperatorType};

pub enum VarType {
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
    Custom(String), // an user-defined type (ex. struct or enum)
}

pub enum NodeValue {
    Int(i64),
    UInt(u64),
    Float(f64),
    Str(String),
}

pub struct NodeVariableCall {
    r#type: VarType,
    value: Box<NodeAST>,
}

pub struct NodeVariableDefinition {
    r#type: VarType,
    name: String,
    value: Option<Box<NodeValue>>,
}

pub struct NodeFunctionDefinition {
    name: String,
    args: Vec<(String, VarType)>,
    return_type: VarType,
    body: Vec<Box<NodeAST>>,
}

/*
pub struct NodeFunctionDeclaration {
    name: String,
    args: Vec<(String, VarType)>,
    return_type: VarType,
}
*/

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

pub struct NodeVarDef {
    r#type: VarType,
    name: String, // might not need
}

pub struct NodeVarDeclr {
    r#type: VarType,
    name: String, // might not need
    value: Box<NodeAST>,
}

pub struct NodeFuncDef {
    name: String, // might not need
    arg_types: Vec<VarType>,
}

pub struct NodeFuncDeclr {
    name: String, // might not need
    arg_types: Vec<VarType>,
    arg_names: Vec<String>,
    body: Vec<NodeStatement>, // list of statements
}

pub enum NodeStatement {
   Expression(NodeExpression),
   IfStatement(NodeIfStatement),
   WhileLoop(NodeWhileLoop),
   ReturnStatement(NodeReturnStatement),
}

pub struct NodeExpression {
    
}

pub struct NodeIfStatement {
    condition: NodeExpression,
    body: Vec<NodeStatement>
}

pub struct NodeWhileLoop {
    condition: NodeExpression,
    body: Vec<NodeStatement>
}

pub struct ReturnStatement {
    value: NodeExpression,
}

pub enum NodeAST {
   VarDef(NodeVarDef), 
   VarDeclr(NodeVarDeclr),
   FuncDef(NodeFuncDef),
   FuncDeclr(NodeFuncDeclr),
   Statement(NodeStatement), 
    
   /*
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
    */
}
