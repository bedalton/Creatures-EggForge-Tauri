use std::path::{Path, PathBuf};

use tauri::{Config, Manager, Result, Window};
use tauri::api::dialog;

use crate::config::get_config_value_string;

#[derive(Clone, serde::Serialize)]
struct OpenFolderPayload {
    path: String
}

// #[tauri::command]
// pub async fn open_sibling_body_data(window: &Window, path: &str) -> Result<Option<String>> {
//     let window: Window = window.clone();
//     let path = Path::new(path);
//     let mut output: Option<String> = None;
//     if let Some(parent) = path.parent() {
//         let body_data = parent().join("Body Data");
//         if body_data.exists() && body_data.is_dir() {
//             let _ = window.app_handle().fs_scope().allow_directory(body_data.clone(), false)?;
//             if let Some(path) = body_data.into_os_string().into_string() {
//                 output = Some(path);
//             }
//         }
//     }
//     Result(output)
// }

pub async fn open_project(window: &Window, config: &Config, is_starting: bool) -> bool {
    let result = get_project_path(config);

    if result.is_none() {
        return false;
    }

    let path = result.unwrap();

    if !path.clone().exists() {
        return false;
    }

    let _ = window.clone().app_handle().clone().fs_scope().allow_file(path.clone());

    let path_string = path
            .as_path()
            .to_str()
            .unwrap();


    let message_json = format!(
        "{{\"action\": \"open_project_root\", \"path\": \"{}\"}}",
        path_string
            .replace("\\", "\\\\")
            .replace("/", "\\/")
    );

    if is_starting {
        let js = format!("window.startupRequests = [...(window.startupRequests || []), {}]; ", message_json);
        match window.eval(js.as_str()) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Failed to set open egg into js startup;\n\t{}", e);
                false
            }
        }
    } else {
        window.emit("open_egg", OpenFolderPayload { path: path_string.to_owned() }).is_ok()
    }
}

fn get_project_path(config: &Config) -> Option<PathBuf> {
    let mut open_dialog = dialog::blocking::FileDialogBuilder::new()
        .set_title("Open Project Root")
        ;
    
    // Set starting directory
    let starting_directory = get_starting_directory(config);
    if starting_directory.is_some() {
        open_dialog = open_dialog.set_directory(starting_directory?)
    }

    open_dialog.pick_folder()
}

fn get_starting_directory(config: &Config) -> Option<String> {
    get_config_value_string(config, |config| {
        config.last_project_root
    })
}
