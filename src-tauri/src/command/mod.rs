#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::file::CustomFile;
use rfd::FileDialog;

#[tauri::command]
pub fn backend_add(number: i32) -> i32 {
    println!(
        "This is just a demo: Backend was called with an argument: {}",
        number
    );
    number + 2
}

#[tauri::command]
pub fn open_file_dialog() -> Result<Vec<CustomFile>, String> {
    let image_types = ["mp3"];
    let files = FileDialog::new()
        .add_filter("", &image_types)
        .set_directory("/Users/linya/Downloads/package")
        .pick_files();
    let mut file_vec = vec![];

    match files.clone() {
        Some(files_path) => {
            for (_, file_path) in files_path.iter().enumerate() {
                if let Ok(file) = CustomFile::new(file_path.clone()) {
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
