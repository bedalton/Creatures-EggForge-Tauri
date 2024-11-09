use std::path::PathBuf;

use tauri::{AppHandle, Config, Manager};

use crate::config::{get_config, AppConfig};


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
/// add_previous_genome_to_scope(app_handle, "~/Documents/Creatures/Docking Station/Genetics/bruin.ex47.gno")
/// ```
#[tauri::command]
pub async fn add_previous_genome_to_scope(app_handle: AppHandle, path: &str) -> tauri::Result<bool> {
    let mut the_path = path.to_string();
    if !the_path.to_lowercase().ends_with(".gen") {
        the_path = format!("{:?}.gen", the_path);
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

pub fn add_previous_genomes_to_scope(app_handle: AppHandle, tauri_config: &Config) {
    match get_config(tauri_config) {
        Some(config) => {
            let genomes = get_previous_genomes(config);
            for genome in genomes {
                let _ = app_handle.fs_scope().allow_file(genome);
            }
        }
        None => {}
    }
}

fn get_previous_genomes(config: AppConfig) -> Vec<PathBuf> {
    let previous = config.previous_genomes.clone();
    match previous {
        Some(genomes) => {
            if genomes.is_empty() {
                return Vec::new();
            }
            let mut out: Vec<PathBuf> = Vec::new();
            for path_string in genomes.iter() {
                let path = PathBuf::from(path_string);
                if is_valid_genome(path.clone()) {
                    out.push(path);
                }
            }
            out
        }
        None => Vec::new()
    }
}

fn is_valid_genome(path: PathBuf) -> bool {
    if !path.clone().exists() {
        return false;
    }
    match path.clone().extension() {
        Some(extension) => {
            if extension.to_ascii_lowercase() == "gen" {
                true
            } else {
                println!("Invalid file extension on genomoe in previous genome list; Path: {:?}", path);
                false
            }
        }
        None => {
            println!("Invalid genome filename in previous genome list; Path: {:?}", path);
            false
        }
    }
}