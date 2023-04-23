// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use calculator::context::Context;
use calculator::eval::eval_stmt;
use calculator::parser::parse_stmt;

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
    let (_, stmt) = parse_stmt(stmt).unwrap();
    let res = eval_stmt(&stmt, &mut ctx);
    match res {
        Ok(res) => (res.to_string(), ctx),
        Err(e) => (e.to_string(), ctx),
    }
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
