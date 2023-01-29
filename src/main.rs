use std::path::Path;

use hate;

fn main() {
    // TODO: integrate path as argument
    let target_folder: &Path = Path::new("target/data");
    
    if !target_folder.exists() && !target_folder.is_dir() {
        hate::abort("folder does not exists - abort");
    }

    let file_hash_map = hate::get_file_sha256_map(&target_folder);
    
    
    
    // println!("{:?}", file_hash_map);


    for file in file_hash_map {
        println!("filepath {} - SHA256: {:?}", file.0, hate::vec_u8_to_hex_string(&file.1));
    }
}



