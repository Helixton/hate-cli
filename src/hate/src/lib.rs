
use std::{process::exit};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use sha2::{Sha256, Digest};

extern crate walkdir;
use walkdir::{WalkDir, DirEntry};

pub fn info() -> String {
    String::from("hate - v0.1 - get shit done efficiently!")
}

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
    let all_files = self::get_all_files(target_folder);

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

pub fn vec_u8_to_hex_string(input: &Vec<u8>) -> String {
    let mut output = String::new();
    for byte in input {
        output.push_str(&format!("{:02x}", byte));
    }
    output
}

pub fn reverse(input: &String) -> String {
    return input.chars().rev().collect();
}

pub fn inspect(input: &String, digits: bool) -> (i32, String) {
    if !digits {
        return (input.len() as i32, String::from("char"));
    }
    return (inspect_numbers(input), String::from("digit"));
}

fn inspect_numbers(input: &String) -> i32 {
    let mut count = 0;
    for c in input.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }
    return count;
}