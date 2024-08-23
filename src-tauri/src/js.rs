use tauri::Window;

pub fn request_update_view_mode_from_js(window: &Window) -> &Window {
    window.emit("update_view_mode_in_tauri", ()).unwrap();
    window
}

pub fn toggle_egg_mode_in_js(window: &Window) {
    window.emit("toggle_egg_mode", ()).unwrap()
}

pub fn reset_view(window: &Window) {
    window.emit("reset_view", ()).unwrap()
}
