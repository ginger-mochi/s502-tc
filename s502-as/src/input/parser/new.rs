use super::super::super::ir::program::Program;
use super::super::super::ir::token::{Token, TokenType};
use super::super::lexer;
use super::Parser;

impl Parser {
    pub fn new(source_file: String) -> Self {
        Parser {
            iter: lexer::tokenize(source_file),
            current: Token {
                pos: (String::new(), 0u32),
                val: TokenType::EndFile,
            },
            prog: Program {},
        }
    }
}
