use crate::errors::lexer_errors::LexerError;
use crate::Span;
use crate::errors::span::pos::Position;

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
    Auto,
    Fn,
    Return,
    End,
    If,
    Else,
    Elif,
    While,
    For,
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
        if src.contains(char::is_whitespace) || src == "" {
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
           "auto"   => return TokenKind::Keyword(Keyword::Auto),
           "null"   => return TokenKind::Null,

           "fn"     => return TokenKind::Keyword(Keyword::Fn),
           "return" => return TokenKind::Keyword(Keyword::Return),
           "end"    => return TokenKind::Keyword(Keyword::End),
           "if"     => return TokenKind::Keyword(Keyword::If),
           "else"   => return TokenKind::Keyword(Keyword::Else),
           "elif"   => return TokenKind::Keyword(Keyword::Elif),
           "while"  => return TokenKind::Keyword(Keyword::While),
           "for"    => return TokenKind::Keyword(Keyword::For),

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

        if src.chars().nth(0).unwrap().is_numeric() || !src.chars().all(|c: char| -> bool {c.is_ascii_alphanumeric()}) {
            return TokenKind::Error(LexerError::InvalidIdentifier);
        }

        return TokenKind::Identifier(src.to_string());
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    start_pos: Position,
    end_pos: Position,
    src: String,
}

impl Token {
    pub fn new(src: String, start_pos: Position, end_pos: Position) -> Token {
        Token {
            kind: TokenKind::from(&src[..]),
            start_pos: start_pos,
            end_pos: end_pos,
            src: src,
        }
    }

    pub fn err(start_pos: Position, end_pos: Position, err_kind: LexerError) -> Token {
        Token {
            kind: TokenKind::Error(err_kind),
            start_pos: start_pos,
            end_pos: end_pos,
            src: "".to_string(),
        }
    }
    
    pub fn err_msg(&self, src: &Span) -> Result<(), ()>{
        if let TokenKind::Error(err) = &self.kind {
            match err {
                LexerError::InvalidIdentifier => {
                    let span: (String, usize) = src.context_span(self.start_pos(), self.end_pos()).unwrap();
                    eprintln!("Error: invalid identifier:");
                    eprintln!("{}", span.0);

                    let ln_len = std::cmp::max(self.end_pos.ln.to_string().len(), 4);
                    for _ in 0 .. ln_len { eprint!(" "); } 
                    eprint!("| ");
                    for _ in 0 .. span.1 - (ln_len + 2){ eprint!(" "); }

                    // here we don't use self.src.len() in case the invalid chars are UTF-8,
                    // resulting in a difference in length
                    for _ in 0 .. self.end_pos.col - self.start_pos.col { eprint!("^"); }
                    eprintln!("");
                },

                LexerError::UnclosedString => {
                    let span: (String, usize) = src.context_span(self.start_pos(), self.end_pos()).unwrap();
                    eprintln!("Error: unclosed string:");
                    eprintln!("{}", span.0);
                },

                LexerError::UnclosedComment => {
                    let span: (String, usize) = src.context_span(self.start_pos(), self.end_pos()).unwrap();
                    eprintln!("Error: unclosed multiline comment:");
                    eprintln!("{}", span.0);
                },

                LexerError::UnclosedDelimiter(del) => {
                    let span: (String, usize) = src.context_span(self.start_pos(), self.end_pos()).unwrap();
                    eprintln!("Error: unclosed delimiter ('{}'):", del);
                    eprintln!("{}", span.0);
                },

                LexerError::UnexpectedClosingDelimiter(del) => {
                    let span: (String, usize) = src.context_span(self.start_pos(), self.end_pos()).unwrap();
                    eprintln!("Error: unexpected closing delimiter ('{}'):", del);
                    eprintln!("{}", span.0);
                },

                // odel - opening delimiter, cdel - closing delimiter
                LexerError::MissmatchingDelimiter(odel, cdel) => {
                    let span: (String, usize) = src.context_span(self.start_pos(), self.end_pos()).unwrap();
                    eprintln!("Error: missmatching delimiter ('{}' and '{}'):", odel, cdel);
                    eprintln!("{}", span.0);
                },
            }
        }
        Ok(())
    }

    pub fn get_kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn start_pos(&self) ->  &Position {
        &self.start_pos
    }

    pub fn end_pos(&self) -> &Position {
        &self.end_pos
    }

    pub fn src(&self) -> &str {
        &self.src
    }
}

