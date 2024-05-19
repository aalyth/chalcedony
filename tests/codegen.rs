use chalcedony::common::operators::{AssignOprType, BinOprType};
use chalcedony::common::{Bytecode, Type};

use chalcedony::parser::ast::{
    class::Member, func::Arg, NodeAssign, NodeAttrRes, NodeAttribute, NodeBreakStmnt, NodeClass,
    NodeContStmnt, NodeElifStmnt, NodeElseStmnt, NodeExpr, NodeExprInner, NodeFuncCall,
    NodeFuncCallStmnt, NodeFuncDef, NodeIfBranch, NodeIfStmnt, NodeInlineClass, NodeRetStmnt,
    NodeStmnt, NodeThrow, NodeTryCatch, NodeValue, NodeVarCall, NodeVarDef, NodeWhileLoop,
};

use chalcedony::interpreter::{
    ArgAnnotation, Chalcedony, ClassNamespace, FuncAnnotation, MemberAnnotation, ToBytecode,
};

use chalcedony::mocks::{hash_map, vecdeq, SpanMock};

use std::rc::Rc;

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
        args: vecdeq![],
        ret_type: Type::Void,
        namespace: None,
        body: vec![
            // let i = 0
            NodeStmnt::VarDef(NodeVarDef {
                name: "i".to_string(),
                ty: Type::Any,
                value: NodeExpr {
                    expr: vecdeq![NodeExprInner::Value(NodeValue::Uint(0))],
                    span: SpanMock::new(),
                },
                is_const: false,
                span: SpanMock::new(),
            }),
            // while i < 100:
            NodeStmnt::WhileLoop(NodeWhileLoop {
                condition: NodeExpr {
                    expr: vecdeq![
                        NodeExprInner::Resolution(NodeAttrRes {
                            resolution: vec![NodeAttribute::VarCall(NodeVarCall {
                                name: "i".to_string(),
                                span: SpanMock::new()
                            })],
                            span: SpanMock::new(),
                        }),
                        NodeExprInner::Value(NodeValue::Uint(100)),
                        NodeExprInner::BinOpr(BinOprType::Lt)
                    ],
                    span: SpanMock::new(),
                },
                body: vec![
                    // i += 1
                    NodeStmnt::Assign(NodeAssign {
                        lhs: NodeAttrRes {
                            resolution: vec![NodeAttribute::VarCall(NodeVarCall {
                                name: "i".to_string(),
                                span: SpanMock::new(),
                            })],
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
                                NodeExprInner::Resolution(NodeAttrRes {
                                    resolution: vec![NodeAttribute::VarCall(NodeVarCall {
                                        name: "i".to_string(),
                                        span: SpanMock::new(),
                                    })],
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
                                    NodeExprInner::Resolution(NodeAttrRes {
                                        resolution: vec![NodeAttribute::VarCall(NodeVarCall {
                                            name: "i".to_string(),
                                            span: SpanMock::new(),
                                        })],
                                        span: SpanMock::new()
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
                        is_const: false,
                        span: SpanMock::new(),
                    }),
                    // while j < 10:
                    NodeStmnt::WhileLoop(NodeWhileLoop {
                        condition: NodeExpr {
                            expr: vecdeq![
                                NodeExprInner::Resolution(NodeAttrRes {
                                    resolution: vec![NodeAttribute::VarCall(NodeVarCall {
                                        name: "j".to_string(),
                                        span: SpanMock::new(),
                                    })],
                                    span: SpanMock::new()
                                }),
                                NodeExprInner::Value(NodeValue::Uint(10)),
                                NodeExprInner::BinOpr(BinOprType::Lt)
                            ],
                            span: SpanMock::new(),
                        },
                        body: vec![
                            // j += 1
                            NodeStmnt::Assign(NodeAssign {
                                lhs: NodeAttrRes {
                                    resolution: vec![NodeAttribute::VarCall(NodeVarCall {
                                        name: "j".to_string(),
                                        span: SpanMock::new(),
                                    })],
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
                                        NodeExprInner::Resolution(NodeAttrRes {
                                            resolution: vec![NodeAttribute::VarCall(NodeVarCall {
                                                name: "j".to_string(),
                                                span: SpanMock::new(),
                                            })],
                                            span: SpanMock::new()
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
        body: vec![NodeStmnt::FuncCall(NodeFuncCallStmnt(NodeAttrRes {
            resolution: vec![NodeAttribute::FuncCall(NodeFuncCall {
                name: "print".to_string(),
                namespace: None,
                args: vec![NodeExpr {
                    expr: vecdeq![NodeExprInner::Value(NodeValue::Str("one".to_string()))],
                    span: SpanMock::new(),
                }],
                span: SpanMock::new(),
            })],
            span: SpanMock::new(),
        }))],
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
                body: vec![NodeStmnt::FuncCall(NodeFuncCallStmnt(NodeAttrRes {
                    resolution: vec![NodeAttribute::FuncCall(NodeFuncCall {
                        name: "print".to_string(),
                        namespace: None,
                        args: vec![NodeExpr {
                            expr: vecdeq![NodeExprInner::Value(NodeValue::Str("two".to_string()))],
                            span: SpanMock::new(),
                        }],
                        span: SpanMock::new(),
                    })],
                    span: SpanMock::new(),
                }))],
            }),
            NodeIfBranch::Else(NodeElseStmnt {
                body: vec![NodeStmnt::FuncCall(NodeFuncCallStmnt(NodeAttrRes {
                    resolution: vec![NodeAttribute::FuncCall(NodeFuncCall {
                        name: "print".to_string(),
                        namespace: None,
                        args: vec![NodeExpr {
                            expr: vecdeq![NodeExprInner::Value(NodeValue::Str(
                                "default".to_string()
                            ))],
                            span: SpanMock::new(),
                        }],
                        span: SpanMock::new(),
                    })],
                    span: SpanMock::new(),
                }))],
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
        Bytecode::Print,
        Bytecode::Jmp(10),
        // elif 3 > 4:
        Bytecode::ConstU(3),
        Bytecode::ConstU(4),
        Bytecode::Gt,
        Bytecode::If(3),
        // print("two")
        Bytecode::ConstS("two".to_string().into()),
        Bytecode::Print,
        Bytecode::Jmp(3),
        // else:
        //     print("default")
        Bytecode::ConstS("default".to_string().into()),
        Bytecode::Print,
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
        args: vecdeq![Arg {
            name: "n".to_string(),
            ty: Type::Int,
        }],
        ret_type: Type::Uint,
        namespace: None,
        body: vec![
            NodeStmnt::IfStmnt(NodeIfStmnt {
                condition: NodeExpr {
                    expr: vecdeq![
                        NodeExprInner::Resolution(NodeAttrRes {
                            resolution: vec![NodeAttribute::VarCall(NodeVarCall {
                                name: "n".to_string(),
                                span: SpanMock::new()
                            })],
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
                            NodeExprInner::Resolution(NodeAttrRes {
                                resolution: vec![NodeAttribute::FuncCall(NodeFuncCall {
                                    name: "fib".to_string(),
                                    args: vec![NodeExpr {
                                        expr: vecdeq![
                                            NodeExprInner::Resolution(NodeAttrRes {
                                                resolution: vec![NodeAttribute::VarCall(
                                                    NodeVarCall {
                                                        name: "n".to_string(),
                                                        span: SpanMock::new(),
                                                    }
                                                )],
                                                span: SpanMock::new()
                                            }),
                                            NodeExprInner::Value(NodeValue::Uint(2)),
                                            NodeExprInner::BinOpr(BinOprType::Sub)
                                        ],
                                        span: SpanMock::new()
                                    }],
                                    namespace: None,
                                    span: SpanMock::new()
                                })],
                                span: SpanMock::new()
                            }),
                            NodeExprInner::Resolution(NodeAttrRes {
                                resolution: vec![NodeAttribute::FuncCall(NodeFuncCall {
                                    name: "fib".to_string(),
                                    args: vec![NodeExpr {
                                        expr: vecdeq![
                                            NodeExprInner::Resolution(NodeAttrRes {
                                                resolution: vec![NodeAttribute::VarCall(
                                                    NodeVarCall {
                                                        name: "n".to_string(),
                                                        span: SpanMock::new(),
                                                    }
                                                )],
                                                span: SpanMock::new()
                                            }),
                                            NodeExprInner::Value(NodeValue::Uint(1)),
                                            NodeExprInner::BinOpr(BinOprType::Sub)
                                        ],
                                        span: SpanMock::new()
                                    }],
                                    namespace: None,
                                    span: SpanMock::new()
                                })],
                                span: SpanMock::new()
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
        Bytecode::GetLocal(0),
        Bytecode::ConstU(2),
        Bytecode::Gt,
        Bytecode::If(11),
        // return fib(n-2) + fib(n-1)
        Bytecode::GetLocal(0),
        Bytecode::ConstU(2),
        Bytecode::Sub,
        Bytecode::CallFunc(fib_id),
        Bytecode::GetLocal(0),
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
            NodeStmnt::FuncCall(NodeFuncCallStmnt(NodeAttrRes {
                resolution: vec![NodeAttribute::FuncCall(NodeFuncCall {
                    name: "print".to_string(),
                    namespace: None,
                    args: vec![NodeExpr {
                        expr: vecdeq![
                            NodeExprInner::Value(NodeValue::Uint(21)),
                            NodeExprInner::Value(NodeValue::Uint(2)),
                            NodeExprInner::BinOpr(BinOprType::Mul)
                        ],
                        span: SpanMock::new(),
                    }],
                    span: SpanMock::new(),
                })],
                span: SpanMock::new(),
            })),
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
        catch_body: vec![NodeStmnt::FuncCall(NodeFuncCallStmnt(NodeAttrRes {
            resolution: vec![NodeAttribute::FuncCall(NodeFuncCall {
                name: "print".to_string(),
                namespace: None,
                args: vec![NodeExpr {
                    expr: vecdeq![
                        NodeExprInner::Value(NodeValue::Str(
                            "Received the exception: ".to_string()
                        )),
                        NodeExprInner::Resolution(NodeAttrRes {
                            resolution: vec![NodeAttribute::VarCall(NodeVarCall {
                                name: "exc".to_string(),
                                span: SpanMock::new()
                            })],
                            span: SpanMock::new()
                        }),
                        NodeExprInner::BinOpr(BinOprType::Add)
                    ],
                    span: SpanMock::new(),
                }],
                span: SpanMock::new(),
            })],
            span: SpanMock::new(),
        }))],
    };

    let exp = vec![
        // try:
        Bytecode::TryScope(7),
        // print(21 * 2)
        Bytecode::ConstU(21),
        Bytecode::ConstU(2),
        Bytecode::Mul,
        Bytecode::Print,
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
        Bytecode::Print,
    ];

    let recv = code
        .to_bytecode(&mut Chalcedony::new())
        .expect("did not compile properly");

    assert_eq!(exp, recv);
}

#[test]
fn compile_classes() {
    // equivalent to the code:
    // ```
    // class Example:
    //     result: uint
    //
    //     fn new(value: uint) -> Example:
    //         return Example {result: value * value}
    //
    // print(Example::new(4).result)
    // ```

    let class_def = NodeClass {
        name: "Example".to_string(),
        members: vec![Member {
            name: "result".to_string(),
            ty: Type::Uint,
            span: SpanMock::new(),
        }],
        methods: vec![NodeFuncDef {
            name: "new".to_string(),
            ret_type: Type::Custom(Box::new("Example".to_string())),
            namespace: Some("Example".to_string()),
            args: vecdeq![Arg {
                name: "value".to_string(),
                ty: Type::Uint
            }],
            body: vec![NodeStmnt::RetStmnt(NodeRetStmnt {
                value: NodeExpr {
                    expr: vecdeq![NodeExprInner::InlineClass(NodeInlineClass {
                        class: "Example".to_string(),
                        members: hash_map!(
                                "result".to_string() => (NodeExpr {
                                        expr: vecdeq![
                                            NodeExprInner::Resolution(
                                                NodeAttrRes {
                                                    resolution: vec![
                                                        NodeAttribute::VarCall(NodeVarCall {
                                                                name: "value".to_string(),
                                                                span: SpanMock::new() }
                                                            )],
                                                    span: SpanMock::new(),
                                                }
                                            ),
                                            NodeExprInner::Resolution(
                                                NodeAttrRes {
                                                    resolution: vec![
                                                    NodeAttribute::VarCall(NodeVarCall {
                                                        name: "value".to_string(),
                                                        span: SpanMock::new() }
                                                    )],
                                                span: SpanMock::new(),
                                                }
                                            ),
                                            NodeExprInner::BinOpr(BinOprType::Mul)
                                        ],
                                    span: SpanMock::new(),
                                }, SpanMock::new())
                        ),
                        span: SpanMock::new(),
                    })],
                    span: SpanMock::new(),
                },
                span: SpanMock::new(),
            })],
            span: SpanMock::new(),
        }],
        span: SpanMock::new(),
    };

    let print_code = NodeFuncCall {
        name: "print".to_string(),
        namespace: None,
        args: vec![NodeExpr {
            expr: vecdeq![NodeExprInner::Resolution(NodeAttrRes {
                resolution: vec![
                    NodeAttribute::FuncCall(NodeFuncCall {
                        name: "new".to_string(),
                        namespace: Some("Example".to_string()),
                        args: vec![NodeExpr {
                            expr: vecdeq![NodeExprInner::Value(NodeValue::Uint(4))],
                            span: SpanMock::new(),
                        }],
                        span: SpanMock::new(),
                    }),
                    NodeAttribute::VarCall(NodeVarCall {
                        name: "result".to_string(),
                        span: SpanMock::new()
                    })
                ],
                span: SpanMock::new(),
            })],
            span: SpanMock::new(),
        }],
        span: SpanMock::new(),
    };

    let mut interpreter = Chalcedony::new();
    /* the id of `Example::new()` */
    let example_new_id = interpreter.get_next_func_id();

    let exp_class_namespace = ClassNamespace {
        members: hash_map!(
        "result".to_string() => MemberAnnotation {id: 0, ty: Type::Uint},
        ),
        methods: hash_map!(
        "new".to_string() => vec![Rc::new(FuncAnnotation::new(
                example_new_id,
                vec![ArgAnnotation::new(0, "value".to_string(), Type::Uint)],
                Type::Custom(Box::new("Example".to_string())),
                false
                ))]
        ),
    };

    let exp_print_bytecode = vec![
        Bytecode::ConstU(4),
        Bytecode::CallFunc(example_new_id),
        Bytecode::GetAttr(0),
        Bytecode::Print,
    ];

    let class_bytecode = class_def
        .to_bytecode(&mut interpreter)
        .expect("could not compile NodeClass");
    assert_eq!(Vec::<Bytecode>::new(), class_bytecode);
    assert_eq!(
        Some(&exp_class_namespace),
        interpreter.get_namespace("Example")
    );

    let print_bytecode = print_code
        .to_bytecode(&mut interpreter)
        .expect("could not compile NodeClass");
    assert_eq!(exp_print_bytecode, print_bytecode)
}
