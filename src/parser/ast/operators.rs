#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOprType {
    Neg,  // - (negative)
    Bang, // !
}

#[derive(Debug, PartialEq)]
pub enum AssignOprType {
    Eq,    // =
    AddEq, // +=
    SubEq, // -=
    MulEq, // *=
    DivEq, // /=
    ModEq, // %=
}
