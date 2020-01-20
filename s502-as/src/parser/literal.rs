use super::super::ir::*;

use nom::{
    branch::alt,
    character::complete::{char, digit1, hex_digit1, oct_digit1, one_of},
    combinator::{opt, verify},
    error::ErrorKind,
    multi::many1,
    sequence::{preceded, tuple},
    Err::Error,
    IResult,
};

fn num_prefix(input: &str) -> IResult<&str, Option<char>> {
    opt(alt((char('<'), char('>'))))(input)
}

fn hex_number(input: &str) -> IResult<&str, Literal> {
    let (input, (part, num)) = verify(
        tuple((num_prefix, preceded(char('$'), hex_digit1))),
        |n: &(Option<char>, &str)| n.1.len() < 5,
    )(input)?;
    if num.len() > 2 {
        Ok((input, Literal::Word(u16::from_str_radix(num, 16).unwrap())))
    } else {
        Ok((input, Literal::Byte(u8::from_str_radix(num, 16).unwrap())))
    }
}

fn oct_number(input: &str) -> IResult<&str, Literal> {
    let (input, num) = verify(preceded(char('@'), oct_digit1), |n: &str| n.len() < 7)(input)?;

    return match u16::from_str_radix(num, 8) {
        Ok(n) => {
            if n > 255 || num.len() > 3 {
                Ok((input, Literal::Word(n)))
            } else {
                Ok((input, Literal::Byte(n as u8)))
            }
        }
        Err(_) => Err(Error((input, ErrorKind::OctDigit))),
    };
}

fn bin_number(input: &str) -> IResult<&str, Literal> {
    let (input, read) = verify(preceded(char('%'), many1(one_of("01"))), |n: &Vec<char>| {
        n.len() < 17
    })(input)?;
    if read.len() > 16 {
        return Err(Error((input, ErrorKind::Verify)));
    }
    let num: String = read.into_iter().collect();

    if num.len() > 8 {
        Ok((input, Literal::Word(u16::from_str_radix(&num, 2).unwrap())))
    } else {
        Ok((input, Literal::Byte(u8::from_str_radix(&num, 2).unwrap())))
    }
}

fn dec_number(input: &str) -> IResult<&str, Literal> {
    let (input, num) = verify(digit1, |n: &str| n.len() < 6)(input)?;

    return match u16::from_str_radix(num, 10) {
        Ok(n) => {
            if n > 255 || num.len() > 3 {
                Ok((input, Literal::Word(n)))
            } else {
                Ok((input, Literal::Byte(n as u8)))
            }
        }
        Err(_) => Err(Error((input, ErrorKind::Digit))),
    };
}

pub fn literal(input: &str) -> IResult<&str, Literal> {
    alt((hex_number, dec_number, bin_number, oct_number))(input)
}

#[cfg(test)]
speculate! {
    it "reads hex" {
        let code0 = "$02 a";
        let code1 = "$ b";
        let code2 = "$00a c";
        let code3 = "25 d";
        let code4 = "$ffff e";
        let code5 = "$fafaf f";

        assert_eq!(hex_number(code0), Ok((" a", Literal::Byte(2))));
        assert_eq!(hex_number(code1), Err(Error((" b", ErrorKind::HexDigit))));
        assert_eq!(hex_number(code2), Ok((" c", Literal::Word(10))));
        assert_eq!(hex_number(code3), Err(Error(("25 d", ErrorKind::Char))));
        assert_eq!(hex_number(code4), Ok((" e", Literal::Word(65535))));
        assert_eq!(hex_number(code5), Err(Error(("$fafaf f", ErrorKind::Verify))));
    }

    it "reads oct" {
        let code0 = "@02";
        let code1 = "@";
        let code2 = "@0010";
        let code3 = "10";
        let code4 = "@177777";
        let code5 = "@777777";

        assert_eq!(oct_number(code0), Ok(("", Literal::Byte(2))));
        assert_eq!(oct_number(code1), Err(Error(("", ErrorKind::OctDigit))));
        assert_eq!(oct_number(code2), Ok(("", Literal::Word(8))));
        assert_eq!(oct_number(code3), Err(Error(("10", ErrorKind::Char))));
        assert_eq!(oct_number(code4), Ok(("", Literal::Word(65535))));
        assert_eq!(oct_number(code5), Err(Error(("", ErrorKind::OctDigit))));
    }

    it "reads bin" {
        let code0 = "%10";
        let code1 = "%";
        let code2 = "%0000001000000";
        let code3 = "25";
        let code4 = "%1111111111111111";
        let code5 = "%11111111111111111";

        assert_eq!(bin_number(code0), Ok(("", Literal::Byte(2))));
        assert_eq!(bin_number(code1), Err(Error(("", ErrorKind::OneOf))));
        assert_eq!(bin_number(code2), Ok(("", Literal::Word(64))));
        assert_eq!(bin_number(code3), Err(Error(("25", ErrorKind::Char))));
        assert_eq!(bin_number(code4), Ok(("", Literal::Word(65535))));
        assert_eq!(bin_number(code5), Err(Error(("%11111111111111111", ErrorKind::Verify))));
    }

    it "reads dec" {
        let code0 = "2";
        let code1 = "0008";
        let code2 = "65535";
        let code3 = "65536";
        let code4 = "100000";

        assert_eq!(dec_number(code0), Ok(("", Literal::Byte(2))));
        assert_eq!(dec_number(code1), Ok(("", Literal::Word(8))));
        assert_eq!(dec_number(code2), Ok(("", Literal::Word(65535))));
        assert_eq!(dec_number(code3), Err(Error(("", ErrorKind::Digit))));
        assert_eq!(dec_number(code4), Err(Error(("100000", ErrorKind::Verify))));
    }

    it "reads literals " {
        let code0 = "2";
        let code1 = "$101";
        let code2 = "@0001";
        let code3 = "%0101";

        assert_eq!(literal(code0), Ok(("", Literal::Byte(2))));
        assert_eq!(literal(code1), Ok(("", Literal::Word(257))));
        assert_eq!(literal(code2), Ok(("", Literal::Word(1))));
        assert_eq!(literal(code3), Ok(("", Literal::Byte(5))));
    }
}
