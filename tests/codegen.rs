use chalcedony::common::operators::{AssignOprType, BinOprType};
use chalcedony::common::{Bytecode, Type};

use chalcedony::parser::ast::{
    func::Arg, NodeBreakStmnt, NodeContStmnt, NodeElifStmnt, NodeElseStmnt, NodeExpr,
    NodeExprInner, NodeFuncCall, NodeFuncDef, NodeIfBranch, NodeIfStmnt, NodeThrow, NodeTryCatch,
    NodeValue, NodeVarDef, NodeWhileLoop,
};
use chalcedony::parser::ast::{NodeAssign, NodeRetStmnt, NodeStmnt, NodeVarCall};

use chalcedony::interpreter::{Chalcedony, ToBytecode};

use chalcedony::mocks::{vecdeq, SpanMock};

/// This at first glance random test is an actual corner case that was
/// encountered during the implementation of the control flow statments `break`
/// and `continue.
#[test]
fn compile_control_flow() {
    // equivalent to the code:
    // ```
    // fn ctrl_flow():
    //     let i = 0
    //     while i < 100:
    //         i += 1
    //         if i == 42:
    //             break
    //         elif i % 2 == 0:
    //             continue
    //         let j = 0
    //         while j < 10:
    //             j += 1
    //             if j % 2 == 1:
    //                 continue
    // ```

    let code = NodeFuncDef {
        name: "ctrl_flow".to_string(),
        args: vec![],
        ret_type: Type::Void,
        body: vec![
            // let i = 0
            NodeStmnt::VarDef(NodeVarDef {
                name: "i".to_string(),
                ty: Type::Any,
                value: NodeExpr {
                    expr: vecdeq![NodeExprInner::Value(NodeValue::Uint(0))],
                    span: SpanMock::new(),
                },
                span: SpanMock::new(),
            }),
            // while i < 100:
            NodeStmnt::WhileLoop(NodeWhileLoop {
                condition: NodeExpr {
                    expr: vecdeq![
                        NodeExprInner::VarCall(NodeVarCall {
                            name: "i".to_string(),
                            span: SpanMock::new()
                        }),
                        NodeExprInner::Value(NodeValue::Uint(100)),
                        NodeExprInner::BinOpr(BinOprType::Lt)
                    ],
                    span: SpanMock::new(),
                },
                body: vec![
                    // i += 1
                    NodeStmnt::Assign(NodeAssign {
                        lhs: NodeVarCall {
                            name: "i".to_string(),
                            span: SpanMock::new(),
                        },
                        opr: AssignOprType::AddEq,
                        rhs: NodeExpr {
                            expr: vecdeq![NodeExprInner::Value(NodeValue::Uint(1))],
                            span: SpanMock::new(),
                        },
                    }),
                    // if i == 42:
                    NodeStmnt::IfStmnt(NodeIfStmnt {
                        condition: NodeExpr {
                            expr: vecdeq![
                                NodeExprInner::VarCall(NodeVarCall {
                                    name: "i".to_string(),
                                    span: SpanMock::new(),
                                }),
                                NodeExprInner::Value(NodeValue::Uint(42)),
                                NodeExprInner::BinOpr(BinOprType::EqEq)
                            ],
                            span: SpanMock::new(),
                        },
                        // break
                        body: vec![NodeStmnt::BreakStmnt(NodeBreakStmnt {
                            span: SpanMock::new(),
                        })],
                        // elif i % 2 == 0:
                        branches: vec![NodeIfBranch::Elif(NodeElifStmnt {
                            condition: NodeExpr {
                                expr: vecdeq![
                                    NodeExprInner::VarCall(NodeVarCall {
                                        name: "i".to_string(),
                                        span: SpanMock::new(),
                                    }),
                                    NodeExprInner::Value(NodeValue::Uint(2)),
                                    NodeExprInner::BinOpr(BinOprType::Mod),
                                    NodeExprInner::Value(NodeValue::Uint(0)),
                                    NodeExprInner::BinOpr(BinOprType::EqEq)
                                ],
                                span: SpanMock::new(),
                            },
                            // continue
                            body: vec![NodeStmnt::ContStmnt(NodeContStmnt {
                                span: SpanMock::new(),
                            })],
                        })],
                    }),
                    // let j = 0
                    NodeStmnt::VarDef(NodeVarDef {
                        ty: Type::Any,
                        name: "j".to_string(),
                        value: NodeExpr {
                            expr: vecdeq![NodeExprInner::Value(NodeValue::Uint(0))],
                            span: SpanMock::new(),
                        },
                        span: SpanMock::new(),
                    }),
                    // while j < 10:
                    NodeStmnt::WhileLoop(NodeWhileLoop {
                        condition: NodeExpr {
                            expr: vecdeq![
                                NodeExprInner::VarCall(NodeVarCall {
                                    name: "j".to_string(),
                                    span: SpanMock::new(),
                                }),
                                NodeExprInner::Value(NodeValue::Uint(10)),
                                NodeExprInner::BinOpr(BinOprType::Lt)
                            ],
                            span: SpanMock::new(),
                        },
                        body: vec![
                            // j += 1
                            NodeStmnt::Assign(NodeAssign {
                                lhs: NodeVarCall {
                                    name: "j".to_string(),
                                    span: SpanMock::new(),
                                },
                                opr: AssignOprType::AddEq,
                                rhs: NodeExpr {
                                    expr: vecdeq![NodeExprInner::Value(NodeValue::Uint(1))],
                                    span: SpanMock::new(),
                                },
                            }),
                            // if j % 2 == 1:
                            NodeStmnt::IfStmnt(NodeIfStmnt {
                                condition: NodeExpr {
                                    expr: vecdeq![
                                        NodeExprInner::VarCall(NodeVarCall {
                                            name: "j".to_string(),
                                            span: SpanMock::new(),
                                        }),
                                        NodeExprInner::Value(NodeValue::Uint(2)),
                                        NodeExprInner::BinOpr(BinOprType::Mod),
                                        NodeExprInner::Value(NodeValue::Uint(1)),
                                        NodeExprInner::BinOpr(BinOprType::EqEq)
                                    ],
                                    span: SpanMock::new(),
                                },
                                // continue
                                body: vec![NodeStmnt::ContStmnt(NodeContStmnt {
                                    span: SpanMock::new(),
                                })],
                                branches: vec![],
                            }),
                        ],
                    }),
                ],
            }),
        ],
        span: SpanMock::new(),
    };

    let exp = vec![
        // fn ctrl_flow():
        Bytecode::CreateFunc(0),
        // let i = 0
        Bytecode::ConstU(0),
        Bytecode::SetLocal(0),
        // while i < 100:
        Bytecode::GetLocal(0),
        Bytecode::ConstU(100),
        Bytecode::Lt,
        Bytecode::If(38),
        // i += 1
        Bytecode::GetLocal(0),
        Bytecode::ConstU(1),
        Bytecode::Add,
        Bytecode::SetLocal(0),
        // if i == 42:
        Bytecode::GetLocal(0),
        Bytecode::ConstU(42),
        Bytecode::Eq,
        Bytecode::If(2),
        // break
        Bytecode::Jmp(29),
        Bytecode::Jmp(8),
        // elif i % 2 == 0:
        Bytecode::GetLocal(0),
        Bytecode::ConstU(2),
        Bytecode::Mod,
        Bytecode::ConstU(0),
        Bytecode::Eq,
        Bytecode::If(2),
        // continue
        Bytecode::Jmp(-21),
        Bytecode::Nop,
        // let j = 0
        Bytecode::ConstU(0),
        Bytecode::SetLocal(1),
        // while j < 10:
        Bytecode::GetLocal(1),
        Bytecode::ConstU(10),
        Bytecode::Lt,
        Bytecode::If(13),
        // j += 1
        Bytecode::GetLocal(1),
        Bytecode::ConstU(1),
        Bytecode::Add,
        Bytecode::SetLocal(1),
        // if j % 2 == 1
        Bytecode::GetLocal(1),
        Bytecode::ConstU(2),
        Bytecode::Mod,
        Bytecode::ConstU(1),
        Bytecode::Eq,
        Bytecode::If(2),
        // continue
        Bytecode::Jmp(-15),
        Bytecode::Nop,
        /* go back to the second loop */
        Bytecode::Jmp(-17),
        /* go back to the first loop */
        Bytecode::Jmp(-42),
        /* implicit return void */
        Bytecode::ReturnVoid,
    ];

    let recv = code
        .to_bytecode(&mut Chalcedony::new())
        .expect("did not compile function");

    assert_eq!(exp, recv);
}

