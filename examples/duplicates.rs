

// fn reverse(data: &str) -> String {
//     let mut new_data: String = String::new();
//     for i in (0..data.len()-1).rev() {
//         let element = data.chars().nth(i).unwrap();
//         new_data.push(element);
//     }
//     return new_data;
// }

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}

fn find_duplicates() {
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

fn main() {
    let data: &str = "Hello";
    println!("{}", reverse(&data));
}