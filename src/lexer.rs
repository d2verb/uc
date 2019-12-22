use crate::token::{Token, TokenKind};
use crate::common::{error, Loc};

#[derive(Debug, PartialEq, Eq)]
pub struct Lexer<'input> {
    code: &'input str,
    loc: Loc,
}

impl<'input> Lexer<'input> {
    pub fn new(code: &'input str) -> Self {
        Lexer {
            code,
            loc: Loc::new(),
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
