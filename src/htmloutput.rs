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
            TOKEN::STRIKETHROUGH(b) => bool_to_html("del", b),
            TOKEN::CODE(b) => bool_to_html("code", b),
            TOKEN::BLOCKCODE(b) => bool_to_html("code", b),
            TOKEN::HORIZONTALLINE => String::from("<hr/>"),
            TOKEN::LINEBREAK => String::from("<br/>"),
            TOKEN::TABLE(b) => bool_to_html("table", b),
            TOKEN::TABLEROW(b) => bool_to_html("tr", b),
            TOKEN::TABLEDATACELL(b) => bool_to_html("td", b),
            TOKEN::TABLEHEADERCELL(b) => bool_to_html("th", b),
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


    // Fix paragraph <p>s
    // TODO FIX THIS ABSOLUTE HORROR OF CODE
    let out_string = out_string.replace("<p><h1>", "<h1>")
                        .replace("<p><h2>", "<h2>")
                        .replace("<p><h3>", "<h3>")
                        .replace("<p><h4>", "<h4>")
                        .replace("<p><h5>", "<h5>")
                        .replace("<p><h6>", "<h6>")
                        .replace("</h1></p>", "</h1>")
                        .replace("</h2></p>", "</h2>")
                        .replace("</h3></p>", "</h3>")
                        .replace("</h4></p>", "</h4>")
                        .replace("</h5></p>", "</h5>")
                        .replace("</h6></p>", "</h6>")
                        .replace("<p><code>", "<code>")
                        .replace("</code></p>", "</code>")
                        .replace("<p><table>", "<table>")
                        .replace("</table></p>", "</table>")
                        .replace("<p><ol>", "<ol>")
                        .replace("</ol></p>", "</ol>")
                        .replace("<p><ul>", "<ul>")
                        .replace("</ul></p>", "</ul>")
                        .replace("<p><blockquote>", "<blockquote>")
                        .replace("</blockquote></p>", "</blockquote>")
                        .replace("<p><hr/>", "<hr/>")
                        .replace("<hr/></p>", "<hr/>");

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
