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
    #[test]
    fn test_atom_eval() {
        todo!()
    }
}
