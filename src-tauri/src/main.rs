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

#[tauri::command]
async fn add_gno(app_handle: tauri::AppHandle, path: &str) -> Result<bool, String> {
    let mut the_path = path.to_string();
    if !the_path.to_lowercase().ends_with(".gno") {
        the_path = format!("{:?}.gno", the_path);
    }
    let result = app_handle.fs_scope().allow_file(the_path);
    match result {
        Ok(_) => Ok(true.into()),
        Err(e) => {
            println!("Failed to add gno path {:?}", e);
            Ok(false.into())
        }
    }
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_dir, add_gno])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
