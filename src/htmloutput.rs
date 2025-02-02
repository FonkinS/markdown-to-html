use crate::lexer::TOKEN;
use crate::lexer::LINETYPE;

struct Flags {
    bold: bool,
    italic: bool,
    code: bool,
}

fn flip_flag(flag: &mut bool) -> &str {
    if *flag {*flag = false; "</"} else {*flag = true; "<"}
}

fn convert_token<'a>(token: &TOKEN<String>, flags: &mut Flags) -> String {
    match token {
        TOKEN::TEXT(t) => t.to_string(),
        TOKEN::BOLD => (flip_flag(&mut flags.bold).to_owned() + "b>").to_string(),
        TOKEN::ITALIC => (flip_flag(&mut flags.italic).to_owned() + "i>").to_string(),
        TOKEN::BOLDITALIC => (flip_flag(&mut flags.bold).to_owned() + "b>" + flip_flag(&mut flags.italic) + "i>").to_string(),
        TOKEN::CODE => (flip_flag(&mut flags.code).to_owned() + "code>").to_string(),
        TOKEN::LINK(_t, _link) => "<a>".to_string(),
        TOKEN::LINEBREAK => "\n".to_string(),
        TOKEN::ESCAPE => "\\".to_string(),
        TOKEN::TABLESPLIT => "HELP".to_string()
    }
}

fn convert_tokens<'a>(tokens: &Vec<TOKEN<String>>, flags: &mut Flags) -> String {
    let mut s = String::new();
    for t in tokens {
        s.push_str(&convert_token(t, flags))
    }
    return s;
}

pub fn convert(line_types: Vec<LINETYPE>, line_tokens: Vec<Vec<TOKEN<String>>>) -> String {
    let mut flags : Flags = Flags {bold:false,italic:false,code:false};
    let mut html : Vec<String> = vec!();
    for l in 0..line_types.len() {
        let mut line = String::new();
        match line_types[l] {
            LINETYPE::LtH1 => {line += "<h1>"}
            LINETYPE::LtH2 => {line += "<h2>"}
            LINETYPE::LtH3 => {line += "<h3>"}
            LINETYPE::LtH4 => {line += "<h4>"}
            LINETYPE::LtH5 => {line += "<h5>"}
            LINETYPE::LtH6 => {line += "<h6>"}
            LINETYPE::LtEmpty => {line += "</p><p>"}
            _ => {}
        }
        line += &convert_tokens(&line_tokens[l], &mut flags);
        match line_types[l] {
            LINETYPE::LtH1 => {line += "</h1>"}
            LINETYPE::LtH2 => {line += "</h2>"}
            LINETYPE::LtH3 => {line += "</h3>"}
            LINETYPE::LtH4 => {line += "</h4>"}
            LINETYPE::LtH5 => {line += "</h5>"}
            LINETYPE::LtH6 => {line += "</h6>"}
            _ => {}
        }
        html.push(line);
    }

    return html.join("\n");
    /*for l in line_tokens {
        for t in l {
            match t {
                TOKEN::TEXT(x) => {print!("{x}")}
                TOKEN::TABLESPLIT => {print!(" | ")}
                _ => {print!("{:?}", t)}
            }
        }
        println!("");
    }*/
}
