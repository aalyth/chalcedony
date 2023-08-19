
pub enum AssignOpr {
    Eq,      // =
    AddEq,   // +=
    SubEq,   // -=
    MulEq,   // *=
    DivEq,   // /=
    ModEq,   // %=
}

pub struct NodeAssign {
    varname: String,
    operator: AssignOpr,
    rhs: Box<NodeExpr>,
}
