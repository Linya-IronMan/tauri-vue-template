#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod file;

mod command;
use command::{backend_add, open_file_dialog};

use tauri::api::shell;
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![backend_add, open_file_dialog])
        .menu(
            tauri::Menu::os_default("Tauri Vue Template").add_submenu(Submenu::new(
                "Help",
                Menu::with_items([CustomMenuItem::new(
                    "Online Documentation",
                    "Online Documentation",
                )
                .into()]),
            )),
        )
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "Online Documentation" => {
                    let url = "https://github.com/Uninen/tauri-vue-template".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                _ => {}
            }
        })
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let main_window = _app.get_window("main").unwrap();
                main_window.open_devtools();
            }
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
