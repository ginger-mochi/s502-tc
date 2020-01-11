use super::super::super::ir::program::Program;
use super::super::super::ir::token::{Token, TokenType};
use super::super::lexer;
use super::Parser;

impl Parser {
    pub fn new(source_file: String) -> Self {
        Parser {
            iter: lexer::tokenize(source_file),
            current: Token {
                pos: (String::new(), 1, 0),
                val: TokenType::Period,
            },
            prog: Program {},
        }
    }
}
