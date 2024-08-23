use std::borrow::Borrow;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use tauri::{Manager, Window};
use Result;

#[cfg(not(target_os = "macos"))]
use tauri::api::dialog::blocking::FileDialogBuilder;

#[cfg(target_os = "macos")]
use crate::dialog::blocking::FileDialogBuilder;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogFilter {
    name: String,
    extensions: Vec<String>,
}


/// The options for the save dialog API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDialogOptions {
    /// The title of the dialog window.
    pub title: Option<String>,
    /// The filters of the dialog.
    #[serde(default)]
    pub filters: Vec<DialogFilter>,
    /// The initial path of the dialog.
    pub default_path: Option<PathBuf>,
}

#[tauri::command]
pub async fn save_file(
    window: Window,
    options: SaveDialogOptions,
) -> Result<Option<PathBuf>, String> {
    let mut dialog_builder = FileDialogBuilder::new();
    #[cfg(any(windows, target_os = "macos"))]
    {
        dialog_builder = dialog_builder.set_parent(window.borrow());
    }
    if let Some(title) = options.title {
        dialog_builder = dialog_builder.set_title(&title);
    }
    if let Some(default_path) = options.default_path {
        dialog_builder = set_default_path(dialog_builder, default_path);
    }
    for filter in options.filters {
        let extensions: Vec<&str> = filter.extensions.iter().map(|s| &**s).collect();
        dialog_builder = dialog_builder.add_filter(filter.name, &extensions);
    }

    let scopes = window.fs_scope();

    let path = dialog_builder.save_file();
    if let Some(p) = &path {
        scopes.allow_file(p).map_err(|err| err.to_string())?
    }

    Ok(path)
}

fn set_default_path(
    mut dialog_builder: FileDialogBuilder,
    default_path: PathBuf,
) -> FileDialogBuilder {
    if default_path.as_os_str().is_empty() {
        return dialog_builder;
    }

    // we need to adjust the separator on Windows: https://github.com/tauri-apps/tauri/issues/8074
    let default_path: &Path = default_path.as_path();

    if default_path.is_file() || !default_path.exists() {
        if let (Some(parent), Some(file_name)) = (default_path.parent(), default_path.file_name()) {
            if parent.components().count() > 0 {
                dialog_builder = dialog_builder.set_directory(parent);
            }

            println!("FileName: {}; Directory: {}", &file_name.to_string_lossy(), &parent.to_string_lossy());
            dialog_builder = dialog_builder.set_file_name(&file_name.to_string_lossy());
        } else if !default_path.is_file() {
            println!("Directory: {}", &default_path.to_string_lossy());
            dialog_builder = dialog_builder.set_directory(default_path);
        }
        dialog_builder
    } else {
        dialog_builder.set_directory(default_path)
    }
}

fn into_anyhow<T: std::fmt::Display>(err: T) -> anyhow::Error {
    anyhow::anyhow!(err.to_string())
}
