use std::collections::HashMap;
use std::iter::zip;

use dyn_clone::DynClone;
use num::BigRational;
use num::Complex;

mod context;
mod eval;

use context::Context;
use eval::eval_expr;

pub type Number = Complex<BigRational>;
pub type CalcResult = Result<Number, CalcError>;

pub trait Callable: DynClone {
    /// Returns the number of parameters a function has.
    fn arity(&self) -> usize;

    /// Call the function on some arguments.
    fn call(&self, args: &[Number], ctx: &Context) -> CalcResult;
}

impl Clone for Box<dyn Callable> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

#[derive(Clone)]
struct CalcFunc {
    bindings: Vec<String>,
    body: Expr,
}

impl Callable for CalcFunc {
    fn arity(&self) -> usize {
        self.bindings.len()
    }

    fn call(&self, args: &[Number], ctx: &Context) -> CalcResult {
        // Create evaluation scope
        let mut eval_scope = ctx.clone();
        let bindings = HashMap::from_iter(zip(self.bindings.iter().cloned(), args.iter().cloned()));
        eval_scope.add_scope(bindings);

        // Eval body in new scope
        eval_expr(&self.body, &eval_scope)
    }
}

#[derive(Debug, Clone)]
pub enum CalcError {
    NameNotFound,
    NameAlreadyBound,
    IncorrectArity,
}

#[derive(Clone)]
pub enum Atom {
    Symbol(String),
    Num(Number),
}

#[derive(Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Clone)]
pub enum UnaryOp {
    Negate,
}

#[derive(Clone)]
pub enum Expr {
    AtomExpr(Atom),
    UnaryExpr {
        op: UnaryOp,
        data: Box<Expr>,
    },
    BinaryExpr {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: BinaryOp,
    },
    FunctionCall {
        function: String,
        args: Vec<Expr>,
    },
    BlockExpr {
        stmts: Vec<Stmt>,
        final_expr: Box<Expr>,
    },
}

#[derive(Clone)]
pub enum Stmt {
    Assignment { name: String, value: Expr },
    ExprStmt(Expr),
}

fn main() {
    println!("hello, world");
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::eval::*;
    use super::*;

    #[test]
    fn test_atom_eval() {
        let mut ctx = Context::new();
        let one_two_three: Number = Complex::from_str("123").expect("failed to parse 123");
        ctx.bind_value("a".to_string(), one_two_three.clone())
            .expect("failed to bind value");

        let sym_atom = Atom::Symbol("a".to_string());
        let res = eval_atom(&sym_atom, &ctx).expect("failed to evaluate symbol atom");
        assert_eq!(one_two_three, res);

        let num_atom = Atom::Num(one_two_three.clone());
        let res = eval_atom(&num_atom, &ctx).expect("failed to evaluate number atom");
        assert_eq!(one_two_three, res);
    }

    #[test]
    fn test_expr_eval() {
        let ctx = Context::new();

        let data: Number = Complex::from_str("123").unwrap();
        let expected: Number = Complex::from_str("-123").unwrap();
        let expr = Expr::UnaryExpr {
            op: UnaryOp::Negate,
            data: Box::new(Expr::AtomExpr(Atom::Num(data))),
        };
        let res = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(res, expected);

        let lhs = Expr::AtomExpr(Atom::Num(Complex::from_str("10").unwrap()));
        let rhs = Expr::AtomExpr(Atom::Num(Complex::from_str("20").unwrap()));
        let add_expr = Expr::BinaryExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op: BinaryOp::Plus,
        };
        let expected = Complex::from_str("30").unwrap();
        let res = eval_expr(&add_expr, &ctx).unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_function_call() {
        let mut ctx = Context::new();

        let func = CalcFunc {
            bindings: vec!["x".to_string(), "y".to_string()],
            body: Expr::BinaryExpr {
                lhs: Box::new(Expr::AtomExpr(Atom::Symbol("x".to_string()))),
                rhs: Box::new(Expr::AtomExpr(Atom::Symbol("y".to_string()))),
                op: BinaryOp::Plus,
            },
        };
        ctx.bind_fn("f".to_string(), Box::new(func)).unwrap();

        let func_call = Expr::FunctionCall {
            function: "f".to_string(),
            args: vec![
                Expr::AtomExpr(Atom::Num(Complex::from_str("10").unwrap())),
                Expr::AtomExpr(Atom::Num(Complex::from_str("20").unwrap())),
            ],
        };

        let res = eval_expr(&func_call, &ctx).unwrap();
        let expected = Complex::from_str("30").unwrap();

        assert_eq!(res, expected);
    }
}
