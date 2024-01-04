use crate::error::span::pos::Position;
use crate::error::span::Span;
use crate::error::{ChalError, InternalError, LexerError};

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    Let,
    Fn,
    Return,
    If,
    Elif,
    Else,
    While,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Int,
    Uint,
    Float,
    Str,
    Bool,
    Any,
    Void,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Special {
    Comma,         // ,
    Dot,           // .
    Colon,         // :
    SemiColon,     // ;
    Newline,       // \n
    RightArrow,    // ->
    BigRightArrow, // =>
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
    Neg, // - (negation as an unary operator)
    Eq,  // =
    Lt,  // <
    Gt,  // >

    Bang,   // !
    BinAnd, // &
    BinOr,  // |
    Tilde,  // ~
    Xor,    // ^
    And,    // &&
    Or,     // ||

    AddEq,  // +=
    SubEq,  // -=
    MulEq,  // *=
    DivEq,  // /=
    ModEq,  // %=
    EqEq,   // ==
    LtEq,   // <=
    GtEq,   // >=
    BangEq, // !=
    Walrus, // :=
}

#[derive(PartialEq, Debug, Clone)]
pub enum Delimiter {
    OpenPar,      // (
    ClosePar,     // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
}

impl Delimiter {
    pub fn inverse(&self) -> Self {
        match *self {
            Delimiter::OpenPar => Delimiter::ClosePar,
            Delimiter::ClosePar => Delimiter::OpenPar,
            Delimiter::OpenBrace => Delimiter::CloseBrace,
            Delimiter::CloseBrace => Delimiter::OpenBrace,
            Delimiter::OpenBracket => Delimiter::CloseBracket,
            Delimiter::CloseBracket => Delimiter::OpenBracket,
        }
    }
}

pub fn is_special(c: &char) -> bool {
    match *c {
        '(' | ')' | '[' | ']' | '{' | '}' | ':' | ';' | '+' | '-' | '*' | '/' | '%' | '=' | '<'
        | '>' | '!' | ',' | '&' | '|' => true,
        _ => false,
    }
}

