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
        },
        None => {}
    }
}

fn get_previous_genomes(config: AppConfig) -> Vec<PathBuf> {
    let previous = config.previous_genomes.clone();
    match previous {
        Some(genomes) => {
            if genomes.is_empty() {
                return Vec::new()
            }
            let mut out: Vec<PathBuf> = Vec::new();
            for path_string in genomes.iter() {
                let path = PathBuf::from(path_string);
                if path.clone().exists() {
                    out.push(path);
                } else {
                    println!("Skipping previous genomes: {:?}; Genome not found", path);
                }
            }
            out
        },
        None => Vec::new()
    }
}

