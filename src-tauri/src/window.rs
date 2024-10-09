use tauri::{AppHandle, Manager, Window};
use crate::make_window_with_app_handle;

pub fn get_focused_window_or_create<'a>(app_handle: AppHandle) -> (AppHandle, Window) {
    let window = app_handle.get_focused_window();
    if window.is_some() {
        return (app_handle, window.unwrap());
    }
    let (app_handle, wind) = get_any_window_or_create(app_handle);

    let _ = wind.set_focus();
    return (app_handle, wind.clone());
}

pub fn get_any_window_or_create<'a>(app_handle: AppHandle) -> (AppHandle, Window) {
    let windows = app_handle.clone().windows();
    if windows.is_empty() {
        let window = make_window_with_app_handle(app_handle.clone());
        return (app_handle, window);
    }
    for key in windows.clone().keys() {
        match windows.get(key) {
            Some(wind) => {
                return (app_handle, wind.clone());
            },
            None => continue
        }
    }
    let window = make_window_with_app_handle(app_handle.clone());
    (app_handle, window)
}