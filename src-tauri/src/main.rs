// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use calculator::context::Context;
use calculator::eval::eval_stmt;
use calculator::parser::parse_stmt;
use calculator::parser::parse_stmt_list;
use calculator::CalcError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_context() -> Context {
    Context::new()
}

#[tauri::command]
fn tauri_eval_stmt(stmt: &str, mut ctx: Context) -> (String, Context) {
    let (rest, stmt) = match parse_stmt(stmt) {
        Ok((rest, stmt)) => (rest, stmt),
        Err(_) => return (CalcError::ParseError.to_string(), ctx),
    };

    if !rest.is_empty() {
        return (CalcError::ParseError.to_string(), ctx);
    }

    let res = eval_stmt(&stmt, &mut ctx);
    match res {
        Ok(res) => (res.to_string(), ctx),
        Err(e) => (e.to_string(), ctx),
    }
}

#[tauri::command]
fn tauri_eval_stmt_list(stmts: &str, mut ctx: Context) -> (Vec<String>, Context) {
    let (rest, stmts) = match parse_stmt_list(stmts) {
        Ok((rest, stmt)) => (rest, stmt),
        Err(_) => return (vec![CalcError::ParseError.to_string()], ctx),
    };

    if !rest.is_empty() {
        return (vec![CalcError::ParseError.to_string()], ctx);
    }

    let mut strs = vec![];
    for stmt in stmts {
        let res = eval_stmt(&stmt, &mut ctx);
        let str_res = match res {
            Ok(res) => res.to_string(),
            Err(e) => e.to_string(),
        };
        strs.push(str_res);
    }

    (strs, ctx)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            create_context,
            tauri_eval_stmt
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
