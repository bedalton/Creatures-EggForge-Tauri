use std::path::PathBuf;

use tauri::api::dialog;
use tauri::{AppHandle, Config, Manager, Window};

use crate::config::get_config_value_string;

#[derive(Clone, serde::Serialize)]
struct OpenEggPayload {
    path: String
}


/// Adds the AGENTS file to the filesystem scope for a given agent file
///
/// # Arguments
///
/// * `app_handle`: handle to tauri app
/// * `path`: path to agent file if any
///
/// returns: Result<bool, Error> Ok(true) if GNO was added
///
/// # Examples
///
/// ```
/// add_agent_file_to_scope(app_handle, "~/Documents/Creatures/Docking Station/Genetics/bruin.ex47.gno")
/// ```
#[tauri::command]
pub async fn add_agent_file_to_scope(app_handle: AppHandle, path: &str) -> tauri::Result<bool> {
    let the_path: String = path.to_string();
    if !the_path.to_lowercase().ends_with(".agents") && !the_path.to_lowercase().ends_with(".agent") {
        println!("Failed to add agents path. Path is not an agents path");
        return Ok(false)
    }
    let result = app_handle.fs_scope().allow_file(the_path);
    match result {
        Ok(_) => Ok(true.into()),
        Err(e) => {
            eprintln!("Failed to add agents path {:?}", e);
            Ok(false.into())
        }
    }
}

pub async fn import_egg_file_into_window(window: &Window, config: &Config, is_starting: bool) -> bool {
    let result = get_egg_agent_path(config);

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
        "{{\"action\": \"open_egg\", \"path\": \"{}\"}}",
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
        window.emit("open_egg", OpenEggPayload { path: path_string.to_owned() }).is_ok()
    }
}

fn get_egg_agent_path(config: &Config) -> Option<PathBuf> {
    let extensions: [&str; 2] = ["agents", "agent"];
    let mut open_dialog = dialog::blocking::FileDialogBuilder::new()
        .set_title("Import Egg Agent")
        .add_filter("Agent", &extensions)
        ;
    
    // Set starting directory
    let starting_directory = get_starting_directory(config);
    if starting_directory.is_some() {
        open_dialog = open_dialog.set_directory(starting_directory.unwrap())
    }

    open_dialog.pick_file()
}

fn get_starting_directory(config: &Config) -> Option<String> {
    get_config_value_string(config, |config| {
        config.last_egg_import_directory
    })
}
