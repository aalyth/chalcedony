use crate::error::{ChalError, ParserError, Span};
use crate::lexer;
use crate::lexer::{Delimiter, Token, TokenKind};
use crate::parser::ast::operators::{BinOprType, UnaryOprType};
use crate::parser::ast::{NodeFuncCall, NodeValue, NodeVarCall};

use crate::parser::TokenReader;

use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
enum NodeExprInner {
    BinOpr(BinOprType),
    UnaryOpr(UnaryOprType),
    Value(NodeValue),
    VarCall(NodeVarCall),
    FuncCall(NodeFuncCall),
}

#[derive(Debug)]
pub struct NodeExpr {
    expr: VecDeque<NodeExprInner>,
}

struct Stack<T> {
    values: VecDeque<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            values: VecDeque::<T>::new(),
        }
    }

    fn push(&mut self, val: T) {
        self.values.push_back(val);
    }

    fn pop(&mut self) -> Option<T> {
        self.values.pop_back()
    }

    fn peek(&self) -> Option<&T> {
        self.values.back()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<T> Into<VecDeque<T>> for Stack<T> {
    fn into(self) -> VecDeque<T> {
        self.values
    }
}

#[derive(PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,

    And,
    Or,

    Gt,
    Lt,
    GtEq,
    LtEq,
    EqEq,
    BangEq,
    Bang,

    OpenPar,
}

impl Operator {
    fn precedence(&self) -> u64 {
        match self {
            Operator::Add => 5,
            Operator::Sub => 5,
            Operator::Mul => 6,
            Operator::Div => 6,
            Operator::Mod => 6,

            Operator::And => 2,
            Operator::Or => 1,

            Operator::Gt => 4,
            Operator::Lt => 4,
            Operator::GtEq => 4,
            Operator::LtEq => 4,
            Operator::EqEq => 3,
            Operator::BangEq => 3,

            /* technically the negation and bang operators are right-associative, but having highest
             * precedence achieves the same result without needing to refactor current code */
            Operator::Bang => 999,
            Operator::Neg => 999,
            Operator::OpenPar => 0,
        }
    }
}

impl TryInto<NodeExprInner> for Operator {
    type Error = ();
    fn try_into(self) -> Result<NodeExprInner, ()> {
        return match self {
            Operator::Add => Ok(NodeExprInner::BinOpr(BinOprType::Add)),
            Operator::Sub => Ok(NodeExprInner::BinOpr(BinOprType::Sub)),
            Operator::Mul => Ok(NodeExprInner::BinOpr(BinOprType::Mul)),
            Operator::Div => Ok(NodeExprInner::BinOpr(BinOprType::Div)),
            Operator::Mod => Ok(NodeExprInner::BinOpr(BinOprType::Mod)),

            Operator::And => Ok(NodeExprInner::BinOpr(BinOprType::And)),
            Operator::Or => Ok(NodeExprInner::BinOpr(BinOprType::Or)),

            Operator::Gt => Ok(NodeExprInner::BinOpr(BinOprType::Gt)),
            Operator::Lt => Ok(NodeExprInner::BinOpr(BinOprType::Lt)),
            Operator::GtEq => Ok(NodeExprInner::BinOpr(BinOprType::GtEq)),
            Operator::LtEq => Ok(NodeExprInner::BinOpr(BinOprType::LtEq)),
            Operator::EqEq => Ok(NodeExprInner::BinOpr(BinOprType::EqEq)),
            Operator::BangEq => Ok(NodeExprInner::BinOpr(BinOprType::BangEq)),

            Operator::Bang => Ok(NodeExprInner::UnaryOpr(UnaryOprType::Bang)),
            Operator::Neg => Ok(NodeExprInner::UnaryOpr(UnaryOprType::Neg)),
            _ => Err(()),
        };
    }
}

impl TryFrom<&lexer::Operator> for Operator {
    type Error = ();

