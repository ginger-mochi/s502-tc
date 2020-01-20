use super::super::ir::program::*;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{
        alpha1, alphanumeric1, anychar, char, digit1, hex_digit1, line_ending, none_of,
        not_line_ending, oct_digit1, one_of,
    },
    combinator::{iterator, not, opt, verify},
    error::ErrorKind,
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated, tuple},
    Err::Error,
    IResult,
};

// fn ident(input: &str) -> IResult<&str, Program> {

// }
