use crate::parser::ast::{VarType, NodeExpr};
use crate::lexer::{Token, TokenKind, Keyword};

use crate::errors::{ParserErrors, span::Span};
use crate::parser::TokenReader;

use std::collections::VecDeque;

pub struct NodeVarCall {
    name: String,
}

pub struct NodeVarDef {
    r#type: VarType,
    name:   String,
    value:  Option<NodeExpr>
}

impl NodeVarDef {
    fn new(tokens: VecDeque<Token>, span: &Span) -> Result<NodeVarDef, ()> {
        let mut reader = TokenReader::new(tokens, span);
        let mut result: NodeVarDef;

        reader.expect(&TokenKind::Keyword(Keyword::Let))?;

        if let TokenKind::Identifier(name) = reader.expect(&TokenKind::Identifier(String::new()))?.get_kind() {
            result.name = name.clone();
        }

        match reader.peek() {
            Some(token) => match token.get_kind() {
                TokenKind::Colon => {
                    reader.advance(); // skip the ':'

                },
                _ => (),
            },
            None => ParserErrors::ExpectedToken::msg(reader.pos(), span, TokenKind::Eq),
        }

        reader.expect(&TokenKind::Eq);

        // parse the right side expression

        Ok(result)
    }
}
