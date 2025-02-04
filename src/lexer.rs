
pub enum LINETYPE {
    LtP,
    LtH1,
    LtH2,
    LtH3,
    LtH4,
    LtH5,
    LtH6,
    LtEmpty,
    LtLine,
    LtQuote,
    LtUL,
    LtOL,
    LtCodeChange,
    LtCodeIndent,
    LtImage,
    LtTable,
    LtTableSplit,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TOKEN<T> {
    TEXT(T),
    BOLD,
    ITALIC,
    BOLDITALIC,
    CODE,
    LINK(T, String),
    LINEBREAK, // TODO
    ESCAPE, // TODO
    TABLESPLIT
}

#[allow(dead_code)]
enum OTHERTOKEN<T> {
    TEXT(T),
    BOLD(bool),
    ITALIC(bool),
    CODE(bool),
    BLOCKCODE(bool),
    LINEBREAK,
    TABLEROW(bool),
    TABLECELL(bool),
    LINK {text: T, link:String, caption:String},
    IMAGE {caption: T, link: String},
    LISTORDERED(bool),
    LISTUNORDERED(bool),
    LISTITEM(bool),
    BLOCKQUOTE(bool),
    HEADER1(bool),
    HEADER2(bool),
    HEADER3(bool),
    HEADER4(bool),
    HEADER5(bool),
    HEADER6(bool),
}


mod line_check;

fn implement<'a>(val: Option<(&'a str, LINETYPE)>, lc: &mut Vec<&'a str>, lt: &mut Vec<LINETYPE>) -> bool {
    match val {
        Some(x) => {
            lc.push(x.0);
            lt.push(x.1);
            return true;
        }
        None => {return false;}
    }
}

fn analyze_lines<'a>(lines: &Vec<&'a str>) -> (Vec<LINETYPE>, Vec<&'a str>){
    let mut lt : Vec<LINETYPE> = vec!(); 
    let mut lc : Vec<&str> = vec!();

    for l in 0..lines.len() {
        let line = lines[l].trim_start();

        if implement(line_check::check_empty(line), &mut lc, &mut lt) {continue;}
        if implement(line_check::check_header(line), &mut lc, &mut lt) {continue;}
        if implement(line_check::check_line(line), &mut lc, &mut lt) {continue;}
        if implement(line_check::check_blockquote(line), &mut lc, &mut lt) {continue;}
        if implement(line_check::check_unorderedlist(line), &mut lc, &mut lt) {continue;}
        if implement(line_check::check_orderedlist(line), &mut lc, &mut lt) {continue;}
        if implement(line_check::check_code_indent(lines[l]), &mut lc, &mut lt) {continue;}
        if implement(line_check::check_code_change(line), &mut lc, &mut lt) {continue;}
        if implement(line_check::check_image(line), &mut lc, &mut lt) {continue;}
        if implement(line_check::check_table_split(line), &mut lc, &mut lt) {continue;}
        if implement(line_check::check_table(line), &mut lc, &mut lt) {continue;}
        implement(line_check::check_paragraph(line), &mut lc, &mut lt);
    }
    assert!(lc.len() == lines.len());
    assert!(lc.len() == lt.len());

    return (lt, lc);

}

fn char_is_emphasis(c: char) -> bool {
    c == '*' || c == '_'
}

fn check_emphasis(prev_chars: [char;3]) -> Option<TOKEN<String>> {
    if char_is_emphasis(prev_chars[0]) {
        if char_is_emphasis(prev_chars[1]) {
            if char_is_emphasis(prev_chars[2]) {
                return Some(TOKEN::BOLDITALIC);
            }
            return Some(TOKEN::BOLD);
        }
        return Some(TOKEN::ITALIC);
    }
    return None;
}

// TODO Proper Code Support
fn analyze_tokens(line: &str) -> Vec<TOKEN<String>> {
    let mut tokens : Vec<TOKEN<String>> = vec!();
    let mut line_chars = line.chars();
    let mut prev_chars: [char;3] = [' ';3];
    let mut current_token : String = String::new();
    loop {
        let this_char : char;
        match line_chars.next() {
            None => break,
            Some(x) => this_char = x
        }

        if char_is_emphasis(this_char) {
            if !char_is_emphasis(prev_chars[0]) {
                tokens.push(TOKEN::TEXT(current_token.clone()));
                current_token = String::new();
            }
        } else {
            match check_emphasis(prev_chars) {
                None => {}
                Some(x) => {tokens.push(x);current_token = String::new()}
            }
        }

        current_token.push(this_char);

        prev_chars[2] = prev_chars[1];
        prev_chars[1] = prev_chars[0];
        prev_chars[0] = this_char;
    }

    match check_emphasis(prev_chars) {
        None => tokens.push(TOKEN::TEXT(current_token)),
        Some(x) => tokens.push(x)
    }

    return tokens;
}

fn analyze_table(line: &str) -> Vec<TOKEN<String>> {
    let mut tokens : Vec<TOKEN<String>> = vec!(TOKEN::TABLESPLIT);
    let line : Vec<&str> = line.split("|").collect::<Vec<&str>>()
        .iter().map(|x| x.trim()).collect::<Vec<&str>>()
        .into_iter().filter(|x| x.len() > 0).collect();
    for c in line {
        let tok = analyze_tokens(c);
        for t in tok {
            tokens.push(t);
        }
        tokens.push(TOKEN::TABLESPLIT);
    }
    return tokens;
}

fn create_tokens(line_contents: &Vec<&str>, line_types: &Vec<LINETYPE>) -> Vec<Vec<TOKEN<String>>> {
    let mut line_tokens : Vec<Vec<TOKEN<String>>> = vec!();
    for l in 0..line_contents.len() {
        match line_types[l] {
            LINETYPE::LtImage => line_tokens.push(vec!()),
            LINETYPE::LtCodeChange => line_tokens.push(vec!()),
            LINETYPE::LtCodeIndent => line_tokens.push(vec!(TOKEN::TEXT(line_contents[l].to_string()))),
            LINETYPE::LtTableSplit => line_tokens.push(vec!()),
            LINETYPE::LtEmpty => line_tokens.push(vec!()),
            LINETYPE::LtLine => line_tokens.push(vec!()),
            LINETYPE::LtTable => line_tokens.push(analyze_table(line_contents[l])),
            _ => line_tokens.push(analyze_tokens(line_contents[l])),
        }
    }

    return line_tokens;

}


pub fn analyze(lines: &Vec<&str>) -> (Vec<LINETYPE>, Vec<Vec<TOKEN<String>>>) {
    let (line_types, line_contents) = analyze_lines(lines);
    let line_tokens = create_tokens(&line_contents, &line_types);

    /*for l in &line_tokens {
        for t in l {
            match t {
                TOKEN::TEXT(x) => {print!("{x}")}
                TOKEN::TABLESPLIT => {print!(" | ")}
                _ => {print!("{:?}", t)}
            }
        }
        println!("");
    }*/
   
    return (line_types, line_tokens);
    //println!("{:?}", line_contents);
}
