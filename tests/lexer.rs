use chalcedony::common::Type;
use chalcedony::lexer::{Delimiter, Keyword, Lexer, Line, Operator, Special, TokenKind};

use std::collections::VecDeque;
use std::iter::zip;

use chalcedony::{chunk, line};

#[test]
fn lex_basic() {
    let expected = vec![
        TokenKind::Identifier("identifier123".to_string()),
        TokenKind::Operator(Operator::Add),
        TokenKind::Delimiter(Delimiter::OpenPar),
        TokenKind::Operator(Operator::Neg),
        TokenKind::Identifier("someOtherIdent".to_string()),
        TokenKind::Delimiter(Delimiter::ClosePar),
        TokenKind::Delimiter(Delimiter::OpenBracket),
        TokenKind::Uint(4),
        TokenKind::Operator(Operator::Sub),
        TokenKind::Uint(1214),
        TokenKind::Special(Special::Comma),
        TokenKind::Identifier("a".to_string()),
        TokenKind::Operator(Operator::Mul),
        TokenKind::Int(-15),
        TokenKind::Delimiter(Delimiter::CloseBracket),
        TokenKind::Special(Special::RightArrow),
        TokenKind::Str("hello world 412".to_string()),
        TokenKind::Newline,
    ];

    let code = "identifier123+(-someOtherIdent) [  4  - 12__14  , a *   -15] -> 'hello world 412'";
    let mut lexer = Lexer::new(code);

    for exp in expected {
        let tok = lexer.advance().expect("expected an ok token");
        assert_eq!(tok.kind, exp);
    }

    assert!(lexer.is_empty())
}

#[test]
fn lex_program_chunks() {
    let code = "
fn f(a: int) -> int:
    return a

let i = 14
while i > 0:
    i -= 1

if __name__ == '__main__':
    print('hello world')
    ";
    let mut lexer = Lexer::new(code);

    let expected = vec![
        chunk!(
            line!(
                0,
                TokenKind::Keyword(Keyword::Fn),
                TokenKind::Identifier("f".to_string()),
                TokenKind::Delimiter(Delimiter::OpenPar),
                TokenKind::Identifier("a".to_string()),
                TokenKind::Special(Special::Colon),
                TokenKind::Type(Type::Int),
                TokenKind::Delimiter(Delimiter::ClosePar),
                TokenKind::Special(Special::RightArrow),
                TokenKind::Type(Type::Int),
                TokenKind::Special(Special::Colon)
            ),
            line!(
                4,
                TokenKind::Keyword(Keyword::Return),
                TokenKind::Identifier("a".to_string())
            )
        ),
        chunk!(line!(
            0,
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Identifier("i".to_string()),
            TokenKind::Operator(Operator::Eq),
            TokenKind::Uint(14)
        )),
        chunk!(
            line!(
                0,
                TokenKind::Keyword(Keyword::While),
                TokenKind::Identifier("i".to_string()),
                TokenKind::Operator(Operator::Gt),
                TokenKind::Uint(0),
                TokenKind::Special(Special::Colon)
            ),
            line!(
                4,
                TokenKind::Identifier("i".to_string()),
                TokenKind::Operator(Operator::SubEq),
                TokenKind::Uint(1)
            )
        ),
        chunk!(
            line!(
                0,
                TokenKind::Keyword(Keyword::If),
                TokenKind::Identifier("__name__".to_string()),
                TokenKind::Operator(Operator::EqEq),
                TokenKind::Str("__main__".to_string()),
                TokenKind::Special(Special::Colon)
            ),
            line!(
                4,
                TokenKind::Identifier("print".to_string()),
                TokenKind::Delimiter(Delimiter::OpenPar),
                TokenKind::Str("hello world".to_string()),
                TokenKind::Delimiter(Delimiter::ClosePar)
            )
        ),
    ];

    assert_chunks(expected, &mut lexer);
    assert!(lexer.is_empty())
}

fn assert_line(expected: Line, received: Line) {
    assert_eq!(expected.tokens.len(), received.tokens.len());
    assert_eq!(expected.indent, received.indent);

    for (exp, recv) in zip(expected.tokens, received.tokens) {
        assert_eq!(exp.kind, recv.kind)
    }
}

fn assert_chunks(expected: Vec<VecDeque<Line>>, lexer: &mut Lexer) {
    for exp_chunk in expected {
        let recv_chunk = lexer.advance_prog().expect("expected an ok chunk");
        assert_eq!(recv_chunk.len(), exp_chunk.len());
        for (exp, recv) in zip(exp_chunk, recv_chunk) {
            assert_line(exp, recv)
        }
    }
}

