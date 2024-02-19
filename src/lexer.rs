#![allow(unused)]

use std::{str::Chars, collections::VecDeque};

#[derive(Debug)]
pub struct Lexer {
    text: VecDeque<char>,
    column: usize,
    row: usize,
}

impl Lexer {
    pub fn new(str: String) -> Self {
        Self {
            text: str.chars().collect(),
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
