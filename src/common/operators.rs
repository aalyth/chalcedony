#[derive(Debug, Clone, Copy)]
pub enum BinOprType {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %

    And, // &&
    Or,  // ||

    Lt,     // <
    Gt,     // >
    LtEq,   // <=
    GtEq,   // >=
    EqEq,   // ==
    BangEq, // !=
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOprType {
    Neg,  // - (negative)
    Bang, // !
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AssignOprType {
    Eq,    // =
    AddEq, // +=
    SubEq, // -=
    MulEq, // *=
    DivEq, // /=
    ModEq, // %=
}
