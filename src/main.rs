use std::io::{self, Write};

//use num::BigRational;
//use num::Complex;
use rug::Complex;

use crate::context::Context;

mod ast;
mod context;
mod eval;
mod parser;

pub type Number = Complex;
pub type CalcResult = Result<Number, CalcError>;
// Preicison of floating point numbers
pub const PREC: u32 = 53;

#[derive(Debug, Clone)]
pub enum CalcError {
    NameNotFound,
    NameAlreadyBound,
    IncorrectArity,
    ParseNum,
    ParseError,
    IOError,
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

fn eval(stmt: ast::Stmt, ctx: &mut Context) -> CalcResult {
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
    use super::ast::*;
    use super::context::Context;
    use super::eval::*;

    use rug::Complex;

    fn rug_float(r: f64, i: f64) -> Complex {
        Complex::with_val(53, (r, i))
    }

    #[test]
    fn test_atom_eval() {
        let num = rug_float(123f64, 0f64);

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
        // Create complex number 123 + 0i
        let num = rug_float(123f64, 0f64);

        // Create complex number -123 + 0i
        let num2 = rug_float(-123f64, 0f64);

        // Create complex number 10 + 0i
        let num3 = rug_float(10f64, 0f64);

        // Create complex number 20 + 0i
        let num4 = rug_float(20f64, 0f64);

        // Create complex number 30 + 0i
        let num5 = rug_float(30f64, 0f64);

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
        // Create complex number 10 + 0i
        let num1 = rug_float(10f64, 0f64);

        // Create complex number 20 + 0i
        let num2 = rug_float(20f64, 0f64);

        // Create complex number 30 + 0i
        let num3 = rug_float(30f64, 0f64);

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
