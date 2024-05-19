use crate::error::span::Span;
use crate::error::{ChalError, LexerError, LexerErrorKind};

use crate::common::Type;

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    Let,
    Fn,
    Return,
    If,
    Elif,
    Else,
    While,
    Continue,
    Break,
    For,
    In,
    Try,
    Catch,
    Throw,
    Import,
    Const,
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

/* if an matches!() macro is used the formatting becomes very bad */
#[allow(clippy::match_like_matches_macro)]
pub fn is_special(c: &char) -> bool {
    match *c {
        '(' | ')' | '[' | ']' | '{' | '}' | ':' | ';' | '+' | '-' | '*' | '/' | '%' | '=' | '<'
        | '>' | '!' | ',' | '&' | '|' => true,
        _ => false,
    }
}

pub fn is_operator(c: &char) -> bool {
    matches!(
        *c,
        '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | ':' | '&' | '|'
    )
}

/// The types of tokens which could be built from the source code.
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
    fn new(src: &str, span: &Span) -> Result<TokenKind, ChalError> {
        if src.is_empty() {
            panic!("TokenKind::new(): lexing an empty string")
        }
        if src == "\n" {
            return Ok(TokenKind::Newline);
        }

        match src {
            /* Types */
            "int" => return Ok(TokenKind::Type(Type::Int)),
            "uint" => return Ok(TokenKind::Type(Type::Uint)),
            "float" => return Ok(TokenKind::Type(Type::Float)),
            "str" => return Ok(TokenKind::Type(Type::Str)),
            "bool" => return Ok(TokenKind::Type(Type::Bool)),
            "void" => return Ok(TokenKind::Type(Type::Void)),
            "exception" => return Ok(TokenKind::Type(Type::Exception)),

            /* Keywords */
            "let" => return Ok(TokenKind::Keyword(Keyword::Let)),
            "fn" => return Ok(TokenKind::Keyword(Keyword::Fn)),
            "return" => return Ok(TokenKind::Keyword(Keyword::Return)),
            "if" => return Ok(TokenKind::Keyword(Keyword::If)),
            "else" => return Ok(TokenKind::Keyword(Keyword::Else)),
            "elif" => return Ok(TokenKind::Keyword(Keyword::Elif)),
            "while" => return Ok(TokenKind::Keyword(Keyword::While)),
            "continue" => return Ok(TokenKind::Keyword(Keyword::Continue)),
            "break" => return Ok(TokenKind::Keyword(Keyword::Break)),
            "for" => return Ok(TokenKind::Keyword(Keyword::For)),
            "in" => return Ok(TokenKind::Keyword(Keyword::In)),
            "try" => return Ok(TokenKind::Keyword(Keyword::Try)),
            "catch" => return Ok(TokenKind::Keyword(Keyword::Catch)),
            "throw" => return Ok(TokenKind::Keyword(Keyword::Throw)),
            "import" => return Ok(TokenKind::Keyword(Keyword::Import)),
            "const" => return Ok(TokenKind::Keyword(Keyword::Const)),

            /* Delimiters */
            "(" => return Ok(TokenKind::Delimiter(Delimiter::OpenPar)),
            ")" => return Ok(TokenKind::Delimiter(Delimiter::ClosePar)),
            "[" => return Ok(TokenKind::Delimiter(Delimiter::OpenBracket)),
            "]" => return Ok(TokenKind::Delimiter(Delimiter::CloseBracket)),
            "{" => return Ok(TokenKind::Delimiter(Delimiter::OpenBrace)),
            "}" => return Ok(TokenKind::Delimiter(Delimiter::CloseBrace)),

            /* Specials */
            "," => return Ok(TokenKind::Special(Special::Comma)),
            "." => return Ok(TokenKind::Special(Special::Dot)),
            ":" => return Ok(TokenKind::Special(Special::Colon)),
            ";" => return Ok(TokenKind::Special(Special::SemiColon)),
            "->" => return Ok(TokenKind::Special(Special::RightArrow)),
            "=>" => return Ok(TokenKind::Special(Special::BigRightArrow)),

            /* Operators */
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

        /* this way digits such as `123_456` are supported */
        let potential_digit = src.replace('_', "");

        if let Ok(val) = potential_digit.parse::<u64>() {
            return Ok(TokenKind::Uint(val));
        }

        if let Ok(val) = potential_digit.parse::<i64>() {
            return Ok(TokenKind::Int(val));
        }

        if let Ok(val) = potential_digit.parse::<f64>() {
            return Ok(TokenKind::Float(val));
        }

        if (src.starts_with('"') && src.ends_with('"'))
            || (src.starts_with('\'') && src.ends_with('\''))
        {
            return Ok(TokenKind::Str(src[1..src.len() - 1].to_string()));
        }
        if src.starts_with('"') || src.starts_with('\'') {
            return Err(LexerError::new(LexerErrorKind::UnclosedString, span.clone()).into());
        }

        Ok(TokenKind::Identifier(src.to_string()))
    }

    /* used to perform checks such as checking whether an `-` is unary or binary */
    pub fn is_terminal(&self) -> bool {
        matches!(
            *self,
            TokenKind::Int(_)
                | TokenKind::Uint(_)
                | TokenKind::Float(_)
                | TokenKind::Str(_)
                | TokenKind::Identifier(_)
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub src: String,
}

impl Token {
    pub fn new(src: String, span: Span) -> Result<Self, ChalError> {
        let kind = TokenKind::new(&src, &span)?;
        Ok(Token { kind, span, src })
    }

    pub fn into_neg(self) -> Result<Self, ChalError> {
        if self.kind != TokenKind::Operator(Operator::Sub) {
            panic!("Token::into_neg(): trying to convert a non-sub token into unary negation")
        }

        Ok(Token {
            kind: TokenKind::Operator(Operator::Neg),
            span: self.span,
            src: self.src,
        })
    }
}
