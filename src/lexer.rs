use crate::token::{Token, TokenKind};
use crate::common::{error, Loc};
use crate::common::{is_whitespace, is_alphabetic, is_numeric, is_alphanumeric};

#[derive(Debug, PartialEq, Eq)]
pub struct Lexer<'input> {
    code: &'input str,
    loc: Loc,
}

pub struct IntoIter<'input>(Lexer<'input>);

impl<'input> Lexer<'input> {
    pub fn new(code: &'input str) -> Self {
        Lexer {
            code,
            loc: Loc::new(),
        }
    }

    pub fn into_iter(self) -> IntoIter<'input> {
        IntoIter(self)
    }

    pub fn tokenize(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.byte_at(self.loc.abs).map(|byte| {
            match byte {
                b if is_numeric(b) => Token::new(TokenKind::Number(1), self.loc.clone()),
                _ => {
                    error(&format!("unknown byte found: '{}'", byte as char));
                    unreachable!();
                }
            }
        })
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.byte_at(self.loc.abs) {
                None => break,
                Some(byte) if !is_whitespace(byte) => break,
                Some(byte) => self.loc.advance(byte),
            }
        }
    }

    fn byte_at(&self, index: usize) -> Option<u8> {
        if self.code.as_bytes().len() <= index {
            None
        } else {
            Some(self.code.as_bytes()[index])
        }
    }
}

impl<'input> Iterator for IntoIter<'input> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.tokenize()
    }
}
