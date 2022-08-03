#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::sync::atomic::{AtomicI32, Ordering};

///数字を格納する構造体
struct CountValue(AtomicI32);

///数字を増やす関数
#[tauri::command]
fn increment(state: tauri::State<CountValue>) -> i32 {
  state.0.fetch_add(1, Ordering::SeqCst) + 1
}

///数字を減らす関数
#[tauri::command]
fn decrement(state: tauri::State<CountValue>) -> i32 {
  state.0.fetch_sub(1, Ordering::SeqCst) - 1
}

fn main() {
  tauri::Builder::default()
      .manage(CountValue(AtomicI32::new(0)))
      .invoke_handler(tauri::generate_handler![increment,decrement])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

