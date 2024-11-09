#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(dead_code)]
use std::borrow::Borrow;
use std::collections::HashSet;
use std::{env, fs, thread};
use std::fmt::format;
use std::ops::Deref;
use std::path::Path;
use std::sync::Mutex;

use rand::distributions::Alphanumeric;
use rand::Rng;
use tauri::{AppHandle, LogicalSize, Manager, Size, State, Window, WindowBuilder, WindowUrl};
use tauri::async_runtime::block_on;
use crate::import_egg::{add_agent_file_to_scope, import_egg_file_into_window};
use crate::open_project::open_project;
use crate::view_mode::set_egg_mode_in_tauri;
use crate::save_dialog::save_file;
use crate::add_att_directory::add_att_directory;
use crate::add_previous_genomes_to_scope::add_previous_genomes_to_scope;
use crate::add_previous_genomes_to_scope::add_previous_genome_to_scope;
use crate::previous_settings_disabled::{set_genome_list_disabled_in_tauri, set_project_settings_cached_disabled_in_tauri};

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
mod add_previous_genomes_to_scope;
mod previous_settings_disabled;

#[derive(Clone, serde::Serialize)]
struct StringMessage {
    message: String,
}

pub struct AppState {
    last_window_id: Mutex<i32>,
    window_ids: Mutex<HashSet<String>>,
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
fn is_directory(path: &str) -> bool {
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
fn file_exists(path: &str) -> bool {
    let path_for_os = match env::consts::OS {
        "windows" => path.to_string(),
        _ => path.to_string()
    };
    Path::new(path_for_os.as_str()).exists()
}

#[tauri::command]
async fn list_files(app_handle: AppHandle, path: &str) -> Result<Vec<String>, String> {
    let path_for_os = match env::consts::OS {
        "windows" => path.to_string(),
        _ => path.to_string()
    };

    let paths = match fs::read_dir(path_for_os) {
        Ok(result) => result,
        Err(e) => {
            let error = format!("Failed to read_dir; {:?}", e.to_string());
            return Err(error);
        }
    };

    let mut out = Vec::new();

    for path in paths {
        let p = match path {
            Ok(p) => p.path(),
            Err(_) => continue
        };

        if p.clone().is_dir() {
            continue;
        }

        let add = match p.clone().extension().unwrap().to_str() {
            Some(ext) => {
                match ext.to_lowercase().as_str() {
                    "att" => true,
                    "c16" => true,
                    "gno" => true,
                    "gen" => true,
                    "agent" => true,
                    "agents" => true,
                    _ => false
                }
            }
            None => false
        };

        if add {
            continue;
        }

        match app_handle.fs_scope().allow_file(p.clone()) {
            Ok(_) => {
                let path_string = p.clone().into_os_string().into_string().unwrap();
                out.push(path_string);
            }
            Err(_) => {
                let error_path = p.clone().into_os_string();
                let message = format!("Error adding path to scope from list_dir: {:?}", error_path);
                return Err(message.as_str().into());
            }
        }
    }
    Ok(out)
}


#[tauri::command]
async fn get_window_id(
    window: Window,
) -> Result<String, tauri::Error> {
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
async fn add_gno(app_handle: AppHandle, path: &str) -> Result<bool, tauri::Error> {
    let mut the_path = path.to_string();
    if !the_path.to_lowercase().ends_with(".gno") {
        the_path = format!("{}.gno", the_path);
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
            is_directory,
            file_exists,
            list_files,
            add_gno,
            add_previous_genome_to_scope,
            set_egg_mode_in_tauri,
            get_window_id,
            save_file,
            add_att_directory,
            add_agent_file_to_scope,
            set_genome_list_disabled_in_tauri,
            set_project_settings_cached_disabled_in_tauri
        ])
        .setup(move |app| {
            let handle = app.handle();
            let menu_handle = handle.clone();
            add_previous_genomes_to_scope(handle.clone(), handle.clone().config().borrow());
            thread::spawn(move || {
                make_window_with_app_name(app_name_for_setup.to_owned(), menu_handle);
            });
            Ok(())
        })
        .on_menu_event(move |e| {
            let _ = match e.menu_item_id() {
                "toggle_egg_mode" => {
                    js::toggle_egg_mode_in_js(e.window());
                    true
                }
                "reset" => {
                    js::reset_view(e.window());
                    true
                }
                "new_window" => {
                    let window = e.window().clone();
                    let app_handle = window.app_handle();
                    make_window_with_app_name(app_name_for_menu.to_owned(), app_handle);
                    true
                }
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
                "clear_project_settings_for_all_projects" => {
                    let window_ = e.window().clone();
                    previous_settings_disabled::clear_saved_settings_for_all_projects_in_js(window_);
                    true
                }
                "clear_project_settings_for_project" => {
                    let window_ = e.window().clone();
                    previous_settings_disabled::clear_saved_project_settings_for_project_in_js(window_);
                    true
                }

                "toggle_project_settings_disabled" => {
                    let window_ = e.window().clone();
                    previous_settings_disabled::toggle_disable_project_settings_reload_in_js(window_);
                    true
                }

                "clear_previous_genomes_for_project" => {
                    let window_ = e.window().clone();
                    previous_settings_disabled::clear_previous_genomes_list_from_project_in_js(window_);
                    true
                }

                "clear_previous_genomes_for_all_projects" => {
                    let window_ = e.window().clone();
                    previous_settings_disabled::clear_all_previous_genomes_from_all_time_in_js(window_);
                    true
                }

                "toggle_previous_genomes_list_disabled" => {
                    let window_ = e.window().clone();
                    previous_settings_disabled::toggle_disable_genome_list_reload_in_js(window_);
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
        }
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
                }
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