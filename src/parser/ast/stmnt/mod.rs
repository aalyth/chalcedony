mod assignment;
mod if_stmnt;
mod return_stmnt;
mod while_loop;

use assignment::NodeAssign;
use if_stmnt::NodeIfStmnt;
use return_stmnt::NodeRetStmnt;
use while_loop::NodeWhileLoop;

use super::NodeVarDef;

pub enum NodeStmnt {
    VarDef(NodeVarDef),
    Assign(NodeAssign),
    IfStmnt(NodeIfStmnt),
    WhileLoop(NodeWhileLoop),
    RetStmnt(NodeRetStmnt),
}
