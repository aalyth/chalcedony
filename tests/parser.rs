use chalcedony::common::operators::{BinOprType, UnaryOprType};
use chalcedony::common::Type;

use chalcedony::lexer::{Delimiter, Keyword, Operator, Special, TokenKind};
use chalcedony::parser::ast::{
    func::Arg, NodeBreakStmnt, NodeContStmnt, NodeElifStmnt, NodeElseStmnt, NodeExpr,
    NodeExprInner, NodeFuncCall, NodeFuncDef, NodeIfBranch, NodeIfStmnt, NodeValue, NodeVarDef,
    NodeWhileLoop,
};
use chalcedony::parser::ast::{NodeRetStmnt, NodeStmnt, NodeVarCall};

use chalcedony::mocks::{line, line_reader, token_reader, vecdeq, SpanMock};

#[test]
fn parse_var_def() {
    // equivalent to the code:
    // ```
    // let a: uint = (-fib(10)) + 3
    // ```
    let tokens = token_reader!(
        TokenKind::Keyword(Keyword::Let),
        TokenKind::Identifier("a".to_string()),
        TokenKind::Special(Special::Colon),
        TokenKind::Type(Type::Uint),
        TokenKind::Operator(Operator::Eq),
        TokenKind::Delimiter(Delimiter::OpenPar),
        TokenKind::Operator(Operator::Neg),
        TokenKind::Identifier("fib".to_string()),
        TokenKind::Delimiter(Delimiter::OpenPar),
        TokenKind::Uint(10),
        TokenKind::Delimiter(Delimiter::ClosePar),
        TokenKind::Delimiter(Delimiter::ClosePar),
        TokenKind::Operator(Operator::Add),
        TokenKind::Uint(3)
    );

    let recv = NodeVarDef::new(tokens).expect("did not parse NodeVarDef");

    let exp = NodeVarDef {
        ty: Type::Uint,
        name: "a".to_string(),
        value: NodeExpr {
            expr: vecdeq![
                NodeExprInner::FuncCall(NodeFuncCall {
                    name: "fib".to_string(),
                    args: vec![NodeExpr {
                        expr: vecdeq!(NodeExprInner::Value(NodeValue::Uint(10))),
                        span: SpanMock::new()
                    }],
                    span: SpanMock::new(),
                }),
                NodeExprInner::UnaryOpr(UnaryOprType::Neg),
                NodeExprInner::Value(NodeValue::Uint(3)),
                NodeExprInner::BinOpr(BinOprType::Add)
            ],
            span: SpanMock::new(),
        },
        span: SpanMock::new(),
    };

    assert_eq!(exp, recv);
}

