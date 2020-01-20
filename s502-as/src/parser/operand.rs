use super::super::ir::*;
use super::line::ident;
use super::literal::literal;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, tag_no_case},
    character::complete::{
        alpha1, alphanumeric1, anychar, char, digit1, hex_digit1, line_ending, none_of,
        not_line_ending, oct_digit1, one_of, space1,
    },
    combinator::{iterator, not, opt, verify},
    error::ErrorKind,
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated, tuple},
    Err::Error,
    IResult,
};

fn reference(input: &str) -> IResult<&str, OperandVal> {
    ident(input).map(|(inp, s)| (inp, OperandVal::Ref(s)))
}

fn value(input: &str) -> IResult<&str, OperandVal> {
    alt((literal, reference))(input).map(||)
}

fn imme(input: &str) -> IResult<&str, Operand> {
    preceded(char('#'), value)(input).map(|(inp, val)| {
        (
            inp,
            Operand {
                mode: AddressMode::Imme,
                val: val,
            },
        )
    })
}

fn string(input: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        escaped(none_of("\"\\"), '\\', one_of("\"\\")),
        char('"'),
    )(input)
}

#[cfg(test)]
speculate! {
    use AddressMode::*;

    it "reads_immediates" {
        let code0 = "#2";
        let code1 = "$25";
        let code2 = "#@10";
        let code3 = "%101";

        assert_eq!(imme(code0), Ok(("", Operand {
            mode: Imme,
            val: OperandVal::Byte(2)
        })));
        assert_eq!(imme(code1), Err(Error(("$25", ErrorKind::Char))));
        assert_eq!(imme(code2), Ok(("", Operand {
            mode: Imme,
            val: OperandVal::Byte(0o10)
        })));
        assert_eq!(imme(code3), Err(Error(("%101", ErrorKind::Char))));
    }

    it "reads strings" {
        let code0 = "\"c o_de\"";
        let code1 = "\"code1";
        let code2 = "code2";

        assert_eq!(string(code0), Ok(("", "c o_de")));
        assert_eq!(string(code1), Err(Error(("", ErrorKind::Char))));
        assert_eq!(string(code2), Err(Error(("code2", ErrorKind::Char))));
    }
}
