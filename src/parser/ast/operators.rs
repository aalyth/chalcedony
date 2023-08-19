
#[derive(Debug)]
pub enum BinOprType {
    Add,          // +
    Sub,          // -
    Mul,          // *
    Div,          // /
    Mod,          // %
    Eq,           // =

    AddEq,        // +=
    SubEq,        // -=
    MulEq,        // *=
    DivEq,        // /=
    ModEq,        // %=

    /* not needed for now
    BinAnd,       // &
    BinOr,        // |
    Xor,          // ^
    */
}

#[derive(Debug)]
pub enum BinCondType {
    Lt,           // <
    Gt,           // >
    EqEq,         // ==
    LtEq,         // <=
    GtEq,         // >=
    BangEq,       // !=
    And,          // &&
    Or,           // ||
}

#[derive(Debug)]
pub enum UnaryCondType {
    Bang,         // !
}

#[derive(Debug)]
pub enum UnaryOprType {
    Neg,          // - (negative)
    Ref,          // &

    /* not needed for now
    DeRef,        // *
    Tilde,        // ~
    */
}
