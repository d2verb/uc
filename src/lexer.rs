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
            let token = match byte {
                b if is_numeric(b) => self.new_token(TokenKind::Number(1)),
                b if is_alphabetic(b) => self.new_token(TokenKind::Number(1)),
                b'+' => self.new_token(TokenKind::Plus),
                b'-' => self.new_token(TokenKind::Minus),
                b'*' => self.new_token(TokenKind::Asterisk),
                b'/' => self.new_token(TokenKind::Slash),
                b'(' => self.new_token(TokenKind::LParen),
                b')' => self.new_token(TokenKind::RParen),
                b'{' => self.new_token(TokenKind::LBrace),
                b'}' => self.new_token(TokenKind::RBrace),
                b';' => self.new_token(TokenKind::Semicolon),
                b'=' => self.new_token(TokenKind::Equal),
                b'<' => self.new_token(TokenKind::LessThan),
                _ => {
                    error(&format!("unknown byte found: '{}'", byte as char));
                    unreachable!();
                }
            };
            token
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

    fn new_token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.loc.clone())
    }
}

impl<'input> Iterator for IntoIter<'input> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.tokenize()
    }
}
