use crate::error::{Span, ChalError};
use crate::lexer::{Token, TokenKind, Type};
use crate::parser::{LineReader};

use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct NodeVarDef {
    /* the variable type */
    kind:  Type,
    name:  String,
    value: Option<NodeExpr>,
}

impl NodeVarDef {
    pub fn new(tokens: Line, span: &Rc<Span>) -> Result<NodeVarDef, ChalError> {
        let mut reader = LineReader::new
        /*
        let mut reader = TokenReader::new(tokens, span);
        let mut result = NodeVarDef { 
            name: String::new(),
            r#type: VarType::I8,
            value: None,
        };

        reader.expect(TokenKind::Keyword(Keyword::Let))?;

        if let TokenKind::Identifier(name) = reader.expect(TokenKind::Identifier(String::new()))?.get_kind() {
            result.name = name.clone();
        }

        match reader.peek() {
            Some(token) => match token.get_kind() {
                TokenKind::Colon => {
                    reader.advance(); /* skip the ':' */
                    
                    let type_ = VarType::new(reader.expect(TokenKind::Type(Type::Any))?);
                    result.r#type = type_.unwrap();

                    if reader.is_empty() {return Ok(result); } /* Variable Declaration */

                    reader.expect(TokenKind::Eq)?;
                },

                TokenKind::Walrus => {
                    /* automatically determine the node type */
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
    */
    }
}
