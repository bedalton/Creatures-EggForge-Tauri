use std::path::PathBuf;

use tauri::{AppHandle, Config, Manager};

use crate::config::{get_config, AppConfig};

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
        Some(mut extension) => {
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