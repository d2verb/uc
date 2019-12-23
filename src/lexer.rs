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
                b if is_numeric(b) => {
                    return self.tokenize_number();
                }
                b if is_alphabetic(b) => {
                    return self.tokenize_name();
                }
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
                    error(&format!("{}:{}: unknown byte found: '{}'",
                                   self.loc.lin,
                                   self.loc.col,
                                   byte as char));
                    unreachable!();
                }
            };
            self.loc.advance(byte);
            token
        })
    }

    fn tokenize_number(&mut self) -> Token {
        let start_loc = self.loc.clone();
        let mut number: i64 = 0;
        while let Some(byte) = self.byte_at(self.loc.abs) {
            if !is_numeric(byte) {
                break;
            }
            number = number * 10 + (byte - b'0') as i64;
            self.loc.advance(byte);
        }
        Token::new(TokenKind::Number(number), start_loc)
    }

    fn tokenize_name(&mut self) -> Token {
        let start_loc = self.loc.clone();
        let mut name: String = String::new();
        while let Some(byte) = self.byte_at(self.loc.abs) {
            if !is_alphanumeric(byte) {
                break;
            }
            name.push(byte as char);
            self.loc.advance(byte);
        }
        match name.as_str() {
            "while" => Token::new(TokenKind::While, start_loc),
            "define" => Token::new(TokenKind::Define, start_loc),
            "if" => Token::new(TokenKind::If, start_loc),
            "else" => Token::new(TokenKind::Else, start_loc),
            _ => Token::new(TokenKind::Name(name), start_loc)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(byte) = self.byte_at(self.loc.abs) {
            if !is_whitespace(byte) {
                break;
            }
            self.loc.advance(byte);
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

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::token::{Token, TokenKind};
    use crate::common::Loc;

    #[test]
    fn basic() {
        let code = "define i; i = 0; while (i < 10) { i = i + 1; }";
        let mut lexer_iter = Lexer::new(&code).into_iter();
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Define);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Name("i".to_string()));
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Semicolon);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Name("i".to_string()));
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Equal);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Number(0));
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Semicolon);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::While);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::LParen);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Name("i".to_string()));
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::LessThan);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Number(10));
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::RParen);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::LBrace);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Name("i".to_string()));
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Equal);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Name("i".to_string()));
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Plus);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Number(1));
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::Semicolon);
        assert_eq!(lexer_iter.next().unwrap().val, TokenKind::RBrace);
        assert_eq!(lexer_iter.next(), None);
    }

    #[test]
    fn location() {
        let code = "define i;\ni = 0;";
        let mut lexer_iter = Lexer::new(&code).into_iter();
        assert_eq!(lexer_iter.next().unwrap().loc, Loc { lin: 1, col: 1, abs: 0 });  // define
        assert_eq!(lexer_iter.next().unwrap().loc, Loc { lin: 1, col: 8, abs: 7 });  // i
        assert_eq!(lexer_iter.next().unwrap().loc, Loc { lin: 1, col: 9, abs: 8 });  // ;
        assert_eq!(lexer_iter.next().unwrap().loc, Loc { lin: 2, col: 1, abs: 10 }); // i
        assert_eq!(lexer_iter.next().unwrap().loc, Loc { lin: 2, col: 3, abs: 12 }); // =
        assert_eq!(lexer_iter.next().unwrap().loc, Loc { lin: 2, col: 5, abs: 14 }); // 0
        assert_eq!(lexer_iter.next().unwrap().loc, Loc { lin: 2, col: 6, abs: 15 }); // ;
        assert_eq!(lexer_iter.next(), None);
    }
}
