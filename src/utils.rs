use std::path::{Path, PathBuf};
use std::fs::File;
extern crate dirs;

pub fn get_db_path() -> std::fs::File{
    let home_path:std::option::Option<PathBuf> = dirs::home_dir();
    let file_path:PathBuf = Path::new(&home_path.unwrap())
        .join("Ideas")
        .join("SuperDaemon")
        .join("src")
        .join("data.db");

    let json_file_path = file_path.as_path();
    let file = File::open(json_file_path).expect("file not found");
    return file;
}