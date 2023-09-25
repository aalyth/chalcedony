use crate::error::ChalError;
use crate::lexer::{Type, Line};

use std::collections::VecDeque;

pub struct NodeFuncDef {
    name:     String,
    args:     Vec<(String, Type)>,
    ret_type: Type,
    // body:     Vec<NodeStmnt>
}

impl NodeFuncDef {
    pub fn new(chunk: VecDeque<Line>) -> Result<Self, ChalError> {

    }
}