#[test]
fn lex_invalid_program_chunk() {
    let code = "
else:
    print('hello world')
";
    let mut lexer = Lexer::new(code);
    lexer.advance_prog().expect_err("expected an error");
}

#[test]
fn lex_var_def() {
    let code = "let a: bool = (fib(10) - 40) > 3";
    let mut lexer = Lexer::new(code);
    let expected = vec![chunk!(line!(
        0,
        TokenKind::Keyword(Keyword::Let),
        TokenKind::Identifier("a".to_string()),
        TokenKind::Special(Special::Colon),
        TokenKind::Type(Type::Bool),
        TokenKind::Operator(Operator::Eq),
        TokenKind::Delimiter(Delimiter::OpenPar),
        TokenKind::Identifier("fib".to_string()),
        TokenKind::Delimiter(Delimiter::OpenPar),
        TokenKind::Uint(10),
        TokenKind::Delimiter(Delimiter::ClosePar),
        TokenKind::Operator(Operator::Sub),
        TokenKind::Uint(40),
        TokenKind::Delimiter(Delimiter::ClosePar),
        TokenKind::Operator(Operator::Gt),
        TokenKind::Uint(3)
    ))];
    assert_chunks(expected, &mut lexer);
    assert!(lexer.is_empty());
}

#[test]
fn lex_func_def() {
    let code = "
fn fib(a: uint) -> uint:
    if a > 2:
        return fib(a - 2) + fib(a - 1)
    return 1
";
    let mut lexer = Lexer::new(code);
    let expected = vec![chunk!(
        line!(
            0,
            TokenKind::Keyword(Keyword::Fn),
            TokenKind::Identifier("fib".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Identifier("a".to_string()),
            TokenKind::Special(Special::Colon),
            TokenKind::Type(Type::Uint),
            TokenKind::Delimiter(Delimiter::ClosePar),
            TokenKind::Special(Special::RightArrow),
            TokenKind::Type(Type::Uint),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            4,
            TokenKind::Keyword(Keyword::If),
            TokenKind::Identifier("a".to_string()),
            TokenKind::Operator(Operator::Gt),
            TokenKind::Uint(2),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            8,
            TokenKind::Keyword(Keyword::Return),
            TokenKind::Identifier("fib".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Identifier("a".to_string()),
            TokenKind::Operator(Operator::Sub),
            TokenKind::Uint(2),
            TokenKind::Delimiter(Delimiter::ClosePar),
            TokenKind::Operator(Operator::Add),
            TokenKind::Identifier("fib".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Identifier("a".to_string()),
            TokenKind::Operator(Operator::Sub),
            TokenKind::Uint(1),
            TokenKind::Delimiter(Delimiter::ClosePar)
        ),
        line!(4, TokenKind::Keyword(Keyword::Return), TokenKind::Uint(1))
    )];
    assert_chunks(expected, &mut lexer);
    assert!(lexer.is_empty());
}

#[test]
fn lex_if_statement() {
    let code = "
if 2 > 3:
    print('one')
elif 3 > 4:
    print('two')
else:
    print('default')
";
    let mut lexer = Lexer::new(code);
    let expected = vec![chunk!(
        line!(
            0,
            TokenKind::Keyword(Keyword::If),
            TokenKind::Uint(2),
            TokenKind::Operator(Operator::Gt),
            TokenKind::Uint(3),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            4,
            TokenKind::Identifier("print".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Str("one".to_string()),
            TokenKind::Delimiter(Delimiter::ClosePar)
        ),
        line!(
            0,
            TokenKind::Keyword(Keyword::Elif),
            TokenKind::Uint(3),
            TokenKind::Operator(Operator::Gt),
            TokenKind::Uint(4),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            4,
            TokenKind::Identifier("print".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Str("two".to_string()),
            TokenKind::Delimiter(Delimiter::ClosePar)
        ),
        line!(
            0,
            TokenKind::Keyword(Keyword::Else),
            TokenKind::Special(Special::Colon)
        ),
        line!(
            4,
            TokenKind::Identifier("print".to_string()),
            TokenKind::Delimiter(Delimiter::OpenPar),
            TokenKind::Str("default".to_string()),
            TokenKind::Delimiter(Delimiter::ClosePar)
        )
    )];
    assert_chunks(expected, &mut lexer);
    assert!(lexer.is_empty());
}
