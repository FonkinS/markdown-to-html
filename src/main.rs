use std::env;
use std::fs;
use std::io::Write;

// cargo run -- test.md


mod lexer;

mod htmloutput;


fn handle_error(error: String) {
    println!("🚧 Error: {}", error);
    return;
}

fn main() {
    // Get Args
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return handle_error("File path expected!".to_string());
    } else if args.len() == 2 {
        return handle_error("Output path expected!".to_string());
    }

    // Open File
    let file_contents = fs::read_to_string(&args[1]);
    if let Err(e) = file_contents {
        return handle_error(e.to_string());
    }
    let file_contents = file_contents.expect("Not error");

    // Parse lines
    let lines : Vec<&str> = file_contents.split("\n").collect();

    let tokens = lexer::analyze(&lines);

    //let (line_types, line_tokens) = lexer::analyze(&lines);
    let html = htmloutput::convert(tokens);

    let mut out_file = fs::File::create(&args[2]).unwrap();
    let _ = out_file.write_all(html.as_bytes());


}
