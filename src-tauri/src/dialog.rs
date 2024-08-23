#![cfg(target_os = "macos")]
// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::ops::Deref;

use tauri::{Manager, Runtime, Window};

pub struct MyWindow<R: Runtime>(Window<R>);


unsafe impl<R: Runtime> raw_window_handle::HasRawWindowHandle for MyWindow<R> {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.0.raw_window_handle()
    }
}

impl<R: Runtime> MyWindow<R> {
    fn get_window_handle_from_parent(&self) {}
}

macro_rules! run_dialog {
  ($e:expr, $h: ident) => {{
    std::thread::spawn(move || {
      let response = $e;
      $h(response);
    });
  }};
}

macro_rules! run_file_dialog {
  ($e:expr, $h: ident) => {{
    std::thread::spawn(move || {
      let response = tauri::async_runtime::block_on($e);
      $h(response);
    });
  }};
}

macro_rules! run_dialog_sync {
  ($e:expr) => {{
    let (tx, rx) = sync_channel(0);
    let cb = move |response| {
      tx.send(response).unwrap();
    };
    run_file_dialog!($e, cb);
    rx.recv().unwrap()
  }};
}

/// Options for action buttons on message dialogs.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageDialogButtons {
    /// Ok button.
    Ok,
    /// Ok and Cancel buttons.
    OkCancel,
    /// Yes and No buttons.
    YesNo,
    /// OK button with customized text.
    OkWithLabel(String),
    /// Ok and Cancel buttons with customized text.
    OkCancelWithLabels(String, String),
}

impl From<MessageDialogButtons> for rfd::MessageButtons {
    fn from(kind: MessageDialogButtons) -> Self {
        match kind {
            MessageDialogButtons::Ok => Self::Ok,
            MessageDialogButtons::OkCancel => Self::OkCancel,
            MessageDialogButtons::YesNo => Self::YesNo,
            MessageDialogButtons::OkWithLabel(ok_text) => Self::OkCustom(ok_text),
            MessageDialogButtons::OkCancelWithLabels(ok_text, cancel_text) => {
                Self::OkCancelCustom(ok_text, cancel_text)
            }
        }
    }
}

/// Blocking interfaces for the dialog APIs.
///
/// The blocking APIs will block the current thread to execute instead of relying on callback closures,
/// which makes them easier to use.
///
/// **NOTE:** You cannot block the main thread when executing the dialog APIs, so you must use the [`crate::api::dialog`] methods instead.
/// Examples of main thread context are the [`crate::App::run`] closure and non-async commands.
pub mod blocking {
    use std::path::{Path, PathBuf};
    use std::sync::mpsc::sync_channel;

    use raw_window_handle::{HasRawWindowHandle, HasWindowHandle};

    #[cfg(target_os = "linux")]
    type FileDialog = rfd::FileDialog;
    #[cfg(not(target_os = "linux"))]
    type FileDialog = rfd::AsyncFileDialog;

    /// The file dialog builder.
    ///
    /// Constructs file picker dialogs that can select single/multiple files or directories.
    #[derive(Debug, Default)]
    pub struct FileDialogBuilder(FileDialog);

    impl FileDialogBuilder {
        /// Gets the default file dialog builder.
        pub fn new() -> Self {
            Default::default()
        }

        /// Add file extension filter. Takes in the name of the filter, and list of extensions
        #[must_use]
        pub fn add_filter(mut self, name: impl AsRef<str>, extensions: &[&str]) -> Self {
            self.0 = self.0.add_filter(name.as_ref(), extensions);
            self
        }

        /// Set starting directory of the dialog.
        #[must_use]
        pub fn set_directory<P: AsRef<Path>>(mut self, directory: P) -> Self {
            self.0 = self.0.set_directory(directory);
            self
        }

        /// Set starting file name of the dialog.
        #[must_use]
        pub fn set_file_name(mut self, file_name: &str) -> Self {
            self.0 = self.0.set_file_name(file_name);
            self
        }

        /// Sets the parent window of the dialog.
        #[must_use]
        pub fn set_parent<W: HasRawWindowHandle>(mut self, parent: &W) -> Self {
            self.0 = self.0.set_parent(parent);
            self
        }

