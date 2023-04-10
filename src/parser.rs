use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::{char, digit1, multispace0, satisfy};
use nom::combinator::{cut, map, map_res, opt, recognize};
use nom::multi::{fold_many0, separated_list0};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use rug::Complex;
use rug::Float;

use crate::CalcError;
use crate::Number;
use crate::{ast, PREC};

pub fn parse_stmt(input: &str) -> IResult<&str, ast::Stmt> {
    terminated(
        alt((
            map(
                tuple((
                    delimited(multispace0, parse_symbol, multispace0),
                    char('='),
                    delimited(multispace0, parse_expr, multispace0),
                )),
                |(name, _, value)| ast::Stmt::Assignment { name, value },
            ),
            map(delimited(multispace0, parse_expr, multispace0), |expr| {
                ast::Stmt::ExprStmt(expr)
            }),
        )),
        char(';'),
    )(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, ast::Expr> {
    let (input, first_term) = parse_term(input)?;
    // I cannot for the life of me figure out why I need to bind this or clone first_term but here we are.
    let x = fold_many0(
        tuple((parse_addop, parse_term)),
        || first_term.clone(),
        |acc, (op, term)| ast::Expr::BinaryExpr {
            lhs: Box::new(acc),
            rhs: Box::new(term),
            op,
        },
    )(input);
    x
}

fn parse_term(input: &str) -> IResult<&str, ast::Expr> {
    let (input, first_factor) = parse_exponent(input)?;
    let x = fold_many0(
        tuple((parse_mulop, parse_exponent)),
        || first_factor.clone(),
        |acc, (op, factor)| ast::Expr::BinaryExpr {
            lhs: Box::new(acc),
            rhs: Box::new(factor),
            op,
        },
    )(input);
    x
}

fn parse_exponent(input: &str) -> IResult<&str, ast::Expr> {
    let (input, first_base) = parse_parens(input)?;
    let x = fold_many0(
        tuple((parse_mulop, parse_exponent)),
        || first_base.clone(),
        |base, (op, expt)| ast::Expr::BinaryExpr {
            lhs: Box::new(base),
            rhs: Box::new(expt),
            op,
        },
    )(input);
    x
}

fn parse_parens(input: &str) -> IResult<&str, ast::Expr> {
    // Try parentheses delimited expression, otherwise try function call, otherwise try atom.
    alt((
        delimited(
            preceded(multispace0, char('(')),
            parse_expr,
            terminated(char(')'), multispace0),
        ),
        parse_function_call,
        map(parse_atom, |atom| ast::Expr::AtomExpr(atom)),
    ))(input)
}

fn parse_function_call(input: &str) -> IResult<&str, ast::Expr> {
    map(
        tuple((
            parse_symbol,
            delimited(char('('), separated_list0(char(','), parse_expr), char(')')),
        )),
        |(function, args)| ast::Expr::FunctionCall { function, args },
    )(input)
}

fn parse_addop(input: &str) -> IResult<&str, ast::BinaryOp> {
    map(alt((char('+'), char('-'))), |c: char| match c {
        '+' => ast::BinaryOp::Plus,
        '-' => ast::BinaryOp::Minus,
        _ => unreachable!(),
    })(input)
}

fn parse_mulop(input: &str) -> IResult<&str, ast::BinaryOp> {
    map(alt((char('*'), char('/'))), |c: char| match c {
        '*' => ast::BinaryOp::Times,
        '/' => ast::BinaryOp::Divide,
        _ => unreachable!(),
    })(input)
}

fn parse_atom(input: &str) -> IResult<&str, ast::Atom> {
    delimited(
        multispace0,
        alt((
            map(parse_number, |num| ast::Atom::Num(num)),
            map(parse_symbol, |sym| ast::Atom::Symbol(sym)),
        )),
        multispace0,
    )(input)
}

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

fn parse_number(input: &str) -> IResult<&str, Number> {
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
            Ok(Complex::with_val(PREC, (num, 0)))
        }
    })(input)
}

fn parse_symbol(input: &str) -> IResult<&str, String> {
    map(
        recognize(tuple((
            satisfy(|c| is_symbol_character(c) && !c.is_ascii_digit()),
            take_while(is_symbol_character),
        ))),
        |s: &str| s.to_string(),
    )(input)
}

fn is_symbol_character(c: char) -> bool {
    c != '(' && c != ')' && c != '"' && c != ';' && c != ',' && !c.is_whitespace()
}

#[cfg(test)]
mod tests {
    use crate::{context::Context, eval, PREC};

    use super::*;

    use rug::Complex;

    fn rug_num(r: f64, i: f64) -> Complex {
        Complex::with_val(PREC, (r, i))
    }

    #[test]
    fn test_parse_number() {
        recognize_number("123i").unwrap();
        recognize_number("123").unwrap();
        recognize_number("123.456").unwrap();
        recognize_number("123E10").unwrap();
        recognize_number("-12.45E-10i").unwrap();

        let (_rest, num) = parse_number("123i").unwrap();
        assert_eq!(num, rug_num(0_f64, 123_f64));
        let (_rest, num) = parse_number("10e10").unwrap();
        assert_eq!(num, rug_num(10e10_f64, 0_f64));
        let (_rest, num) = parse_number("-12.45E-10i").unwrap();
        assert_eq!(num, rug_num(0_f64, -12.45e-10_f64));
    }

    #[test]
    fn test_parse_expr() {
        let (_rest, expr) = parse_expr("123 + 456 + 7").unwrap();

        let ctx = Context::new();

        assert_eq!(
            eval::eval_expr(&expr, &ctx).unwrap(),
            rug_num(123_f64 + 456_f64 + 7_f64, 0_f64)
        );
    }

    #[test]
    fn test_parse_fn_call() {
        let (_rest, _expr) = parse_function_call("g(x,y)").unwrap();
    }

    #[test]
    fn test_parse_stmt() {
        let (_rest, _stmt) = parse_stmt("1 + 2;").unwrap();
    }
}
