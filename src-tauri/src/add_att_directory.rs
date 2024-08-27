use std::path::Path;
use tauri::{AppHandle, Manager, Result};

#[tauri::command]
pub async fn add_att_directory(app_handle: AppHandle, path: &str) -> Result<bool> {
    let path = Path::new(path);

    if !path.exists() {
        return Ok(false);
    }

    let has_atts = match has_atts_or_empty(path).await {
        Ok(has) => has,
        Err(_) => return Ok(false),
    };

    if !has_atts {
        return Ok(false);
    }

    let result = app_handle.fs_scope().allow_file(path);
    match result {
        Ok(_) => Ok(true),
        Err(e) => {
            println!("Failed to add ATT folder {:?}", e);
            Ok(false.into())
        }
    }
}

async fn has_atts_or_empty(path: &Path) -> Result<bool> {
    if let Ok(contents) = path.read_dir() {
        let mut is_empty = true;
        let mut has_att = false;
        for f in contents {
            if let Ok(f) = f {
                let path = f.path();
                is_empty = false;

                let extension = match path.extension() {
                    Some(extension) => {
                        match extension.to_os_string().into_string() {
                            Ok(extension) => extension,
                            Err(_) => continue,
                        }
                    }
                    None => continue
                };

                if extension == "att" {
                    has_att = true;
                    break;
                }
            }
        }
        return Ok(has_att || is_empty);
    }
    Ok(true)
}