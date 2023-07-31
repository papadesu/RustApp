// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      print_command,
      greet,
      test,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn print_command() {
  println!("call form JavaScript");
}

#[tauri::command]
fn greet() {
  println!("call form Python");
}

#[tauri::command]
fn test() {
  let status = Command::new("./binaries/dist/output") // 実行したい実行ファイルのパスを指定します
  .status()
  .expect("Failed to execute the command.");

  if status.success() {
    println!("Command execution succeeded.");
  } else {
    println!("Command execution failed with exit code: {}", status);
  }
}