#[test]
fn parse_func_def() {
    // equivalent to the code:
    // ```
    // fn fib(n: int) -> uint:
    //     if n > 2:
    //         return fib(n-2) + fib(n-1)
    //     return 1
    // ```
    let code = line_reader!(
        line!(
            0,
            TokenKind::Keyword(Keyword::Fn),
            TokenKind::Identifier("fib".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Identifier("n".to_string()),
            TokenKind::Special(Special::Colon),
            TokenKind::Type(Type::Int),
            TokenKind::Delimiter(Delimiter::ClosePar),
            TokenKind::Special(Special::RightArrow),
            TokenKind::Type(Type::Uint),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            4,
            TokenKind::Keyword(Keyword::If),
            TokenKind::Identifier("n".to_string()),
            TokenKind::Operator(Operator::Gt),
            TokenKind::Uint(2),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            8,
            TokenKind::Keyword(Keyword::Return),
            TokenKind::Identifier("fib".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Identifier("n".to_string()),
            TokenKind::Operator(Operator::Sub),
            TokenKind::Uint(2),
            TokenKind::Delimiter(Delimiter::ClosePar),
            TokenKind::Operator(Operator::Add),
            TokenKind::Identifier("fib".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Identifier("n".to_string()),
            TokenKind::Operator(Operator::Sub),
            TokenKind::Uint(1),
            TokenKind::Delimiter(Delimiter::ClosePar)
        ),
        line!(4, TokenKind::Keyword(Keyword::Return), TokenKind::Uint(1))
    );

    let recv = NodeFuncDef::new(code).expect("did not parse NodeFuncDef");

    let exp = NodeFuncDef {
        name: "fib".to_string(),
        args: vec![Arg {
            name: "n".to_string(),
            ty: Type::Int,
        }],
        ret_type: Type::Uint,
        body: vec![
            /* if n > 2: */
            NodeStmnt::IfStmnt(NodeIfStmnt {
                condition: NodeExpr {
                    expr: vecdeq![
                        NodeExprInner::VarCall(NodeVarCall {
                            name: "n".to_string(),
                            span: SpanMock::new()
                        }),
                        NodeExprInner::Value(NodeValue::Uint(2)),
                        NodeExprInner::BinOpr(BinOprType::Gt)
                    ],
                    span: SpanMock::new(),
                },
                /* return fib(n-2) + fib(n-1) */
                body: vec![NodeStmnt::RetStmnt(NodeRetStmnt {
                    value: NodeExpr {
                        expr: vecdeq![
                            NodeExprInner::FuncCall(NodeFuncCall {
                                name: "fib".to_string(),
                                args: vec![NodeExpr {
                                    expr: vecdeq![
                                        NodeExprInner::VarCall(NodeVarCall {
                                            name: "n".to_string(),
                                            span: SpanMock::new(),
                                        }),
                                        NodeExprInner::Value(NodeValue::Uint(2)),
                                        NodeExprInner::BinOpr(BinOprType::Sub)
                                    ],
                                    span: SpanMock::new()
                                }],
                                span: SpanMock::new(),
                            }),
                            NodeExprInner::FuncCall(NodeFuncCall {
                                name: "fib".to_string(),
                                args: vec![NodeExpr {
                                    expr: vecdeq![
                                        NodeExprInner::VarCall(NodeVarCall {
                                            name: "n".to_string(),
                                            span: SpanMock::new(),
                                        }),
                                        NodeExprInner::Value(NodeValue::Uint(1)),
                                        NodeExprInner::BinOpr(BinOprType::Sub)
                                    ],
                                    span: SpanMock::new()
                                }],
                                span: SpanMock::new(),
                            }),
                            NodeExprInner::BinOpr(BinOprType::Add)
                        ],
                        span: SpanMock::new(),
                    },
                    span: SpanMock::new(),
                })],
                branches: vec![],
            }),
            /* return 1 */
            NodeStmnt::RetStmnt(NodeRetStmnt {
                value: NodeExpr {
                    expr: vecdeq![NodeExprInner::Value(NodeValue::Uint(1))],
                    span: SpanMock::new(),
                },
                span: SpanMock::new(),
            }),
        ],
        span: SpanMock::new(),
    };

    assert_eq!(exp, recv);
}

#[test]
fn parse_if_statement() {
    // equivalent to the code:
    // ```
    // if 2 > 3:
    //     print('one')
    // elif 3 > 4:
    //     print('two')
    // else:
    //     print('default')
    // ```
    let code = line_reader!(
        line!(
            0,
            TokenKind::Keyword(Keyword::If),
            TokenKind::Uint(2),
            TokenKind::Operator(Operator::Gt),
            TokenKind::Uint(3),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            4,
            TokenKind::Identifier("print".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Str("one".to_string()),
            TokenKind::Delimiter(Delimiter::ClosePar)
        ),
        line!(
            0,
            TokenKind::Keyword(Keyword::Elif),
            TokenKind::Uint(3),
            TokenKind::Operator(Operator::Gt),
            TokenKind::Uint(4),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            4,
            TokenKind::Identifier("print".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Str("two".to_string()),
            TokenKind::Delimiter(Delimiter::ClosePar)
        ),
        line!(
            0,
            TokenKind::Keyword(Keyword::Else),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            4,
            TokenKind::Identifier("print".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Str("default".to_string()),
            TokenKind::Delimiter(Delimiter::ClosePar)
        )
    );

    let recv = NodeIfStmnt::new(code).expect("did not parse NodeIfStmnt");

    let exp = NodeIfStmnt {
        condition: NodeExpr {
            expr: vecdeq![
                NodeExprInner::Value(NodeValue::Uint(2)),
                NodeExprInner::Value(NodeValue::Uint(3)),
                NodeExprInner::BinOpr(BinOprType::Gt)
            ],
            span: SpanMock::new(),
        },
        body: vec![NodeStmnt::FuncCall(NodeFuncCall {
            name: "print".to_string(),
            args: vec![NodeExpr {
                expr: vecdeq![NodeExprInner::Value(NodeValue::Str("one".to_string()))],
                span: SpanMock::new(),
            }],
            span: SpanMock::new(),
        })],
        branches: vec![
            NodeIfBranch::Elif(NodeElifStmnt {
                condition: NodeExpr {
                    expr: vecdeq![
                        NodeExprInner::Value(NodeValue::Uint(3)),
                        NodeExprInner::Value(NodeValue::Uint(4)),
                        NodeExprInner::BinOpr(BinOprType::Gt)
                    ],
                    span: SpanMock::new(),
                },
                body: vec![NodeStmnt::FuncCall(NodeFuncCall {
                    name: "print".to_string(),
                    args: vec![NodeExpr {
                        expr: vecdeq![NodeExprInner::Value(NodeValue::Str("two".to_string()))],
                        span: SpanMock::new(),
                    }],
                    span: SpanMock::new(),
                })],
            }),
            NodeIfBranch::Else(NodeElseStmnt {
                body: vec![NodeStmnt::FuncCall(NodeFuncCall {
                    name: "print".to_string(),
                    args: vec![NodeExpr {
                        expr: vecdeq![NodeExprInner::Value(NodeValue::Str("default".to_string()))],
                        span: SpanMock::new(),
                    }],
                    span: SpanMock::new(),
                })],
            }),
        ],
    };

    assert_eq!(exp, recv);
}

#[test]
fn parse_while_statement() {
    // equivalent to the code:
    // ```
    // while !(2 < 3):
    //     print("something's wrong")
    //     break
    //     continue
    // ```
    let code = line_reader!(
        line!(
            0,
            TokenKind::Keyword(Keyword::While),
            TokenKind::Operator(Operator::Bang),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Uint(2),
            TokenKind::Operator(Operator::Lt),
            TokenKind::Uint(3),
            TokenKind::Delimiter(Delimiter::ClosePar),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            4,
            TokenKind::Identifier("print".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Str("something's wrong".to_string()),
            TokenKind::Delimiter(Delimiter::ClosePar)
        ),
        line!(4, TokenKind::Keyword(Keyword::Break)),
        line!(4, TokenKind::Keyword(Keyword::Continue))
    );

    let recv = NodeWhileLoop::new(code).expect("did not parse NodeWhileLoop");

    let exp = NodeWhileLoop {
        condition: NodeExpr {
            expr: vecdeq![
                NodeExprInner::Value(NodeValue::Uint(2)),
                NodeExprInner::Value(NodeValue::Uint(3)),
                NodeExprInner::BinOpr(BinOprType::Lt),
                NodeExprInner::UnaryOpr(UnaryOprType::Bang)
            ],
            span: SpanMock::new(),
        },
        body: vec![
            NodeStmnt::FuncCall(NodeFuncCall {
                name: "print".to_string(),
                args: vec![NodeExpr {
                    expr: vecdeq![NodeExprInner::Value(NodeValue::Str(
                        "something's wrong".to_string()
                    ))],
                    span: SpanMock::new(),
                }],
                span: SpanMock::new(),
            }),
            NodeStmnt::BreakStmnt(NodeBreakStmnt {
                span: SpanMock::new(),
            }),
            NodeStmnt::ContStmnt(NodeContStmnt {
                span: SpanMock::new(),
            }),
        ],
    };

    assert_eq!(exp, recv);
}
