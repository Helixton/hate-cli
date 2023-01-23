
use std::process::exit;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use sha2::{Sha256, Digest};

extern crate walkdir;
use walkdir::{WalkDir, DirEntry};

pub fn hello() {
    println!("Hello, world! - from lib");
}

pub fn abort(msg: &str) {
    println!("{}", msg);
    exit(-1);
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn get_all_files(target_folder: &Path) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(target_folder)
        .into_iter()
        .filter_map(|file| file.ok())
}

pub fn get_file_sha256_map(target_folder: &Path) -> HashMap<String, Vec<u8>> {
    let all_files = get_all_files(target_folder);

    let mut file_hash_map: HashMap<String, Vec<u8>> = HashMap::new();

    for file in all_files {

        let file_path = file.path();

        if !file_path.is_file() {
            continue;
        }

        let file_content = fs::read(file_path).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(file_content);
        let file_hash = hasher.finalize();

        file_hash_map.insert(file_path.display().to_string(), file_hash.to_vec());
        
    }

    file_hash_map
}