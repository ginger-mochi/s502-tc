use super::super::ir::program::Program;
use super::super::ir::token::Token;

mod new;
mod program;

struct Parser {
    // to iterate over tokens
    iter: std::iter::Peekable<std::vec::IntoIter<Token>>,
    // current token read from iter
    current: Token,
    // program to construct
    prog: Program,
}

pub fn parse_program(source_file: String) -> Program {
    Parser::new(source_file).program()
}
