#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rfd::FileDialog;
use serde::Serialize;
use std::path::PathBuf;
use tauri::api::shell;
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

struct FileList {
    files: Vec<File>,
}

impl FileList {
    fn new() -> Self {
        FileList { files: Vec::new() }
    }

    fn add_file(&mut self, file: File) -> &Self {
        self.files.push(file);
        self
    }
}

#[derive(Debug, Serialize)]
struct File {
    path: PathBuf,
    name: String,
    extension: String,
    parent: PathBuf,
    size: u64,
    is_file: bool,
    is_dir: bool,
}

impl File {
    fn new(path: PathBuf) -> Result<Self, std::io::Error> {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let extension = path.extension().unwrap().to_str().unwrap().to_string();
        let parent = path.parent().unwrap().to_path_buf();
        let size = std::fs::metadata(&path).unwrap().len();
        let is_file = std::fs::metadata(&path).unwrap().is_file();
        let is_dir = std::fs::metadata(&path).unwrap().is_dir();

        Ok(File {
            path,
            name,
            size,
            extension,
            parent,
            is_dir,
            is_file,
        })
    }
}

#[tauri::command]
fn backend_add(number: i32) -> i32 {
    // Note: these commands block the main thread and hang the UI until they return.
    // If you need to run a long-running task, use async command instead.
    println!("Backend was called with an argument: {}", number);
    number + 2
}

#[tauri::command]
fn open_file_dialog() -> Result<Vec<File>, String> {
    let image_types = ["mp3"];
    let files = FileDialog::new()
        .add_filter("", &image_types)
        .set_directory("/Users/linya/Downloads/package")
        .pick_files();
    let mut file_vec = vec![];

    match files.clone() {
        Some(files_path) => {
            for (_, file_path) in files_path.iter().enumerate() {
                if let Ok(file) = File::new(file_path.clone()) {
                    file_vec.push(file);
                }

                // 获取文件的元数据
                match std::fs::metadata(&file_path) {
                    Ok(metadata) => {
                        println!("File size: {} bytes", metadata.len());
                        println!("Is file: {}", metadata.is_file());
                        println!("Is directory: {}", metadata.is_dir());
                    }
                    Err(e) => {
                        println!("Failed to get metadata: {}", e);
                    }
                }
            }
        }
        None => {
            todo!()
        }
    }

    Ok(file_vec)
}

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
