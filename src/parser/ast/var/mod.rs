use crate::parser::ast::{VarType, NodeExpr};
use crate::lexer::{Token, TokenKind, Keyword, Type};

use crate::errors::{ParserErrors, span::Span};
use crate::parser::TokenReader;

use std::collections::VecDeque;

pub struct NodeVarCall {
    name: String,
}

impl NodeVarCall {
    pub fn new(token: &Token) -> Option<NodeVarCall> {
        if let TokenKind::Identifier(val) = token.get_kind() {
            return Some( NodeVarCall { name: val.clone() });
        }
        None
    }
}

#[derive(Debug)]
pub struct NodeVarDef {
    r#type: VarType,
    name:   String,
    // value:  Option<NodeExpr>
}

impl NodeVarDef {
    pub fn new(tokens: VecDeque<Token>, span: &Span) -> Result<NodeVarDef, ()> {
        let mut reader = TokenReader::new(tokens, span);
        let mut result = NodeVarDef { 
            name: String::new(),
            r#type: VarType::I8,
            //value: None,
        };

        reader.expect(TokenKind::Keyword(Keyword::Let))?;

        if let TokenKind::Identifier(name) = reader.expect(TokenKind::Identifier(String::new()))?.get_kind() {
            result.name = name.clone();
        }

        match reader.peek() {
            Some(token) => match token.get_kind() {
                TokenKind::Colon => {
                    reader.advance(); // skip the ':'
                    
                    let type_ = VarType::new(reader.expect(TokenKind::Type(Type::Any))?);
                    result.r#type = type_.unwrap();

                    reader.expect(TokenKind::Eq)?;
                },

                TokenKind::Walrus => {
                    //automatically determine the node type
                },

                _ => {
                    ParserErrors::UnexpectedToken::msg(&token, span);
                    return Err(());
                },
            },

            None => {
                ParserErrors::ExpectedToken::msg(reader.pos(), span, TokenKind::Walrus);
                return Err(());
            },
        }

        // parse the right side expression
        Ok(result)
    }
}
