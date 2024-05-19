use crate::error::{span::Span, ChalError};
use crate::lexer::{Keyword, Special, TokenKind};
use crate::parser::ast::NodeFuncDef;

use crate::common::Type;
use crate::parser::LineReader;

#[derive(Debug, PartialEq)]
pub struct Member {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// The structure denoting the definition of a class type.
///
/// Syntax:
/// `class` \<class-name\>:
///     \<member-name\>: \<type\>
///     \<member-name\>: \<type\>
///     (...)
///     `fn` \<method-name\>(`self`, \<arg\>: \<type\>, (...)) -> \<type\>:
///         \<statements\>
///     `fn` \<method-name\>(\<arg>: \<type\>, (...)) -> \<type\>:
///         \<statements\>
///     (...)
///
/// it is important to note that if the first argument to a method definition is
/// the variable `self`, the type could be infered to be the class' type.
#[derive(Debug, PartialEq)]
pub struct NodeClass {
    pub name: String,
    pub members: Vec<Member>,
    pub methods: Vec<NodeFuncDef>,
    // refers to the class' name declaration
    pub span: Span,
}

impl NodeClass {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        let mut header = reader.advance_reader();
        let start = header.current().start;
        header.expect_exact(TokenKind::Keyword(Keyword::Class))?;
        let name = header.expect_ident()?;
        header.expect_exact(TokenKind::Special(Special::Colon))?;
        let span = Span::new(start, header.current().end, header.spanner());
        header.expect_exact(TokenKind::Newline)?;

        let identifier_discriminant = std::mem::discriminant(&TokenKind::Identifier(String::new()));
        let members_chunk = reader.advance_until(|ln| {
            let Some(kind) = ln.peek_kind() else {
                return false;
            };
            std::mem::discriminant(kind) != identifier_discriminant
        })?;
        let mut members_reader = LineReader::new(members_chunk, reader.spanner());

        let mut members = Vec::<Member>::new();
        while !members_reader.is_empty() {
            let mut member = members_reader.advance_reader();
            let start = member.current().start;

            let name = member.expect_ident()?;
            member.expect_exact(TokenKind::Special(Special::Colon))?;
            let ty = member.expect_type()?;
            let span = Span::new(start, member.current().end, member.spanner());
            member.expect_exact(TokenKind::Newline)?;

            members.push(Member { name, ty, span });
        }

        let mut methods = Vec::<NodeFuncDef>::new();
        while !reader.is_empty() {
            let method = reader.advance_chunk()?;
            methods.push(NodeFuncDef::method(method, name.clone())?);
        }

        Ok(NodeClass {
            name,
            members,
            methods,
            span,
        })
    }
}