        /// Set the title of the dialog.
        #[must_use]
        pub fn set_title(mut self, title: &str) -> Self {
            self.0 = self.0.set_title(title);
            self
        }
    }

    impl FileDialogBuilder {
        /// Shows the dialog to select a single file.
        /// This is a blocking operation,
        /// and should *NOT* be used when running on the main thread context.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use tauri::api::dialog::blocking::FileDialogBuilder;
        /// #[tauri::command]
        /// async fn my_command() {
        ///   let file_path = FileDialogBuilder::new().pick_file();
        ///   // do something with the optional file path here
        ///   // the file path is `None` if the user closed the dialog
        /// }
        /// ```
        pub fn pick_file(self) -> Option<PathBuf> {
            #[allow(clippy::let_and_return)]
            let response = run_dialog_sync!(self.0.pick_file());
            #[cfg(not(target_os = "linux"))]
            let response = response.map(|p| p.path().to_path_buf());
            response
        }

        /// Shows the dialog to select multiple files.
        /// This is a blocking operation,
        /// and should *NOT* be used when running on the main thread context.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use tauri::api::dialog::blocking::FileDialogBuilder;
        /// #[tauri::command]
        /// async fn my_command() {
        ///   let file_path = FileDialogBuilder::new().pick_files();
        ///   // do something with the optional file paths here
        ///   // the file paths value is `None` if the user closed the dialog
        /// }
        /// ```
        pub fn pick_files(self) -> Option<Vec<PathBuf>> {
            #[allow(clippy::let_and_return)]
            let response = run_dialog_sync!(self.0.pick_files());
            #[cfg(not(target_os = "linux"))]
            let response =
                response.map(|paths| paths.into_iter().map(|p| p.path().to_path_buf()).collect());
            response
        }

        /// Shows the dialog to select a single folder.
        /// This is a blocking operation,
        /// and should *NOT* be used when running on the main thread context.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use tauri::api::dialog::blocking::FileDialogBuilder;
        /// #[tauri::command]
        /// async fn my_command() {
        ///   let folder_path = FileDialogBuilder::new().pick_folder();
        ///   // do something with the optional folder path here
        ///   // the folder path is `None` if the user closed the dialog
        /// }
        /// ```
        pub fn pick_folder(self) -> Option<PathBuf> {
            #[allow(clippy::let_and_return)]
            let response = run_dialog_sync!(self.0.pick_folder());
            #[cfg(not(target_os = "linux"))]
            let response = response.map(|p| p.path().to_path_buf());
            response
        }

        /// Shows the dialog to select multiple folders.
        /// This is a blocking operation,
        /// and should *NOT* be used when running on the main thread context.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use tauri::api::dialog::blocking::FileDialogBuilder;
        /// #[tauri::command]
        /// async fn my_command() {
        ///   let folder_paths = FileDialogBuilder::new().pick_folders();
        ///   // do something with the optional folder paths here
        ///   // the folder paths value is `None` if the user closed the dialog
        /// }
        /// ```
        pub fn pick_folders(self) -> Option<Vec<PathBuf>> {
            #[allow(clippy::let_and_return)]
            let response = run_dialog_sync!(self.0.pick_folders());
            #[cfg(not(target_os = "linux"))]
            let response =
                response.map(|paths| paths.into_iter().map(|p| p.path().to_path_buf()).collect());
            response
        }

        /// Shows the dialog to save a file.
        /// This is a blocking operation,
        /// and should *NOT* be used when running on the main thread context.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use tauri::api::dialog::blocking::FileDialogBuilder;
        /// #[tauri::command]
        /// async fn my_command() {
        ///   let file_path = FileDialogBuilder::new().save_file();
        ///   // do something with the optional file path here
        ///   // the file path is `None` if the user closed the dialog
        /// }
        /// ```
        pub fn save_file(self) -> Option<PathBuf> {
            #[allow(clippy::let_and_return)]
            let response = run_dialog_sync!(self.0.save_file());
            #[cfg(not(target_os = "linux"))]
            let response = response.map(|p| p.path().to_path_buf());
            response
        }
    }
}