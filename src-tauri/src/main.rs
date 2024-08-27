#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(dead_code)]
use std::borrow::Borrow;
use std::collections::HashSet;
use std::{env, thread};
use std::fmt::format;
use std::ops::Deref;
use std::sync::Mutex;

use rand::distributions::Alphanumeric;
use rand::Rng;
use tauri::{AppHandle, LogicalSize, Manager, Result, Size, State, Window, WindowBuilder, WindowUrl};
use tauri::async_runtime::block_on;
use crate::import_egg::import_egg_file_into_window;
use crate::open_project::open_project;
use crate::view_mode::set_egg_mode_in_tauri;
use crate::save_dialog::save_file;
use crate::add_att_directory::add_att_directory;
mod menu;
mod view_mode;
mod js;
mod import_egg;
mod config;
mod window;
mod save_dialog;
mod dialog;
mod open_project;
mod add_att_directory;


#[derive(Clone, serde::Serialize)]
struct StringMessage {
    message: String,
}

pub struct AppState {
    last_window_id: Mutex<i32>,
    window_ids: Mutex<HashSet<String>>
}

/// JS callable function to find if path is Directory
///
/// # Arguments
///
/// * `path`: absolute path to possible directory or file
///
/// returns: bool true if path points to directory
///
/// # Examples
///
/// ```
/// let dir = "~/Documents";
/// if is_dir(dir) {
///     print("{} is a directory", dir);
/// } else {
///     print("{} is not a directory", dir);
/// }
/// ```
#[tauri::command]
fn is_dir(path: &str) -> bool {
    let path_for_os = match env::consts::OS {
        "windows" => path.to_string(),
        _ => path.to_string()
    };
    let path_for_err = path_for_os.clone();
    let is_dir_result = tauri::api::dir::is_dir(path_for_os);
    match is_dir_result {
        Ok(result) => result,
        Err(error) => {
            println!("Error checking is dir; {:?}; <{:?}> ", error, path_for_err);
            false
        }
    }
}

#[tauri::command]
async fn get_window_id(
    window: Window,
) -> Result<String> {
    Ok(window.label().to_owned())
}

/// Adds the GNO file to the filesystem scope for a given genome
///
/// # Arguments
///
/// * `app_handle`: handle to tauri app
/// * `path`: path to GNO file if any
///
/// returns: Result<bool, Error> Ok(true) if GNO was added
///
/// # Examples
///
/// ```
/// add_gno(app_handle, "~/Documents/Creatures/Docking Station/Genetics/bruin.ex47.gno")
/// ```
#[tauri::command]
async fn add_gno(app_handle: AppHandle, path: &str) -> Result<bool> {
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

/// Adds the GNO file to the filesystem scope for a given genome
///
/// # Arguments
///
/// * `app_handle`: handle to tauri app
/// * `path`: path to GNO file if any
///
/// returns: Result<bool, Error> Ok(true) if GNO was added
///
/// # Examples
///
/// ```
/// add_gno(app_handle, "~/Documents/Creatures/Docking Station/Genetics/bruin.ex47.gno")
/// ```
#[tauri::command]
async fn add_agents(app_handle: AppHandle, path: &str) -> Result<bool> {
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

fn main() {
    let state = AppState {
        last_window_id: Mutex::new(0),
        window_ids: Mutex::new(HashSet::new()),
    };
    let context = tauri::generate_context!();
    let app_name = &context.package_info().name;
    let app_name_for_setup = app_name.clone();
    let app_name_for_menu = app_name.clone();
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            is_dir,
            add_gno,
            set_egg_mode_in_tauri,
            get_window_id,
            save_file,
            add_att_directory,
        ])
        .setup(move |app | {
            let handle = app.handle();
            let menu_handle = handle.clone();
            std::thread::spawn(move || {
                make_window_with_app_name(app_name_for_setup.to_owned(), menu_handle);
            });
            Ok(())
        })
        .on_menu_event(move | e| {
            let _ = match e.menu_item_id() {
                "toggle_egg_mode" => {
                    js::toggle_egg_mode_in_js(e.window());
                    true
                },
                "reset" => {
                    js::reset_view(e.window());
                    true
                },
                "new_window" => {
                    let window = e.window().clone();
                    let app_handle = window.app_handle();
                    make_window_with_app_name(app_name_for_menu.to_owned(), app_handle);
                    true
                },
                "import_egg_agent" => {
                    let window_ = e.window().clone();
                    let config = window_.clone().config();
                    let _ = thread::spawn(move || {
                        let future = import_egg_file_into_window(&window_, config.deref(), false);
                        let imported = block_on(future);
                        if !imported {
                            println!("Failed to set egg import");
                        }
                    });
                    true
                }
                "open_folder" => {
                    let window_ = e.window().clone();
                    let config = window_.clone().config();
                    let _ = thread::spawn(move || {
                        let future = open_project(&window_, config.deref(), false);
                        let imported = block_on(future);
                        if !imported {
                            println!("Failed to set egg import");
                        }
                    });
                    true
                }
                _ => false
            };
        })
        .run(context.into())
        .expect("error while running tauri application");
}

pub fn make_window_with_app_handle(app_handle: AppHandle) -> Window {
    let app_name = &app_handle.package_info().name;
    make_window_with_app_name(app_name.to_string(), app_handle)
}

fn make_window_with_app_name(app_name: String, app_handle: AppHandle) -> Window {
    let state: State<AppState> = app_handle.state();
    let next_window_id: String = next_window_id(state, "egg-window-");

    let mut window_builder = WindowBuilder::new(
        &app_handle,
        next_window_id,
        WindowUrl::App("index.html".into()),
    );

    window_builder = menu::init_menu(app_name.clone(), window_builder);

    let window = window_builder
        .build()
        .unwrap();

    window.set_title("EggForge").unwrap();
    window.set_fullscreen(false).unwrap();
    window.set_size(Size::Logical(LogicalSize { width: 960.0, height: 600.0 })).unwrap();
    window.set_resizable(false).unwrap();
    window.borrow().eval(format(format_args!("setWindowId({});", window.label())).as_str()).unwrap();

    js::request_update_view_mode_from_js(window.borrow());

    window
}


/// Gets the integer id for the next window
///
/// # Arguments
///
/// * `state`: AppState as it is now
/// * `prefix`:
///
/// returns: String
///
/// # Examples
///
/// ```
/// let label = next_window_id(state, "main-window-");
/// ```
fn next_window_id(state: State<AppState>, prefix: &str) -> String {
    let last_id_lock = state.last_window_id.lock();
    let id_string: String = match last_id_lock {
        Ok(mut locked) => {
            let next_id = locked.clone() + 1;
            *locked = next_id.clone();
            let id_int_as_string = next_id.to_string();
            id_int_as_string
        },
        Err(_) => {
            let window_ids_locked = state.window_ids.lock();
            match window_ids_locked {
                Ok(mut window_ids) => {
                    let items_for_search = window_ids.clone();
                    let mut out = random_string(8);
                    while items_for_search.contains(&out) {
                        out = random_string(8)
                    }
                    window_ids.insert(out.clone().to_owned());
                    out
                },
                Err(_) => {
                    random_string(18)
                }
            }
        }
    };
    let mut out = prefix.to_owned();
    out.push_str(id_string.as_str());
    out
}

fn random_string(chars: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(chars)
        .map(char::from)
        .collect()
}