
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

pub enum UnaryCondType {
    Bang,         // !
}

pub enum UnaryOprType {
    Neg,          // - (negative)
    Ref,          // &

    /* not needed for now
    DeRef,        // *
    Tilde,        // ~
    */
}
