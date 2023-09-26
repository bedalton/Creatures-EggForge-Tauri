#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use tauri::Manager;
use std::env;

#[tauri::command]
fn is_dir(path: &str) -> bool {
    let path_for_os = match env::consts::OS {
        "windows" => path.to_string(),
        _ => path.to_string()
    };
    let path_for_err = path_for_os.clone();
    let is_dir_result = tauri::api::dir::is_dir(path_for_os);
    match is_dir_result  {
        Ok(result) => result,
        Result::Err(error) => {
            println!("Error checking is dir; {:?}; <{:?}> ", error, path_for_err);
            false
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
