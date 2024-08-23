use tauri::{ Window };

/// Sets the tauri view mode menu button from JS frontend
///
/// # Arguments
///
/// * `window`: window to set the view mode menu toggle for
/// * `view_mode`: the current view mode
///
/// returns: Result<bool, Error>
///
/// # Examples
///
/// ```
///
/// ```
#[tauri::command]
pub async fn set_egg_mode_in_tauri(
    window: Window,
    egg_mode: String,
) -> tauri::Result<bool> {
    let _ = set_toggle_egg_mode_text_in_menu(window, egg_mode.to_owned());
    Ok(true)
}

/// Sets the view mode toggle text in the app menu
///
/// # Arguments
///
/// * `window`: The window to update toggle view text for
/// * `current_view_mod`: Current view mode; oneof ["simple", "advanced"]
///
///
/// # Examples
///
/// ```
/// set_toggle_view_mode_text_in_menu(app_handle.get_window("main"), "simple");
/// ```
fn set_toggle_egg_mode_text_in_menu(window: Window, current_view_mod: String) {
    let new_menu_text = get_toggle_egg_mode_text(current_view_mod.clone());
    let menu_handle = window.menu_handle();
    menu_handle.get_item("toggle_egg_mode").set_title(new_menu_text).unwrap()
}

///
/// Gets the text that should be displayed in the menu for toggling view modes
/// # Arguments
///
/// * `view_mode`:  oneof ["simple", "advanced"] describes the current view mode
///
/// returns: String
///
/// # Examples
///
/// ```
/// let title = get_toggle_view_mode_text("simple");
/// menu.set_title(title);
/// ```
fn get_toggle_egg_mode_text(view_mode: String) -> String {
    let new_text = match view_mode.as_str() {
        "simple" | "single" => { "Use Advanced Mode" },
        "advanced" | "multi" => { "Use Simple Mode" },
        _ => { "Toggle Simple/Advanced View Mode" }
    };
    new_text.to_string()
}