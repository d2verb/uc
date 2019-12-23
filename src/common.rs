use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

use crate::lexer::Lexer;

extern crate colored;
use colored::*;

pub fn compile(filename: &str) {
    let mut file = match File::open(filename) {
        Err(err) => {
            error(err.description());
            unreachable!();
        }
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(err) => error(err.description()),
        Ok(_) => {},
    }

    let mut lexer_iter = Lexer::new(&content).into_iter();
    while let Some(token) = lexer_iter.next() {
        println!("{:?}", token);
    }
}

pub fn error(msg: &str) {
    eprintln!("{}{}", "error:".red().bold(), msg);
    std::process::exit(1);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loc {
    pub lin: usize,
    pub col: usize,
    pub abs: usize,
}

impl Loc {
    pub fn new() -> Self {
        Loc {
            lin: 1,
            col: 1,
            abs: 0,
        }
    }

    pub fn advance(&mut self, byte: u8) {
        self.abs += 1;
        if byte == b'\n' {
            self.lin += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Annot<T> {
    pub val: T,
    pub loc: Loc,
}

impl<T> Annot<T> {
    pub fn new(val: T, loc: Loc) -> Self {
        Self {
            val,
            loc
        }
    }
}

pub fn is_whitespace(byte: u8) -> bool {
    match byte {
        b'\n' | b'\t' | b' ' => true,
        _ => false
    }
}

pub fn is_alphabetic(byte: u8) -> bool {
    match byte {
        b'a' ... b'z' => true,
        b'A' ... b'Z' => true,
        _ => false
    }
}

pub fn is_numeric(byte: u8) -> bool {
    match byte {
        b'0' ... b'9' => true,
        _ => false
    }
}

pub fn is_alphanumeric(byte: u8) -> bool {
    is_alphabetic(byte) | is_numeric(byte)
}
