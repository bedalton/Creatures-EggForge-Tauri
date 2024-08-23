use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::Config;

fn get_config_file_name() -> String {
    "eggforge.config.json".to_owned()
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub last_egg_import_directory: Option<String>,
    pub last_project_root: Option<String>
}

pub fn get_config(config: &Config) -> Option<AppConfig>  {
    let config_str = read_config_file_as_str(config);
    if config_str.is_none() {
        return None
    }
    let raw_json = config_str.unwrap();
    let json_str = raw_json.as_str();
    match serde_json::from_str(json_str) {
        Ok(conf) => {
            Some(conf)
        },
        Err(e) => {
            eprintln!("Failed to parse config file: \"{}\";\n\tError: {}", raw_json.clone(), e.to_string());
            None
        }
    }
}

fn read_config_file_as_str(config: &Config) -> Option<String> {
    let path_optional = get_config_file_path(config);
    if path_optional.is_none() {
        return None;
    }
    
    let path = path_optional.unwrap();
    
    match fs::read_to_string(path.clone().as_path()) {
        Ok(result) => {
            let raw_json = result.clone();
            if raw_json.clone().trim().is_empty() {
                println!("Config is empty");
                return None
            }
            Some(result)
        },
        Err(e) => {
            eprintln!("Failed to read config file: {};\n\tError: {}", path.as_path().to_str().unwrap(), e.to_string());
            None
        }
    }
}

fn get_config_file_path(config: &Config) -> Option<PathBuf> {
    let path_optional = tauri::api::path::app_config_dir(config);
    if path_optional.is_none() {
        return None;
    }
    let path_buf = path_optional.unwrap();
    let path = path_buf.join(get_config_file_name());
    if !path.clone().exists() {
        return None;
    }
    Some(path)
}

pub fn get_config_value_string<F>(
    config: &Config, 
    f: F
) -> Option<String> where F: Fn(AppConfig) -> Option<String> {
    if let Some(conf) = get_config(config) {
        f(conf)
    } else {
        None
    }
}

pub fn get_config_value_path<F>(
    config: &Config,
    f: F
) -> Option<String> where F: Fn(AppConfig) -> Option<String> {
    if let Some(value) = get_config_value_string(config, f) {
        if (value.clone().trim().is_empty()) {
            return Some(value)
        }
    }
    None
}