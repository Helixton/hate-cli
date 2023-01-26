use std::path::Path;

use hate;

fn main() {
    // TODO: integrate path as argument
    let target_folder: &Path = Path::new("target/data");
    
    if !target_folder.exists() && !target_folder.is_dir() {
        hate::abort("folder does not exists - abort");
    }

    // let all_files = hate::get_all_files(&target_folder);

    let file_hash_map = hate::get_file_sha256_map(&target_folder);
    println!("{:?}", file_hash_map);
}
