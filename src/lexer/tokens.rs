use crate::errors::LexerError::LexerError;
use crate::errors::span::Span;
use crate::errors::span::pos::Position;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    Let,
    Fn,
    Return,
    If,
    Elif,
    Else,
    While,
    For,
    Void,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Str,
    Any,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    OpenPar,       // ( 
    ClosePar,      // )
    OpenBracket,   // [
    CloseBracket,  // ]
    OpenBrace,     // {
    CloseBrace,    // }
    Comma,         // , 
    Dot,           // . 
    Colon,         // : 
    SemiColon,     // ;
    Newline,       // \n 
    RightArrow,    // ->
    BigRightArrow, // =>

    Add,           // +
    Sub,           // -
    Mul,           // *
    Div,           // /
    Mod,           // %
    Eq,            // =
    Lt,            // <
    Gt,            // >

    Bang,          // !
    BinAnd,        // &
    BinOr,         // |
    Tilde,         // ~
    Xor,           // ^
    And,           // &&
    Or,            // ||

    AddEq,         // +=
    SubEq,         // -=
    MulEq,         // *=
    DivEq,         // /=
    ModEq,         // %=
    EqEq,          // ==
    LtEq,          // <=
    GtEq,          // >=
    BangEq,        // !=
    Walrus,        // :=
}

lazy_static! {
    static ref SPECIAL: HashSet<char> = {
        HashSet::from([
           '(', ')', '[', ']', 
           '{', '}', ':', ';',
           '+', '-', '*', '/', 
           '%', '=', '<', '>', 
           '!', ',', 
        ])
    };

    static ref OPERATORS: HashSet<char> = {
        HashSet::from([
           '+', '-', '*', '/', 
           '%', '=', '<', '>', 
           '!', ':'
        ])
    };
}


pub fn is_special(c: &char) -> bool {
    SPECIAL.contains(c)
}

pub fn is_operator(c: &char) -> bool {
    OPERATORS.contains(c)
}


#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),

    Keyword(Keyword),
    Type(Type),
    Identifier(String),
    Operator(Operator),
    Newline,
}

