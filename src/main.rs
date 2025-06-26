use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("File path: {file_path}");
    // read text from file
    let text = fs::read_to_string(file_path)
        .expect("ENOENT");

    println!("{text}");
}
