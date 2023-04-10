use std::collections::HashMap;
use std::iter::zip;

use dyn_clone::DynClone;

use crate::context::Context;
use crate::eval::eval_expr;
use crate::{CalcResult, Number};

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

#[derive(Clone, Debug)]
pub struct CalcFunc {
    bindings: Vec<String>,
    body: Expr,
}

impl CalcFunc {
    pub fn new(bindings: Vec<String>, body: Expr) -> CalcFunc {
        CalcFunc { bindings, body }
    }
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

#[derive(Clone, Debug)]
pub enum Atom {
    Symbol(String),
    Num(Number),
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
    Times,
    Divide,
    Power,
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Negate,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Stmt {
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Expr,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    ExprStmt(Expr),
}
