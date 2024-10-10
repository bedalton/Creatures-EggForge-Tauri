

#[cfg(target_os = "macos")]
use tauri::{AboutMetadata, CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder};

#[cfg(not(target_os = "macos"))]
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder};

pub fn init_menu(app_name: String, window_builder: WindowBuilder) -> WindowBuilder {
    let menu = os_default(app_name.clone().as_str());

    let window_builder = window_builder.menu(menu);
    window_builder
}

fn add_menu_item(menu: Menu, label: &str, title: &str) -> Menu {
    menu.add_item(CustomMenuItem::new(label, title.to_owned()))
}

fn add_menu_item_with_accelerator(menu: Menu, label: &str, title: &str, keyboard_shortcut: &str) -> Menu {
    let item = CustomMenuItem::new(label, title.to_owned())
        .accelerator(keyboard_shortcut);
    menu.add_item(item)
}

pub fn os_default(#[allow(unused)] app_name: &str) -> Menu {
    let mut menu = Menu::new();

    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            app_name,
            Menu::new()
                .add_native_item(MenuItem::About(
                    app_name.to_string(),
                    AboutMetadata::default(),
                ))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Services)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ));
    }

    let mut file_menu = Menu::new();
    file_menu = add_menu_item_with_accelerator(file_menu, "open_folder", "Open Folder", "CommandOrControl+o");
    file_menu = add_menu_item_with_accelerator(file_menu, "new_window", "New Window", "CommandOrControl+n");
    file_menu = add_menu_item_with_accelerator(file_menu, "reset", "Reset", "CommandOrControl+r");
    file_menu = file_menu.add_native_item(MenuItem::Separator);
    file_menu = add_menu_item_with_accelerator(file_menu, "import_egg_agent", "Import Egg Agent", "CommandOrControl+i");
    file_menu = file_menu.add_native_item(MenuItem::Separator);
    file_menu = file_menu.add_native_item(MenuItem::CloseWindow);
    #[cfg(not(target_os = "macos"))]
    {
        file_menu = file_menu.add_native_item(MenuItem::Quit);
    }
    menu = menu.add_submenu(Submenu::new("File", file_menu));

    #[cfg(not(target_os = "linux"))]
    let mut edit_menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::Undo);
        edit_menu = edit_menu.add_native_item(MenuItem::Redo);
        edit_menu = edit_menu.add_native_item(MenuItem::Separator);
    }
    #[cfg(not(target_os = "linux"))]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::Cut);
        edit_menu = edit_menu.add_native_item(MenuItem::Copy);
        edit_menu = edit_menu.add_native_item(MenuItem::Paste);
    }
    #[cfg(target_os = "macos")]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::SelectAll);
    }
    #[cfg(not(target_os = "linux"))]
    {
        menu = menu.add_submenu(Submenu::new("Edit", edit_menu));
    }
    #[cfg(target_os = "macos")]
    {
        let mut view_menu = Menu::new();
        view_menu = add_menu_item_with_accelerator(view_menu, "toggle_egg_mode", "toggle Simple/Advanced mode", "CommandOrControl+t");
        view_menu = view_menu.add_native_item(MenuItem::Separator);
        view_menu = view_menu.add_native_item(MenuItem::EnterFullScreen);
        menu = menu.add_submenu(Submenu::new("View", view_menu));

    }

    #[cfg(not(target_os = "macos"))]
    {
        let mut view_menu = Menu::new();
        view_menu = add_menu_item_with_accelerator(view_menu, "toggle_egg_mode", "toggle Simple/Advanced mode", "CommandOrControl+t");
        menu = menu.add_submenu(Submenu::new("View", view_menu));

    }

    let mut window_menu = Menu::new();
    // window_menu = add_menu_item_with_accelerator(window_menu, "toggle_egg_mode", "toggle Simple/Advanced mode", "CommandOrControl+t");
    // window_menu = window_menu.add_native_item(MenuItem::Separator);
    window_menu = window_menu.add_native_item(MenuItem::Minimize);
    #[cfg(target_os = "macos")]
    {
        window_menu = window_menu.add_native_item(MenuItem::Zoom);
        window_menu = window_menu.add_native_item(MenuItem::Separator);
    }
    window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
    menu = menu.add_submenu(Submenu::new("Window", window_menu));

    menu
}