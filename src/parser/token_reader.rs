
use crate::errors::{ParserErrors, 
                    span::{Span, Position}};

use crate::lexer::tokens::{Token, TokenKind, Keyword};


use std::collections::VecDeque;

pub struct TokenReader<'a> {
    tokens: VecDeque<Token>, 
    span: &'a Span,
    pos: Position,
}

impl TokenReader<'_> {
    pub fn new(tokens_: VecDeque<Token>, span_: &Span) -> TokenReader {
        TokenReader {
            tokens: tokens_,
            span: span_,
            pos: Position::new(1, 1),
        }
    }

    // returns the end position of the last removed element
    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn peek(&self) -> Option<Token> {
        self.tokens.front().cloned()
    }

    pub fn is_empty(&mut self) -> bool {
        self.tokens.is_empty()
    }

    pub fn advance(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            self.pos = token.end().clone();
        }

        self.tokens.pop_front()
    }

    pub fn advance_while(&mut self, condition: fn(&TokenKind) -> bool) -> VecDeque<Token> {
        let mut result = VecDeque::<Token>::new();
        while !self.is_empty() && condition(self.peek().unwrap().get_kind()) { 
            result.push_back(self.advance().unwrap()); 
        }
        result
    }

    // TODO! make it return a proper ast node
    pub fn advance_node(&mut self) -> Option<VecDeque<Token>>  {
        // check the next token:
        // keyword 'let' => new variable - end newline
        // keyword 'if'  => new if       - end closing delimiter
        // default => new statement      - end newline
        
        if let Some(token) = self.peek() {
            match token.get_kind() {
                TokenKind::Keyword(Keyword::Let) => {
                    let mut result = VecDeque::<Token>::new();
                    result.push_back(token.clone());
                    result.append(&mut self.advance_while(|tk| *tk != TokenKind::Newline));
                    self.advance(); // remove the newline
                    return Some(result);
                }, 

                _ => return None,
            }

        } else {
            panic!("Error: TokenReader: advance(): advancing with no tokens.");
        }
    }

    fn expect_check(&mut self, 
                    current: Token, 
                    expected: TokenKind,
                    condition: fn (&Token, TokenKind) -> bool 
    )-> Result<Token, ()> {
        if condition(&current, expected) {
            return Ok( self.advance().unwrap() );
        } else {
            ParserErrors::UnexpectedToken::msg(&current, self.span);
        }
        Err(())
    }

    // advances if the token matches, else throws an error
    pub fn expect(&mut self, expected: TokenKind) -> Result<Token, ()> {
        if let Some(token) = self.peek() {
            // std::mem:discriminant() makes it so we can check only the outer enum variant
            // for example:
            // TokenKind::Identifier('main') is equal to TokenKind::Identifier('')
            match token.get_kind() {
                TokenKind::Keyword(_) => return self.expect_check(token, expected, |curr, exp| *curr.get_kind() == exp),
                _ => return self.expect_check(token, expected, |curr, exp| std::mem::discriminant(curr.get_kind()) == std::mem::discriminant(&exp)),
            }

        } else {
            panic!("Error: TokenReader: expect(): expecting from an empty reader.");
        }
    }
}
