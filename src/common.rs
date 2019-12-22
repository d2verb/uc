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

    let mut lexer = Lexer::new(&content);
}

pub fn error(msg: &str) {
    eprintln!("{} {}", "error:".red().bold(), msg);
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

    pub fn advance(&mut self, ch: u8) {
        self.abs += 1;
        if ch == b'\n' {
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
