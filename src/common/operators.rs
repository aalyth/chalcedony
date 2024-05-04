#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOprType {
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,

    /// &&
    And,
    /// ||
    Or,

    /// <
    Lt,
    /// >
    Gt,
    /// <=
    LtEq,
    /// >=
    GtEq,
    /// ==
    EqEq,
    /// !=
    BangEq,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOprType {
    /// - (negative)
    Neg,
    /// !
    Bang,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssignOprType {
    /// =
    Eq,
    /// +=
    AddEq,
    /// -=
    SubEq,
    /// *=
    MulEq,
    /// /=
    DivEq,
    /// %=
    ModEq,
}
