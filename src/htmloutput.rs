use crate::lexer::TOKEN;

fn bool_to_html(t: &str, b: bool) -> String {
    if b {
        return "<".to_owned() + t + ">";
    } else {
        return "</".to_owned() + t + ">";
    }
}

pub fn convert(tokens: Vec<TOKEN<String>>) -> String {
    let mut out_string = String::from("<p>");

    for t in tokens {
        out_string.push_str(&match t {
            TOKEN::TEXT(t) => t,
            TOKEN::BOLD(b) => bool_to_html("b", b),
            TOKEN::ITALIC(b) => bool_to_html("i", b),
            TOKEN::CODE(b) => bool_to_html("code", b),
            TOKEN::BLOCKCODE(b) => bool_to_html("code", b),
            TOKEN::HORIZONTALLINE => String::from("<hr/>"),
            TOKEN::LINEBREAK => String::from("<br/>"),
            TOKEN::TABLE(_) => String::new(),
            TOKEN::TABLEROW(_) => String::new(),
            TOKEN::TABLECELL(_) => String::new(),
            TOKEN::LINK{text: _t, link:_l, caption:_c} => String::new(),
            TOKEN::IMAGE{caption:_c, link:_l} => String::new(),
            TOKEN::LISTORDERED(b) => bool_to_html("ol", b),
            TOKEN::LISTUNORDERED(b) => bool_to_html("ul", b),
            TOKEN::LISTITEM(b) => bool_to_html("li", b),
            TOKEN::BLOCKQUOTE(b) => bool_to_html("blockquote", b),
            TOKEN::HEADER1(b) => bool_to_html("h1", b),
            TOKEN::HEADER2(b) => bool_to_html("h2", b),
            TOKEN::HEADER3(b) => bool_to_html("h3", b),
            TOKEN::HEADER4(b) => bool_to_html("h4", b),
            TOKEN::HEADER5(b) => bool_to_html("h5", b),
            TOKEN::HEADER6(b) => bool_to_html("h6", b),
            TOKEN::EMPTYLINE => String::from("</p><p>"),
        });

    }
    out_string.push_str("</p>");

    return out_string;

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