    fn try_from(val: &lexer::Operator) -> Result<Operator, ()> {
        return match val {
            lexer::Operator::Add => Ok(Operator::Add),
            lexer::Operator::Sub => Ok(Operator::Sub),
            lexer::Operator::Mul => Ok(Operator::Mul),
            lexer::Operator::Div => Ok(Operator::Div),
            lexer::Operator::Mod => Ok(Operator::Mod),

            lexer::Operator::And => Ok(Operator::And),
            lexer::Operator::Or => Ok(Operator::Or),

            lexer::Operator::Gt => Ok(Operator::Gt),
            lexer::Operator::Lt => Ok(Operator::Lt),
            lexer::Operator::GtEq => Ok(Operator::GtEq),
            lexer::Operator::LtEq => Ok(Operator::LtEq),
            lexer::Operator::EqEq => Ok(Operator::EqEq),
            lexer::Operator::BangEq => Ok(Operator::BangEq),

            lexer::Operator::Bang => Ok(Operator::Bang),
            lexer::Operator::Neg => Ok(Operator::Neg),
            _ => Err(()),
        };
    }
}

impl NodeExpr {
    pub fn new(tokens: VecDeque<Token>, span: Rc<Span>) -> Result<NodeExpr, ChalError> {
        // NOTE: filter unnecessary expressions like:
        // a*--b -> can be simplified to a*b

        /* TODO: catch errors when there are 2 operands next to each other without an operator,
         * or when there are 2 operators without an operand */

        /* this is using the Shunting Yard algorithm */
        let mut reader = TokenReader::new(tokens, span.clone());

        let mut output = Stack::<NodeExprInner>::new();
        let mut operators = Stack::<Operator>::new();

        while !reader.is_empty() {
            let current = reader.advance().unwrap();

            match current.kind() {
                TokenKind::Int(val) => output.push(NodeExprInner::Value(NodeValue::Int(*val))),
                TokenKind::Uint(val) => output.push(NodeExprInner::Value(NodeValue::Uint(*val))),
                TokenKind::Float(val) => output.push(NodeExprInner::Value(NodeValue::Float(*val))),
                TokenKind::Str(val) => {
                    output.push(NodeExprInner::Value(NodeValue::Str(val.clone())))
                }

                TokenKind::Identifier(_) => {
                    let Some(peek) = reader.peek() else {
                        output.push(NodeExprInner::VarCall(NodeVarCall::new(
                            current,
                            span.clone(),
                        )?));
                        continue;
                    };

                    if let TokenKind::Delimiter(Delimiter::OpenPar) = peek.kind() {
                        let mut buffer = VecDeque::<Token>::new();
                        buffer.push_back(current);
                        /* push the open parenthesis */
                        buffer.push_back(reader.advance().unwrap());
                        let mut open_delims: u64 = 1;

                        while !reader.is_empty() && open_delims > 0 {
                            let current = reader.advance().unwrap();

                            match current.kind() {
                                TokenKind::Delimiter(Delimiter::OpenPar) => open_delims += 1,
                                TokenKind::Delimiter(Delimiter::ClosePar) => open_delims -= 1,
                                _ => (),
                            }
                            buffer.push_back(current);
                        }
                        output.push(NodeExprInner::FuncCall(NodeFuncCall::new(
                            buffer,
                            span.clone(),
                        )?));
                        continue;
                    }

                    output.push(NodeExprInner::VarCall(NodeVarCall::new(
                        current,
                        span.clone(),
                    )?));
                }

                TokenKind::Operator(current_opr) => {
                    let Ok(opr) = Operator::try_from(current_opr) else {
                        return Err(ChalError::from(ParserError::unexpected_token(
                            current.kind().clone(),
                            current.start(),
                            current.end(),
                            span.clone(),
                        )));
                    };

                    let current_precedence = opr.precedence();
                    /* NOTE: inside the while we use a greater or equal (>=) check, instead of the usual
                     * greater than (>), due to the fact that in this implementation, right-associative
                     * operators (such as +=, -=, *=, etc.) are handled as statements */
                    while operators.peek().is_some()
                        && operators.peek().unwrap().precedence() >= current_precedence
                    {
                        let top = operators.pop().unwrap();
                        output.push(top.try_into().unwrap());
                    }

                    operators.push(opr);
                }

                TokenKind::Delimiter(Delimiter::OpenPar) => {
                    operators.push(Operator::OpenPar);
                }

                TokenKind::Delimiter(Delimiter::ClosePar) => {
                    while operators.peek() != Some(&Operator::OpenPar) {
                        let opr = operators.pop().unwrap();
                        output.push(opr.try_into().unwrap());
                    }

                    /* remove the ClosePar at the end */
                    operators.pop();
                }

                _ => (),
            }
        }

        while !operators.is_empty() {
            output.push(operators.pop().unwrap().try_into().unwrap());
        }

        Ok(NodeExpr {
            expr: output.into(),
        })
    }
}
