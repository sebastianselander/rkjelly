#![allow(unused)]

use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    text: Chars<'a>,
    cursor: usize,
    column: usize,
    row: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(str: String) -> Self {
        Self {
            text: str.chars(),
            cursor: 0,
            column: 1,
            row: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    EOF,
    LPAREN,
    RPAREN,
    BANG,
    SEMICOLON,
    AMPERSAND,
    BAR,
    OR,
    AND,
    CD,
    HELP,
    QUIT,
    TIME,
    TEXT(String),
}
