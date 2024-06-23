// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use self::hash_function::HashFunction;
use tauri::api::dialog::blocking::FileDialogBuilder;

mod hash_function;

#[tauri::command]
async fn pick_file() -> Option<PathBuf> {
    FileDialogBuilder::new().pick_file()
}

#[tauri::command]
async fn generate_digest(
    path_buf: PathBuf,
    hash_function: HashFunction,
) -> Result<Option<String>, ()> {
    HashFunction::compute_digest(path_buf, hash_function)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![pick_file, generate_digest])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
