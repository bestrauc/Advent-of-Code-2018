use std::fs::File;
use std::io::prelude::*;

pub fn file_to_string(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Input data not found");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    file_contents
}
