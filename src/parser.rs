use std::str::FromStr;

use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{cut, map, map_res, opt, recognize};
use nom::sequence::{pair, tuple};
use nom::IResult;
use rug::float::ParseFloatError;
use rug::Complex;
use rug::Float;

use crate::Number;
use crate::{ast, PREC};
use crate::{CalcError, CalcResult};

fn recognize_number(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        opt(alt((char('+'), char('-')))),
        alt((
            map(tuple((digit1, opt(pair(char('.'), opt(digit1))))), |_| ()),
            map(tuple((char('.'), digit1)), |_| ()),
        )),
        opt(tuple((
            alt((char('e'), char('E'))),
            opt(alt((char('+'), char('-')))),
            cut(digit1),
        ))),
        opt(char('i')),
    )))(input)
}

pub fn parse_number(input: &str) -> IResult<&str, Number> {
    map_res(recognize_number, |s: &str| {
        // If last character is 'i', make it imaginary.
        // Otherwise, make it real.
        if s.chars().last().unwrap() == 'i' {
            let l = s.len();
            let sub = s.get(0..l - 1).ok_or(CalcError::ParseNum)?;
            let num = Float::parse(sub).map_err(|_| CalcError::ParseNum)?;
            Ok::<Complex, CalcError>(Complex::with_val(PREC, (0, num)))
        } else {
            let num = Float::parse(s).map_err(|_| CalcError::ParseNum)?;
            Ok::<Complex, CalcError>(Complex::with_val(PREC, (num, 0)))
        }
    })(input)
}

#[cfg(test)]
mod tests {
    use crate::PREC;

    use super::parse_number;
    use super::recognize_number;
    use rug::Complex;

    #[test]
    fn test_parse_number() {
        recognize_number("123i").unwrap();
        recognize_number("123").unwrap();
        recognize_number("123.456").unwrap();
        recognize_number("123E10").unwrap();
        recognize_number("-12.45E-10i").unwrap();

        let (_rest, num) = parse_number("123i").unwrap();
        assert_eq!(num, Complex::with_val(PREC, (0, 123)));
        let (_rest, num) = parse_number("10e10").unwrap();
        assert_eq!(num, Complex::with_val(PREC, (10e10f64, 0)));
        let (_rest, num) = parse_number("-12.45E-10i").unwrap();
        assert_eq!(num, Complex::with_val(PREC, (0, -12.45e-10_f64)));
    }
}
