use super::super::super::ir::program::Program;
use super::Parser;

impl Parser {
    pub fn program(self) -> Program {
        self.prog
    }
}
