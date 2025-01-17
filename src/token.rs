use crate::common::Annot;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Number(i64),
    Name(String),
    While,
    If,
    Else,
    Define,
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Equal,
    LessThan,
}

pub type Token = Annot<TokenKind>;
