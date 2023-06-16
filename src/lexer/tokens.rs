use crate::errors::lexer_errors::LexerError;
use crate::Span;
use crate::errors::span::pos::Position;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
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
    Let,
    Fn,
    Nf,
    Return,
    If,
    Fi,
    Else,
    Elif,
    While,
    For,
    Done,
}

lazy_static! {
    static ref SPECIAL: HashSet<char> = {
        HashSet::from([
           '(', ')', '[', ']', 
           '{', '}', ':', ';',
           '+', '-', '*', '/', 
           '%', '=', '<', '>', 
           '!', ',', 
           // '&', '|', '~', '^', ','
        ])
    };

    static ref OPERATORS: HashSet<char> = {
        HashSet::from([
           '+', '-', '*', '/', 
           '%', '=', '<', '>', 
           '!' 
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
    Identifier(String),
    Error(LexerError), // an encountered error
    None,
    Null,

    Sharp,        // #
    Dollar,       // $

    OpenPar,      // ( 
    ClosePar,     // )
    OpenBracket,  // [
    CloseBracket, // ]
    OpenBrace,    // {
    CloseBrace,   // }
    Comma,        // , 
    Dot,          // . 
    Colon,        // : 
    SemiColon,    // ;
    Newline,      // \n 
    RightArrow,   // ->
    BigRightArrow,// =>

    Add,          // +
    Sub,          // -
    Mul,          // *
    Div,          // /
    Mod,          // %
    Eq,           // =
    Lt,           // <
    Gt,           // >

    Bang,         // !
    BinAnd,       // &
    BinOr,        // |
    Tilde,        // ~
    Xor,          // ^
    And,          // &&
    Or,           // ||

    AddEq,        // +=
    SubEq,        // -=
    MulEq,        // *=
    DivEq,        // /=
    ModEq,        // %=
    EqEq,         // ==
    LtEq,         // <=
    GtEq,         // >=
    BangEq,       // !=
}

impl From<&str> for TokenKind {
    fn from(src: &str) -> TokenKind {
        if src == "\n" { return TokenKind::Newline; }

        if src == "" {
            return TokenKind::None; // error kind
        }

        match src {
           "i8"     => return TokenKind::Keyword(Keyword::I8),
           "i16"    => return TokenKind::Keyword(Keyword::I16),
           "i32"    => return TokenKind::Keyword(Keyword::I32),
           "i64"    => return TokenKind::Keyword(Keyword::I64),

           "u8"     => return TokenKind::Keyword(Keyword::U8),
           "u16"    => return TokenKind::Keyword(Keyword::U16),
           "u32"    => return TokenKind::Keyword(Keyword::U32),
           "u64"    => return TokenKind::Keyword(Keyword::U64),

           "f32"    => return TokenKind::Keyword(Keyword::F32),
           "f64"    => return TokenKind::Keyword(Keyword::F64),
           "str"    => return TokenKind::Keyword(Keyword::Str),
           "let"    => return TokenKind::Keyword(Keyword::Let),
           "null"   => return TokenKind::Null,

           "fn"     => return TokenKind::Keyword(Keyword::Fn),
           "nf"     => return TokenKind::Keyword(Keyword::Nf),
           "return" => return TokenKind::Keyword(Keyword::Return),
           "if"     => return TokenKind::Keyword(Keyword::If),
           "fi"     => return TokenKind::Keyword(Keyword::Fi),
           "else"   => return TokenKind::Keyword(Keyword::Else),
           "elif"   => return TokenKind::Keyword(Keyword::Elif),
           "while"  => return TokenKind::Keyword(Keyword::While),
           "for"    => return TokenKind::Keyword(Keyword::For),
           "done"   => return TokenKind::Keyword(Keyword::Done),

           "#" => return TokenKind::Sharp,
           "$" => return TokenKind::Dollar,

           "(" => return TokenKind::OpenPar,
           ")" => return TokenKind::ClosePar,
           "[" => return TokenKind::OpenBracket,
           "]" => return TokenKind::CloseBracket,
           "{" => return TokenKind::OpenBrace,
           "}" => return TokenKind::CloseBrace,
           "," => return TokenKind::Comma,
           "." => return TokenKind::Dot,
           ":" => return TokenKind::Colon,
           ";" => return TokenKind::SemiColon,
           "->" => return TokenKind::RightArrow,
           "=>" => return TokenKind::BigRightArrow,

           "+" => return TokenKind::Add,
           "-" => return TokenKind::Sub,
           "*" => return TokenKind::Mul,
           "/" => return TokenKind::Div,
           "%" => return TokenKind::Mod,
           "=" => return TokenKind::Eq,
           "<" => return TokenKind::Lt,
           ">" => return TokenKind::Gt,

           "!" => return TokenKind::Bang,
           "&" => return TokenKind::BinAnd,
           "|" => return TokenKind::BinOr,
           "~" => return TokenKind::Tilde,
           "^" => return TokenKind::Xor,
           "&&" => return TokenKind::And,
           "||" => return TokenKind::Or,

           "+=" => return TokenKind::AddEq,
           "-=" => return TokenKind::SubEq,
           "*=" => return TokenKind::MulEq,
           "/=" => return TokenKind::DivEq,
           "%=" => return TokenKind::ModEq,
           "==" => return TokenKind::EqEq,
           "<=" => return TokenKind::LtEq,
           ">=" => return TokenKind::GtEq,
           "!=" => return TokenKind::BangEq,
           _ => (),
        }

        if let Ok(kind) = src.parse::<u64>() {
            return TokenKind::Uint(kind);
        }

        if let Ok(kind) = src.parse::<i64>() {
            return TokenKind::Int(kind);
        }

        if let Ok(kind) = src.parse::<f64>() {
            return TokenKind::Float(kind);
        }

        if src.chars().nth(0) == Some('"') && src.chars().nth(src.len() - 1) == Some('"') {
            return TokenKind::Str(src.to_string());

        } else if src.chars().nth(0) == Some('"') {
            return TokenKind::Error(LexerError::UnclosedString);
        }

        if src.chars().all(|c: char| is_special(&c) ) {
            return TokenKind::None;
        }

        if src.chars().nth(0).unwrap().is_numeric() || !src.chars().all(|c: char| -> bool {c.is_ascii_alphanumeric()}) {
            return TokenKind::Error(LexerError::InvalidIdentifier);
        }

        return TokenKind::Identifier(src.to_string());
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
    pub fn new(src: String, start: &Position, end: &Position) -> Token {
        Token {
            kind:  TokenKind::from(&src[..]),
            start: start.clone(),
            end:   end.clone(),
            src,
        }
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
                LexerError::InvalidIdentifier => {
                    let span: (String, usize) = src.context_span(self.start(), self.end()).unwrap();
                    eprintln!("Error: invalid identifier:");
                    eprintln!("{}", span.0);

                    // we get the line number offset
                    let ln_len = std::cmp::max(self.end.ln.to_string().len(), 4);
                    for _ in 0 .. ln_len { eprint!(" "); } 

                    eprint!("| ");
                    for _ in 0 .. span.1 - (ln_len + 2) { eprint!(" "); }

                    // here we don't use self.src.len() in case the invalid chars are UTF-8,
                    // resulting in a difference in length
                    for _ in 0 .. self.end.col - self.start.col { eprint!("^"); }
                    eprintln!("\n");
                },

                LexerError::UnclosedString => {
                    let span: (String, usize) = src.context_span(self.start(), self.end()).unwrap();
                    eprintln!("Error: unclosed string:");
                    eprintln!("{}\n", span.0);
                },

                LexerError::UnclosedComment => {
                    let span: (String, usize) = src.context_span(self.start(), self.end()).unwrap();
                    eprintln!("Error: unclosed multiline comment:");
                    eprintln!("{}\n", span.0);
                },

                LexerError::UnclosedDelimiter(del) => {
                    let span: (String, usize) = src.context_span(self.start(), self.end()).unwrap();
                    eprintln!("Error: unclosed delimiter ('{}'):", del);
                    eprintln!("{}\n", span.0);
                },

                LexerError::UnexpectedClosingDelimiter(del) => {
                    let span: (String, usize) = src.context_span(self.start(), self.end()).unwrap();
                    eprintln!("Error: unexpected closing delimiter ('{}'):", del);
                    eprintln!("{}\n", span.0);
                },

                // odel - opening delimiter, cdel - closing delimiter
                LexerError::MissmatchingDelimiter(odel, cdel) => {
                    let span: (String, usize) = src.context_span(self.start(), self.end()).unwrap();
                    eprintln!("Error: missmatching delimiter ('{}' and '{}'):", odel, cdel);
                    eprintln!("{}\n", span.0);
                },
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

