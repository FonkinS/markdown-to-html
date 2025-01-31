use std::env;
use std::fs;

// cargo run -- test.md


use crate::lexer::analyze;

mod lexer;

fn handle_error(error: String) {
    println!("🚧 Error: {}", error);
    return;
}

fn main() {
    // Get Args
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return handle_error("File path expected!".to_string());
    }

    // Open File
    let file_contents = fs::read_to_string(&args[1]);
    if let Err(e) = file_contents {
        return handle_error(e.to_string());
    }
    let file_contents = file_contents.expect("Not error");

    // Parse lines
    let lines : Vec<&str> = file_contents.split("\n").collect();

    analyze(&lines);
}
