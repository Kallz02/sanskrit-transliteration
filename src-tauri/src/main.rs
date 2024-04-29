// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{
    Enigo, Keyboard, Settings,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    dbg!(name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}




#[tauri::command]
fn press(name: &str) {
    dbg!(name);
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ā").unwrap();
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,press])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
