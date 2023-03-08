
pub enum BinaryOperatorType {
    Add,          // +
    Sub,          // -
    Mul,          // *
    Div,          // /
    Mod,          // %
    Eq,           // =
    Lt,           // <
    Gt,           // >

    BinAnd,       // &
    BinOr,        // |
    Xor,          // ^
    And,          // &&
    Or,           // ||

    AddEq,        // +=
    SubEq,        // -=
    MulEq,        // *=
    DivEq,        // /=
    ModEq,        // %=
    EqEq,         // ==
    LtEq,         // <=
    GtEq,         // >=
    BangEq,       // !=
}

pub enum UnaryOperatorType {
    Sub,          // -
    Ref,          // &
    DeRef,        // *
    Bang,         // !
    Tilde,        // ~
}