impl From<&str> for TokenKind {
    fn from(src: &str) -> Result<TokenKind,  {
        if src == "" { return None; }
        if src == "\n" { return Some(TokenKind::Newline); }

        match src {
           "i8"     => return Some(TokenKind::Type(Type::I8)),
           "i16"    => return Some(TokenKind::Type(Type::I16)),
           "i32"    => return Some(TokenKind::Type(Type::I32)),
           "i64"    => return Some(TokenKind::Type(Type::I64)),

           "u8"     => return Some(TokenKind::Type(Type::U8)),
           "u16"    => return Some(TokenKind::Type(Type::U16)),
           "u32"    => return Some(TokenKind::Type(Type::U32)),
           "u64"    => return Some(TokenKind::Type(Type::U64)),

           "f32"    => return Some(TokenKind::Type(Type::F32)),
           "f64"    => return Some(TokenKind::Type(Type::F64)),
           "str"    => return Some(TokenKind::Type(Type::Str)),
           "let"    => return Some(TokenKind::Keyword(Keyword::Let)),
           "void"   => return Some(TokenKind::Keyword(Keyword::Void)),

           "fn"     => return Some(TokenKind::Keyword(Keyword::Fn)),
           "return" => return Some(TokenKind::Keyword(Keyword::Return)),
           "if"     => return Some(TokenKind::Keyword(Keyword::If)),
           "else"   => return Some(TokenKind::Keyword(Keyword::Else)),
           "elif"   => return Some(TokenKind::Keyword(Keyword::Elif)),
           "while"  => return Some(TokenKind::Keyword(Keyword::While)),
           "for"    => return Some(TokenKind::Keyword(Keyword::For)),

           "("  => return Some(TokenKind::Operator(Operator::OpenPar)),
           ")"  => return Some(TokenKind::Operator(Operator::ClosePar)),
           "["  => return Some(TokenKind::Operator(Operator::OpenBracket)),
           "]"  => return Some(TokenKind::Operator(Operator::CloseBracket)),
           "{"  => return Some(TokenKind::Operator(Operator::OpenBrace)),
           "}"  => return Some(TokenKind::Operator(Operator::CloseBrace)),
           ","  => return Some(TokenKind::Operator(Operator::Comma)),
           "."  => return Some(TokenKind::Operator(Operator::Dot)),
           ":"  => return Some(TokenKind::Operator(Operator::Colon)),
           ";"  => return Some(TokenKind::Operator(Operator::SemiColon)),
           "->" => return Some(TokenKind::Operator(Operator::RightArrow)),
           "=>" => return Some(TokenKind::Operator(Operator::BigRightArrow)),

           "+"  => return Some(TokenKind::Operator(Operator::Add)),
           "-"  => return Some(TokenKind::Operator(Operator::Sub)),
           "*"  => return Some(TokenKind::Operator(Operator::Mul)),
           "/"  => return Some(TokenKind::Operator(Operator::Div)),
           "%"  => return Some(TokenKind::Operator(Operator::Mod)),
           "="  => return Some(TokenKind::Operator(Operator::Eq)),
           "<"  => return Some(TokenKind::Operator(Operator::Lt)),
           ">"  => return Some(TokenKind::Operator(Operator::Gt)),

           "!"  => return Some(TokenKind::Operator(Operator::Bang)),
           "&"  => return Some(TokenKind::Operator(Operator::BinAnd)),
           "|"  => return Some(TokenKind::Operator(Operator::BinOr)),
           "~"  => return Some(TokenKind::Operator(Operator::Tilde)),
           "^"  => return Some(TokenKind::Operator(Operator::Xor)),
           "&&" => return Some(TokenKind::Operator(Operator::And)),
           "||" => return Some(TokenKind::Operator(Operator::Or)),

           "+=" => return Some(TokenKind::Operator(Operator::AddEq)),
           "-=" => return Some(TokenKind::Operator(Operator::SubEq)),
           "*=" => return Some(TokenKind::Operator(Operator::MulEq)),
           "/=" => return Some(TokenKind::Operator(Operator::DivEq)),
           "%=" => return Some(TokenKind::Operator(Operator::ModEq)),
           "==" => return Some(TokenKind::Operator(Operator::EqEq)),
           "<=" => return Some(TokenKind::Operator(Operator::LtEq)),
           ">=" => return Some(TokenKind::Operator(Operator::GtEq)),
           "!=" => return Some(TokenKind::Operator(Operator::BangEq)),
           ":=" => return Some(TokenKind::Operator(Operator::Walrus)),

           _ => (),
        }

        if let Ok(kind) = src.parse::<u64>() {
            return Some(TokenKind::Uint(kind));
        }

        if let Ok(kind) = src.parse::<i64>() {
            return Some(TokenKind::Int(kind));
        }

        if let Ok(kind) = src.parse::<f64>() {
            return Some(TokenKind::Float(kind));
        }

        if src.chars().nth(0) == Some('"') && src.chars().nth(src.len() - 1) == Some('"') {
            return Some(TokenKind::Str(src.to_string()));

        } else if src.chars().nth(0) == Some('"') {
            return TokenKind::Error(LexerError::UnclosedString);
        }

        if src.chars().all(|c: char| is_special(&c) ) {
            return TokenKind::None;
        }

        if src.chars().nth(0).unwrap().is_numeric() || src.chars().all(|c: char| -> bool {!c.is_ascii_alphanumeric() && c == '_'}) {
            return TokenKind::Error(LexerError::InvalidIdentifier);
        }

        return Some(TokenKind::Identifier(src.to_string()));
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
    pub fn new(src: String, start: &Position, end: &Position) -> Result<Self, > {

        Ok (
            Token {
                kind:  TokenKind::from(&src[..]),
                start: start.clone(),
                end:   end.clone(),
                src,
            }
        )
    }

    pub fn err(start: &Position, end: &Position, err_kind: &LexerError) -> Token {
        Token {
            kind: TokenKind::Error(err_kind.clone()),
            src: "".to_string(),
            start: start.clone(),
            end:   end.clone(),
        }
    }
    
    pub fn err_msg(&self, src: &Span) -> Result<(), ()>{
        if let TokenKind::Error(err) = &self.kind {
            match err {
                LexerError::InvalidIdentifier => LexerErrors::InvalidIdentifier::msg(&self.start, &self.end, src),
                LexerError::UnclosedString    => LexerErrors::UnclosedString::msg(&self.start, &self.end, src), 
                // we shouldn't reach this case for now
                LexerError::UnclosedComment   => LexerErrors::UnclosedComment::msg(&self.start, &self.end, src),
            }
            return Err(());
        }
        Ok(())
    }

    pub fn get_kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn start(&self) ->  &Position {
        &self.start
    }

    pub fn end(&self) -> &Position {
        &self.end
    }

    pub fn src(&self) -> &str {
        &self.src
    }
}

