use crate::err::Error;
use crate::file_utils::is_creatures_file;
use crate::{err, random_string, AppState};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State, Window};

#[tauri::command]
pub async fn create_agent_cache_directory(window: Window) -> Result<String, Error> {
    let state: State<AppState> = window.state();
    let mut paths_map: HashMap<String, Vec<PathBuf>> = match state.cache_dirs.lock() {
        Ok(paths) => {
            paths.to_owned()
        },
        Err(e) => return Err(err!("Failed to get cache directory lock: {}", e.to_string())),
    };

    let paths_vec = flatten_map(paths_map.clone()).into_iter().map(|x|{
        x.clone().into_os_string().into_string().unwrap()
    }).collect::<Vec<String>>();

    let cache_dir = match window.app_handle().path_resolver().app_cache_dir() {
        Some(cache_dir) => cache_dir.into_os_string().into_string().unwrap(),
        None => return Err(err!("Failed to get cache dir list"))
    };


    let mut path = format!("{}/agent-dump/{}/", cache_dir.clone(), random_string(16).to_lowercase());
    let path_iter = path.clone();
    while paths_vec.contains(&path_iter) {
        path = format!("{}/agent-dump/{}/", cache_dir.clone(), random_string(16).to_lowercase());
    }

    let result =  fs::create_dir_all(path.clone());
    if result.is_err() {
        return match result.err() {
            Some(e) => Err(err!("Failed to create cache dir: {}", e)),
            None => Err(err!("Failed to create cache dir"))
        }
    };

    let window_id = window.label().to_owned();
    match window.fs_scope().allow_directory(path.clone(), true) {
        Ok(_) => {
            let path_buf = PathBuf::from(path.clone());
            if !paths_map.contains_key(&window_id) {
                paths_map.insert(window_id.clone(), vec![]);
            }
            let mut window_paths = paths_map.get_mut(&window_id).unwrap().clone();
            window_paths.push(path_buf);
            Ok(path)
        },
        Err(_) => {
            let _ = fs::remove_dir_all(path.clone());
            Err(err!("Failed to add new cache dir to scope"))
        }
    }
}

#[tauri::command]
pub async fn write_agent_cache_file(app_handle: AppHandle, path: PathBuf, bytes: Vec<u8>) -> Result<bool, Error> {
    let state: State<AppState> = app_handle.state();
    
    let paths = match state.cache_dirs.lock() {
        Ok(paths) => {
            let paths_map = paths.to_owned();
            flatten_map(paths_map)
        },
        Err(e) => return Err(err!("Failed to open cache dirs list; {}", e.to_string()))
    };

    let mut exists = false;
    for a_path_buf in paths {
        exists = exists || match a_path_buf.into_os_string().into_string() {
            Ok(a_path) => path.starts_with(a_path),
            Err(_) => false,
        };
    }
    
    if !exists {
        return Err(err!("Path is not an agent cache path"))
    }
    
    match app_handle.fs_scope().allow_file(path.clone()) {
        Ok(_) => {}
        Err(e) => return Err(err!("Failed to add cache file to scope: {}", e))
    }


    if !is_creatures_file(path.clone()) {
        return Err(err!("File is not creatures file: {}", path.into_os_string().into_string().unwrap_or("UNDEFINED".to_string())))
    }
    
    match fs::write(path.as_path(), bytes) {
        Ok(_) => (),
        Err(e) => return Err(err!("Failed to write cache file: {}", e))
    }

    if !path.as_path().exists() {
        return Err(err!("File does not exist after successful file write call"))
    }

    Ok(false)
}

#[tauri::command]
pub async fn clear_agent_cache_directory(app_handle: AppHandle, path: PathBuf) -> Result<bool, Error> {
    let cache_directory = match app_handle.path_resolver().app_cache_dir() {
        Some(cache_directory) => cache_directory,
        None => return Err(err!("Failed to get cache directory"))
    };

    let agent_cache_directory = format!("{}/{}/", cache_directory.clone().into_os_string().into_string().unwrap(), "agent-dump");
    if path.as_path().starts_with(agent_cache_directory) {
        match fs::remove_dir_all(path.as_path()) {
            Ok(_) => Ok(true),
            Err(e) => Err(err!("Failed to remove cache directory; {}", e.to_string()))
        }
    } else {
        Err(err!("Directory is not an agent cache directory"))
    }
}

pub async fn clear_agent_cache_directories(app_handle: AppHandle) -> bool {
    let cache_dir = match app_handle.path_resolver().app_cache_dir() {
        Some(cache_dir) => cache_dir,
        None => return false,
    };

    let directories = match fs::read_dir(cache_dir.clone()) {
        Ok(dirs) => dirs,
        Err(_) => return false,
    };

    let mut okay = true;
    let cache_dir_string = match cache_dir.into_os_string().to_str() {
        Some(dir) => format!("{}/{}/", dir, "agent-dump"),
        None => return false
    };

    for dir_result in directories {
        let dir = match dir_result {
            Ok(dir) => dir,
            Err(_) => continue,
        };

        match dir.path().to_str() {
            Some(str) => {
                if str.starts_with(cache_dir_string.as_str()) {
                    okay = fs::remove_dir_all(dir.path()).is_ok() && okay;
                }
            },
            None => okay = false
        }
    }
    okay
}

fn flatten_map<K,V: std::clone::Clone>(map: HashMap<K, Vec<V>>) -> Vec<V> {
    let mut flattened: Vec<V> = Vec::new();
    for (_, vec) in map.iter() {
        for v in vec.into_iter().cloned() {
            flattened.push(v);
        }
    }
    flattened
}