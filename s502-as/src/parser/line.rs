use super::super::ir::*;
use super::number::number;

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

fn line_end(input: &str) -> IResult<&str, Option<&str>> {
    opt(line_ending)(input)
}

fn comment(input: &str) -> IResult<&str, ()> {
    tuple((tag(";"), not_line_ending, line_end))(input).map(|(inp, _)| (inp, ()))
}

pub fn ident(input: &str) -> IResult<&str, String> {
    verify(
        tuple((
            verify(anychar, |c: &char| c.is_alphabetic()),
            many0(alt((
                char('_'),
                verify(anychar, |c: &char| c.is_alphanumeric()),
            ))),
        )),
        |t: &(char, Vec<char>)| t.1.len() < 31,
    )(input)
    .map(|(inp, (h, t))| {
        let mut s = h.to_string();
        s.push_str(&t.into_iter().collect::<String>());
        (inp, s)
    })
}

fn mac(input: &str) -> IResult<&str, Macro> {
    tuple((ident, space1, tag_no_case("equ"), space1, number))(input).map(
        |(inp, (name, _, _, _, num))| {
            (
                inp,
                Macro {
                    id: name.to_string(),
                    val: num,
                },
            )
        },
    )
}

#[rustfmt::skip]
fn reference(input: &str) -> IResult<&str, Label> {
    tuple((
        opt(ident),
        opt(tuple((
            tag("."),
            ident))),
    ))(input)
    .map(|(input, (parent, child))| (input, Label {
        parent: parent.map(|p| p.to_string()),
        child: child.map(|(_, c)| c.to_string()),
    }))
}

#[rustfmt::skip]
fn label(input: &str) -> IResult<&str, Label> {
    tuple((
        opt(ident),
        opt(tuple((
            tag("."),
            ident))),
        char(':')
    ))(input)
    .map(|(input, (parent, child, _))| (input, Label {
        parent: parent.map(|r| r.to_string()),
        child: child.map(|(_, c)| c.to_string()),
    }))
}

#[cfg(test)]
speculate! {
    it "reads comments" {
        let code0 = "; test";
        let code1 = "test";

        assert_eq!(comment(code0), Ok(("", ())));
        assert_eq!(comment(code1), Err(Error(("test", ErrorKind::Tag))));
    }

    it "reads identifiers" {
        let code0 = "hello";
        let code1 = "abcdefghijklmnopqrstuvqxyzabcdef";
        assert_eq!(ident(code0), Ok(("", "hello".to_string())));
        assert_eq!(ident(code1), Err(Error(("abcdefghijklmnopqrstuvqxyzabcdef", ErrorKind::Verify))));
    }

    it "reads labels" {
        let code0 = "test:";
        let code1 = "idk";
        let code2 = "one.two:";
        let code3 = ".three:";
        let code4 = ".five";

        assert_eq!(label(code0), Ok(("", Label {
            parent: Some(String::from("test")),
            child: None,
        })));
        assert_eq!(label(code1), Err(Error(("", ErrorKind::Char))));
        assert_eq!(label(code2), Ok(("", Label {
            parent: Some(String::from("one")),
            child: Some(String::from("two")),
        })));
        assert_eq!(label(code3), Ok(("", Label {
            parent: None,
            child: Some(String::from("three")),
        })));
        assert_eq!(label(code4), Err(Error(("", ErrorKind::Char))));
    }

    it "reads macros" {
        let code0 = "one equ 20";
        let code1 = "two equ $010";
        let code2 = "three adc #5";
        let code3 = "equ 2";

        assert_eq!(mac(code0), Ok(("", Macro {
            id: "one".to_string(),
            val: OperandVal::Byte(20),
        })));
        assert_eq!(mac(code1), Ok(("", Macro {
            id: "two".to_string(),
            val: OperandVal::Word(16),
        })));
        assert_eq!(mac(code2), Err(Error(("adc #5", ErrorKind::Tag))));
        assert_eq!(mac(code3), Err(Error(("2", ErrorKind::Tag))));
    }
}
