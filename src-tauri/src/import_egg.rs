use std::path::PathBuf;

use tauri::{Config, Manager, Window};
use tauri::api::dialog;

use crate::{config};
use crate::config::get_config_value_string;

#[derive(Clone, serde::Serialize)]
struct OpenEggPayload {
    path: String
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

    return open_dialog.pick_file();
}

fn get_starting_directory(config: &Config) -> Option<String> {
    return get_config_value_string(config, |config| {
        config.last_egg_import_directory
    })
}
