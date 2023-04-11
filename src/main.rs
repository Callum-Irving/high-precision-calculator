use std::{
    fmt::Display,
    io::{self, Write},
};

use eval::CalcValue;
//use num::BigRational;
//use num::Complex;
//use rug::Complex;
//use dashu::Decimal;
use astro_float::{BigFloat, RoundingMode};

use crate::context::Context;

mod ast;
mod context;
mod eval;
mod parser;

pub type Number = BigFloat;
pub type CalcResult = Result<Number, CalcError>;
// Preicison of floating point numbers
pub const PREC: usize = 128;
pub const RM: RoundingMode = RoundingMode::ToEven;

#[derive(Debug, Clone)]
pub enum CalcError {
    NameNotFound,
    NameAlreadyBound,
    IncorrectArity,
    ParseNum,
    ParseError,
    IOError,
}

impl Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn read() -> Result<ast::Stmt, CalcError> {
    print!("calculator> ");
    io::stdout().flush().map_err(|_| CalcError::IOError)?;
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .map_err(|_| CalcError::IOError)?;
    //let buf = buf.trim_end().to_string();

    let (_, stmt) = parser::parse_stmt(&buf).map_err(|_| CalcError::ParseError)?;

    Ok(stmt)
}

fn eval(stmt: ast::Stmt, ctx: &mut Context) -> Result<CalcValue, CalcError> {
    eval::eval_stmt(&stmt, ctx)
}

fn main() {
    let mut ctx = Context::new();

    loop {
        let input = read().unwrap();
        let result = eval(input, &mut ctx).unwrap();
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use crate::PREC;

    use super::ast::*;
    use super::context::Context;
    use super::eval::*;

    //use rug::Complex;
    //use dashu::Decimal;
    use astro_float::BigFloat;

    #[test]
    fn test_atom_eval() {
        let num = BigFloat::from_i32(123, PREC);

        let mut ctx = Context::new();
        ctx.bind_value("a".to_string(), num.clone())
            .expect("failed to bind value");

        let sym_atom = Atom::Symbol("a".to_string());
        let res = eval_atom(&sym_atom, &ctx).expect("failed to evaluate symbol atom");
        assert_eq!(num, res);

        let num_atom = Atom::Num(num.clone());
        let res = eval_atom(&num_atom, &ctx).expect("failed to evaluate number atom");
        assert_eq!(num, res);
    }

    #[test]
    fn test_expr_eval() {
        let num = BigFloat::from_i32(123, PREC);
        let num2 = BigFloat::from_i32(-123, PREC);
        let num3 = BigFloat::from_i32(10, PREC);
        let num4 = BigFloat::from_i32(20, PREC);
        let num5 = BigFloat::from_i32(30, PREC);

        let ctx = Context::new();

        let expr = Expr::UnaryExpr {
            op: UnaryOp::Negate,
            data: Box::new(Expr::AtomExpr(Atom::Num(num))),
        };
        let res = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(res, num2);

        let lhs = Expr::AtomExpr(Atom::Num(num3));
        let rhs = Expr::AtomExpr(Atom::Num(num4));
        let add_expr = Expr::BinaryExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op: BinaryOp::Plus,
        };
        let res = eval_expr(&add_expr, &ctx).unwrap();
        assert_eq!(res, num5);
    }

    #[test]
    fn test_function_call() {
        let num1 = BigFloat::from_i32(10, PREC);
        let num2 = BigFloat::from_i32(20, PREC);
        let num3 = BigFloat::from_i32(30, PREC);

        let mut ctx = Context::new();

        let func = CalcFunc::new(
            vec!["x".to_string(), "y".to_string()],
            Expr::BinaryExpr {
                lhs: Box::new(Expr::AtomExpr(Atom::Symbol("x".to_string()))),
                rhs: Box::new(Expr::AtomExpr(Atom::Symbol("y".to_string()))),
                op: BinaryOp::Plus,
            },
        );
        ctx.bind_fn("f".to_string(), Box::new(func)).unwrap();

        let func_call = Expr::FunctionCall {
            function: "f".to_string(),
            args: vec![
                Expr::AtomExpr(Atom::Num(num1)),
                Expr::AtomExpr(Atom::Num(num2)),
            ],
        };

        let res = eval_expr(&func_call, &ctx).unwrap();

        assert_eq!(res, num3);
    }
}
