//! Since the `tests` directory is not directly a module of the `chalcedony` crate,
//! it is easier to contain all of the files in this `mocks` module as a part of
//! the global crate.

use crate::error::span::{Position, Span, Spanning};
use crate::lexer::{Token, TokenKind};

use std::rc::Rc;

pub struct SpanMock();

impl Spanning for SpanMock {
    fn context(&self, _: &Position, _: &Position) -> String {
        "a mocked value".to_string()
    }

    fn filename(&self) -> Option<String> {
        None
    }
}

impl SpanMock {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Span {
        Span::new(Position::new(1, 1), Position::new(1, 1), Self::spanner())
    }

    pub fn spanner() -> Rc<dyn Spanning> {
        Rc::new(SpanMock {})
    }
}

pub fn mock_token(kind: TokenKind, span: Span) -> Token {
    let src = "not used".to_string();
    Token { kind, span, src }
}

#[macro_export]
macro_rules! line {
    ($indent: expr, $( $tok_kind:expr ),* ) => {{
        use chalcedony::lexer::{Line, Token};
        use chalcedony::mocks::{SpanMock, mock_token};
        use std::collections::VecDeque;

        let span = SpanMock::new();
        let mut tokens = VecDeque::<Token>::new();
        $( tokens.push_back(mock_token($tok_kind, span.clone())); )*
        tokens.push_back(mock_token(TokenKind::Newline, span.clone()));
        Line::new($indent, tokens)
    }}
}
pub use line;

#[macro_export]
macro_rules! chunk {
    ($($line:expr), *) => {{
        use std::collections::VecDeque;

        let mut vec = VecDeque::new();
        $( vec.push_back($line); )*
        vec
    }}
}
pub use chunk;

#[macro_export]
macro_rules! line_reader {
    ($($line:expr), *) => {{
        use chalcedony::mocks::SpanMock;
        use chalcedony::parser::LineReader;
        use std::collections::VecDeque;

        let mut vec = VecDeque::new();
        $( vec.push_back($line); )*
        LineReader::new(vec, SpanMock::spanner())
    }}
}
pub use line_reader;

#[macro_export]
macro_rules! token_reader {
    ($($tok_kind:expr), *) => {{
        use chalcedony::mocks::{SpanMock, mock_token};
        use chalcedony::parser::TokenReader;
        use std::collections::VecDeque;

        let span = SpanMock::new();
        let mut tokens = VecDeque::new();
        $( tokens.push_back(mock_token($tok_kind, span.clone())); )*
        tokens.push_back(mock_token(TokenKind::Newline, span.clone()));
        TokenReader::new(tokens, span)
    }};
}
pub use token_reader;

#[macro_export]
macro_rules! vecdeq {
    [$($val:expr), *] => {{
        use std::collections::VecDeque;

        let mut vec = VecDeque::new();
        $( vec.push_back($val); )*
        vec
    }};
}
pub use vecdeq;
