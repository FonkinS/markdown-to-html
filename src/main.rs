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
    #[arg(long)]
    custombody: Option<std::path::PathBuf>,
    #[arg(short, long)]
    title: Option<String>,
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
    let mut include_head = false;
    let mut custombody_end = String::new();
    if args.title.is_some() || args.utf8 || args.css_path.is_some() || args.custombody.is_some() {
        include_head = true;
        html.push_str("<!DOCTYPE html><html><head>");
        if args.title.is_some() {
            html.push_str("<title>");
            html.push_str(&args.title.unwrap());
            html.push_str("</title>");
        }
        if args.utf8 {
            html.push_str("<meta charset=\"UTF-8\">");
        }
        if args.css_path.is_some() {
            html.push_str("<link rel=\"stylesheet\" href=\"");
            html.push_str(args.css_path.unwrap().to_str().unwrap());
            html.push_str("\">");
        }
        if args.custombody.is_some() {
            let p = fs::read_to_string(args.custombody.unwrap());
            if let Err(e) = p {
                return handle_error(e.to_string());
            }
            let p = p.expect("not error");
            let p = p.split("\n").collect::<Vec<&str>>();
            html.push_str(p[0]);
            custombody_end = p[1].to_string();
        }
        html.push_str("</head><body>");
    }

    html.push_str(&htmloutput::convert(tokens));
    if include_head {
        if custombody_end.len() != 0 {
            html.push_str(&custombody_end);
        }
        html.push_str("</body></html>");
    }

    let mut out_file = fs::File::create(args.out_path).unwrap();
    let _ = out_file.write_all(html.as_bytes());


}
