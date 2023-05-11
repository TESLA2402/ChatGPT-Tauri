
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    Manager
};

fn main() {
    
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.show().unwrap();
            main_window.set_focus().unwrap();
            Ok(())
        })
        
        .run(context)
        .expect("error while running tauri application");
}