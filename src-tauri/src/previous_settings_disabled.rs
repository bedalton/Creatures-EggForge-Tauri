use tauri::{Window, Result};
use crate::menu::set_menu_item_text;

pub fn toggle_disable_project_settings_reload_in_js(window: Window) {
    window.emit("toggle_disable_project_settings_reload", ()).unwrap()
}

pub fn clear_saved_settings_for_all_projects_in_js(window: Window) {
    window.emit("clear_saved_settings_for_all_projects", ()).unwrap()
}

pub fn clear_saved_project_settings_for_project_in_js(window: Window) {
    window.emit("clear_saved_project_settings_for_project", ()).unwrap()
}

pub fn toggle_disable_genome_list_reload_in_js(window: Window) {
    window.emit("toggle_disable_genome_list_reload", ()).unwrap()
}

pub fn clear_all_previous_genomes_from_all_time_in_js(window: Window) {
    window.emit("clear_all_previous_genomes_from_all_time", ()).unwrap()
}

pub fn clear_previous_genomes_list_from_project_in_js(window: Window) {
    window.emit("clear_previous_genomes_list_from_project", ()).unwrap()
}


#[tauri::command]
pub async fn set_project_settings_cached_disabled_in_tauri(window: Window, disabled: bool) -> Result<bool> {
    let menu_item_id = "toggle_project_settings_disabled";
    let disabled_text = if disabled {
        "Enable"
    } else {
        "Disable"
    };
    let menu_item_text = format!("{} Project Settings Reload", disabled_text.to_string());
    set_menu_item_text(window, menu_item_id, menu_item_text).await
}


#[tauri::command]
pub async fn set_genome_list_disabled_in_tauri(window: Window, disabled: bool) -> Result<bool> {
    let menu_item_id = "toggle_previous_genomes_list_disabled";
    let disabled_text = if disabled {
        "Enable"
    } else {
        "Disable"
    };
    let menu_item_text = format!("{} Previous Genome List Reloading", disabled_text);
    set_menu_item_text(window, menu_item_id, menu_item_text).await
}