pub fn is_operator(c: &char) -> bool {
    match *c {
        '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | ':' | '&' | '|' => true,
        _ => false,
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    Bool(bool),

    Keyword(Keyword),
    Type(Type),
    Identifier(String),
    Special(Special),
    Operator(Operator),
    Delimiter(Delimiter),
    Newline,
}

impl TokenKind {
    fn new(
        src: &str,
        start: &Position,
        end: &Position,
        span: &Rc<Span>,
    ) -> Result<TokenKind, ChalError> {
        if src == "" {
            return Err(InternalError::new("TokenKind::new(): lexing an empty string").into());
        }
        if src == "\n" {
            return Ok(TokenKind::Newline);
        }

        match src {
            /* TYPES */
            "int" => return Ok(TokenKind::Type(Type::Int)),
            "uint" => return Ok(TokenKind::Type(Type::Uint)),
            "float" => return Ok(TokenKind::Type(Type::Float)),
            "str" => return Ok(TokenKind::Type(Type::Str)),
            "bool" => return Ok(TokenKind::Type(Type::Bool)),
            "void" => return Ok(TokenKind::Type(Type::Void)),

            /* KEYWORDS */
            "let" => return Ok(TokenKind::Keyword(Keyword::Let)),
            "fn" => return Ok(TokenKind::Keyword(Keyword::Fn)),
            "return" => return Ok(TokenKind::Keyword(Keyword::Return)),
            "if" => return Ok(TokenKind::Keyword(Keyword::If)),
            "else" => return Ok(TokenKind::Keyword(Keyword::Else)),
            "elif" => return Ok(TokenKind::Keyword(Keyword::Elif)),
            "while" => return Ok(TokenKind::Keyword(Keyword::While)),

            /* DELIMITERS */
            "(" => return Ok(TokenKind::Delimiter(Delimiter::OpenPar)),
            ")" => return Ok(TokenKind::Delimiter(Delimiter::ClosePar)),
            "[" => return Ok(TokenKind::Delimiter(Delimiter::OpenBracket)),
            "]" => return Ok(TokenKind::Delimiter(Delimiter::CloseBracket)),
            "{" => return Ok(TokenKind::Delimiter(Delimiter::OpenBrace)),
            "}" => return Ok(TokenKind::Delimiter(Delimiter::CloseBrace)),

            /* SPECIALS */
            "," => return Ok(TokenKind::Special(Special::Comma)),
            "." => return Ok(TokenKind::Special(Special::Dot)),
            ":" => return Ok(TokenKind::Special(Special::Colon)),
            ";" => return Ok(TokenKind::Special(Special::SemiColon)),
            "->" => return Ok(TokenKind::Special(Special::RightArrow)),
            "=>" => return Ok(TokenKind::Special(Special::BigRightArrow)),

            /* OPERATORS */
            "+" => return Ok(TokenKind::Operator(Operator::Add)),
            "-" => return Ok(TokenKind::Operator(Operator::Sub)),
            "*" => return Ok(TokenKind::Operator(Operator::Mul)),
            "/" => return Ok(TokenKind::Operator(Operator::Div)),
            "%" => return Ok(TokenKind::Operator(Operator::Mod)),
            "=" => return Ok(TokenKind::Operator(Operator::Eq)),
            "<" => return Ok(TokenKind::Operator(Operator::Lt)),
            ">" => return Ok(TokenKind::Operator(Operator::Gt)),

            "!" => return Ok(TokenKind::Operator(Operator::Bang)),
            "&" => return Ok(TokenKind::Operator(Operator::BinAnd)),
            "|" => return Ok(TokenKind::Operator(Operator::BinOr)),
            "~" => return Ok(TokenKind::Operator(Operator::Tilde)),
            "^" => return Ok(TokenKind::Operator(Operator::Xor)),
            "&&" => return Ok(TokenKind::Operator(Operator::And)),
            "||" => return Ok(TokenKind::Operator(Operator::Or)),

            "+=" => return Ok(TokenKind::Operator(Operator::AddEq)),
            "-=" => return Ok(TokenKind::Operator(Operator::SubEq)),
            "*=" => return Ok(TokenKind::Operator(Operator::MulEq)),
            "/=" => return Ok(TokenKind::Operator(Operator::DivEq)),
            "%=" => return Ok(TokenKind::Operator(Operator::ModEq)),
            "==" => return Ok(TokenKind::Operator(Operator::EqEq)),
            "<=" => return Ok(TokenKind::Operator(Operator::LtEq)),
            ">=" => return Ok(TokenKind::Operator(Operator::GtEq)),
            "!=" => return Ok(TokenKind::Operator(Operator::BangEq)),
            ":=" => return Ok(TokenKind::Operator(Operator::Walrus)),

            "true" => return Ok(TokenKind::Bool(true)),
            "false" => return Ok(TokenKind::Bool(false)),

            _ => (),
        };

        /* TODO: check if this is ok
        if let Ok(kind) = src.parse::<u64>() {
            return Ok(TokenKind::Uint(kind));
        }
        */

        if let Ok(val) = src.parse::<i64>() {
            return Ok(TokenKind::Int(val));
        }

        if let Ok(val) = src.parse::<f64>() {
            return Ok(TokenKind::Float(val));
        }

        if (src.chars().nth(0) == Some('"') && src.chars().nth(src.len() - 1) == Some('"'))
            || (src.chars().nth(0) == Some('\'') && src.chars().nth(src.len() - 1) == Some('\''))
        {
            return Ok(TokenKind::Str(src[1 .. src.len() - 1].to_string()));
        } else if src.chars().nth(0) == Some('"') {
            return Err(LexerError::unclosed_string(*start, *end, Rc::clone(span)).into());
        }

        if src.chars().nth(0).unwrap().is_numeric()
            || src
                .chars()
                .all(|c: char| -> bool { !c.is_ascii_alphanumeric() && c == '_' })
        {
            return Err(LexerError::invalid_identifier(*start, *end, Rc::clone(span)).into());
        }

        return Ok(TokenKind::Identifier(src.to_string()));
    }

    pub fn is_terminal(&self) -> bool {
        match *self {
            TokenKind::Int(_)
            | TokenKind::Uint(_)
            | TokenKind::Float(_)
            | TokenKind::Str(_)
            | TokenKind::Identifier(_) => true,

            _ => false,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    start: Position,
    end: Position,
    src: String,
}

impl Token {
    pub fn new(
        src: String,
        start: Position,
        end: Position,
        span: &Rc<Span>,
    ) -> Result<Self, ChalError> {
        let kind = TokenKind::new(&src, &start, &end, span)?;
        Ok(Token {
            kind,
            start,
            end,
            src,
        })
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn into_neg(self) -> Result<Self, ChalError> {
        if self.kind != TokenKind::Operator(Operator::Sub) {
            return Err(InternalError::new(
                "Token::into_neg(): trying to convert a non-subtraction token into unary negation",
            )
            .into());
        }

        Ok(Token {
            kind: TokenKind::Operator(Operator::Neg),
            start: self.start,
            end: self.end,
            src: self.src,
        })
    }
}
