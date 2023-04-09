use std::collections::HashMap;

use crate::context::Context;
use crate::{Atom, BinaryOp, Expr, Stmt, UnaryOp};
use crate::{CalcError, CalcResult, Number};

pub fn eval_atom(atom: &Atom, ctx: &Context) -> CalcResult {
    match atom {
        Atom::Num(num) => Ok(num.clone()),
        Atom::Symbol(name) => ctx.lookup_value(name),
    }
}

pub fn eval_expr(expr: &Expr, ctx: &Context) -> CalcResult {
    match expr {
        Expr::AtomExpr(atom) => eval_atom(atom, ctx),
        Expr::UnaryExpr { op, data } => {
            let data = eval_expr(data, ctx)?;
            match op {
                UnaryOp::Negate => Ok(-data),
            }
        }
        Expr::BinaryExpr { lhs, rhs, op } => {
            let lhs = eval_expr(lhs, ctx)?;
            let rhs = eval_expr(rhs, ctx)?;
            Ok(match op {
                BinaryOp::Plus => lhs + rhs,
                BinaryOp::Minus => lhs - rhs,
                BinaryOp::Times => lhs * rhs,
                BinaryOp::Divide => lhs / rhs,
            })
        }
        Expr::FunctionCall { function, args } => {
            let function = ctx.lookup_fn(function)?;
            let args = args
                .into_iter()
                .map(|arg| eval_expr(arg, ctx))
                .collect::<Result<Vec<Number>, CalcError>>()?;
            function.call(&args, ctx)
        }
        Expr::BlockExpr { stmts, final_expr } => {
            // Create new scope
            let mut eval_scope = ctx.clone();
            eval_scope.add_scope(HashMap::new());

            // Evaulate stmts in new scope
            for stmt in stmts {
                eval_stmt(stmt, &mut eval_scope)?;
            }

            // Evaluate expr in new scope
            eval_expr(final_expr, &eval_scope)
        }
    }
}

pub fn eval_stmt(stmt: &Stmt, ctx: &mut Context) -> CalcResult {
    match stmt {
        Stmt::Assignment { name, value } => {
            let value = eval_expr(value, ctx)?;
            ctx.bind_value(name.clone(), value)
        }
        Stmt::ExprStmt(expr) => eval_expr(expr, ctx),
    }
}
