// use helixton_toolset;

use std::path::Path;
use std::process::exit;

extern crate walkdir;
use walkdir::WalkDir;

fn abort(msg: &str) {
    println!("{}", msg);
    exit(-1);
}

fn main() {
    println!("Hello, world! - from cli");

    // TODO: integrate path as argument
    let target_folder: &Path = Path::new("target/data");

    if !target_folder.exists() && !target_folder.is_dir() {
        abort("folder does not exists - abort");
    }

    let all_files = WalkDir::new(target_folder)
        .into_iter()
        .filter_map(|file| file.ok());

    for file in all_files {
        println!("file: {}", file.path().display());
    }
}