/// This is also an encountered corner case during the implementation of the
/// `if-elif-else` statment.
#[test]
fn compile_if_branching() {
    // equivalent to the code:
    // ```
    // if 2 > 3:
    //     print("one")
    //  elif 3 > 4:
    //     print("two")
    //  else:
    //     print("default")
    // ```

    let code = NodeIfStmnt {
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

    let exp = vec![
        // if 2 > 3:
        Bytecode::ConstU(2),
        Bytecode::ConstU(3),
        Bytecode::Gt,
        Bytecode::If(3),
        // print("one")
        Bytecode::ConstS("one".to_string().into()),
        Bytecode::CallFunc(0), /* print is a builtin function with id 0 */
        Bytecode::Jmp(10),
        // elif 3 > 4:
        Bytecode::ConstU(3),
        Bytecode::ConstU(4),
        Bytecode::Gt,
        Bytecode::If(3),
        // print("two")
        Bytecode::ConstS("two".to_string().into()),
        Bytecode::CallFunc(0), /* print */
        Bytecode::Jmp(3),
        // else:
        //     print("default")
        Bytecode::ConstS("default".to_string().into()),
        Bytecode::CallFunc(0), /* print*/
        Bytecode::Nop,
    ];

    let recv = code
        .to_bytecode(&mut Chalcedony::new())
        .expect("did not compile");

    assert_eq!(exp, recv);
}

#[test]
fn compile_function() {
    // equivalent to the code:
    // ```
    // fn fib(n: int) -> uint:
    //     if n > 2:
    //         return fib(n-2) + fib(n-1)
    //      return 1
    // ```

    let code = NodeFuncDef {
        name: "fib".to_string(),
        args: vec![Arg {
            name: "n".to_string(),
            ty: Type::Int,
        }],
        ret_type: Type::Uint,
        body: vec![
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
                                span: SpanMock::new()
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

    // the reason the function's id is gotten from the interpreter is due to the
    // potential changes in the interpreter's standard library functions
    let mut interpreter = Chalcedony::new();
    let fib_id = interpreter.get_next_func_id();

    let exp = vec![
        // fn fib(n: int) -> uint:
        Bytecode::CreateFunc(1),
        // if n > 2:
        Bytecode::GetArg(0),
        Bytecode::ConstU(2),
        Bytecode::Gt,
        Bytecode::If(11),
        // return fib(n-2) + fib(n-1)
        Bytecode::GetArg(0),
        Bytecode::ConstU(2),
        Bytecode::Sub,
        Bytecode::CallFunc(fib_id),
        Bytecode::GetArg(0),
        Bytecode::ConstU(1),
        Bytecode::Sub,
        Bytecode::CallFunc(fib_id),
        Bytecode::Add,
        Bytecode::Return,
        Bytecode::Nop,
        // return 1
        Bytecode::ConstU(1),
        Bytecode::Return,
    ];

    let recv = code
        .to_bytecode(&mut interpreter)
        .expect("did not compile properly");

    assert_eq!(exp, recv);
}

#[test]
fn compile_try_catch() {
    // equivalent to the code:
    // ```
    // try:
    //     print(21 * 2)
    //     throw 'unexpected error'
    // catch (exc: exception):
    //     print('Received the exception: ' + exc)
    // ```

    let code = NodeTryCatch {
        try_body: vec![
            NodeStmnt::FuncCall(NodeFuncCall {
                name: "print".to_string(),
                args: vec![NodeExpr {
                    expr: vecdeq![
                        NodeExprInner::Value(NodeValue::Uint(21)),
                        NodeExprInner::Value(NodeValue::Uint(2)),
                        NodeExprInner::BinOpr(BinOprType::Mul)
                    ],
                    span: SpanMock::new(),
                }],
                span: SpanMock::new(),
            }),
            NodeStmnt::Throw(NodeThrow(NodeExpr {
                expr: vecdeq![NodeExprInner::Value(NodeValue::Str(
                    "unexpected error".to_string()
                ))],
                span: SpanMock::new(),
            })),
        ],
        try_span: SpanMock::new(),
        exception_var: NodeVarCall {
            name: "exc".to_string(),
            span: SpanMock::new(),
        },
        catch_body: vec![NodeStmnt::FuncCall(NodeFuncCall {
            name: "print".to_string(),
            args: vec![NodeExpr {
                expr: vecdeq![
                    NodeExprInner::Value(NodeValue::Str("Received the exception: ".to_string())),
                    NodeExprInner::VarCall(NodeVarCall {
                        name: "exc".to_string(),
                        span: SpanMock::new()
                    }),
                    NodeExprInner::BinOpr(BinOprType::Add)
                ],
                span: SpanMock::new(),
            }],
            span: SpanMock::new(),
        })],
    };

    let exp = vec![
        // try:
        Bytecode::TryScope(7),
        // print(21 * 2)
        Bytecode::ConstU(21),
        Bytecode::ConstU(2),
        Bytecode::Mul,
        Bytecode::CallFunc(0),
        // throw "unexpected error"
        Bytecode::ConstS("unexpected error".to_string().into()),
        Bytecode::ThrowException,
        Bytecode::CatchJmp(5),
        // catch (exc: exception):
        //     print("Received the exception" + exc)
        Bytecode::SetLocal(0),
        Bytecode::ConstS("Received the exception: ".to_string().into()),
        Bytecode::GetLocal(0),
        Bytecode::Add,
        Bytecode::CallFunc(0),
    ];

    let recv = code
        .to_bytecode(&mut Chalcedony::new())
        .expect("did not compile properly");

    assert_eq!(exp, recv);
}
