use std::fs;
use std::io::Write;

use clap::Parser;

// cargo run -- test.md


mod lexer;

mod htmloutput;


fn handle_error(error: String) {
    println!("🚧 Error: {}", error);
    return;
}

// turn a markdown or MD file into a HTML one
#[derive(Parser)]
struct Cli {
    // The filepath for the markdown input
    in_path: std::path::PathBuf,
    // The filepath for the markdown Output
    #[arg(short='o', long="output")]
    out_path: std::path::PathBuf,
    // the optional filepath for CSS styling
    #[arg(long="css")]
    css_path: Option<std::path::PathBuf>,
    // Flag for whether uft-8 should be allowed
    #[arg(long)]
    utf8: bool,
    // Flag for whether html structure should be included
    #[arg(long)]
    head: bool,
}

fn main() {
    // Get Args
    let args = Cli::parse();

    // Open File
    let file_contents = fs::read_to_string(args.in_path);
    if let Err(e) = file_contents {
        return handle_error(e.to_string());
    }
    let file_contents = file_contents.expect("Not error");

    // Parse lines
    let lines : Vec<&str> = file_contents.split("\n").collect();

    let tokens = lexer::analyze(&lines);

    //let (line_types, line_tokens) = lexer::analyze(&lines);

    let mut html = String::new();
    if args.head {
        html.push_str("<!DOCTYPE html><html><body>");
    }
    if args.utf8 {
        html.push_str("<meta charset=\"UTF-8\">");
    }
    if args.css_path.is_some() {
        html.push_str("<link rel=\"stylesheet\" href=\"");
        html.push_str(args.css_path.unwrap().to_str().unwrap());
        html.push_str("\">");
    }

    html.push_str(&htmloutput::convert(tokens));
    if args.head {
        html.push_str("</body></html>");
    }

    let mut out_file = fs::File::create(args.out_path).unwrap();
    let _ = out_file.write_all(html.as_bytes());


}
