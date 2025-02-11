#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TOKEN<T> {
    TEXT(T),
    BOLD(bool),
    ITALIC(bool),
    CODE(bool),
    BLOCKCODE(bool),
    LINEBREAK,
    TABLE(bool),
    TABLEROW(bool),
    TABLECELL(bool),
    LINK {text: T, link:String, caption:String},
    IMAGE {caption: T, link: String},
    HORIZONTALLINE,
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
    EMPTYLINE,
}


struct Flags {
    in_bold: bool,
    in_italic: bool,
    in_inlinecode: bool,
    in_multilinecode: bool
}


fn tokenize_emphasis(line: &str, flags: &mut Flags) -> Vec<TOKEN<String>> {
    let mut tokens : Vec<TOKEN<String>> = vec!();
    let line = line.replace("_", "*").replace("**", "🤓"); // 🤓 is bold
    let mut chars = line.chars();
    let mut c = chars.next();
    let mut phrase = String::new();

    while !c.is_none() {
        let char = c.unwrap();
        if char == '*' {
            if phrase.len() != 0 {
                tokens.push(TOKEN::TEXT(phrase));
                phrase = String::new();
            }
            flags.in_italic = !flags.in_italic;
            tokens.push(TOKEN::ITALIC(flags.in_italic)); // False for now, we'll change later
        } else if char == '🤓' {
            if phrase.len() != 0 {
                tokens.push(TOKEN::TEXT(phrase));
                phrase = String::new();
            }
            flags.in_bold = !flags.in_bold;
            tokens.push(TOKEN::BOLD(flags.in_bold)); // False for now, we'll change later
        } else if char == '`' {
            if phrase.len() != 0 {
                tokens.push(TOKEN::TEXT(phrase));
                phrase = String::new();
            }
            flags.in_inlinecode = !flags.in_inlinecode;
            tokens.push(TOKEN::CODE(flags.in_inlinecode)); // False for now, we'll change later
        } else {
            phrase.push(char);
        }

        c = chars.next();
    }
    if phrase.len() != 0 {
        tokens.push(TOKEN::TEXT(phrase));
    }

    return tokens;
}

fn tokenize_header<'a>(line: &'a str, flags: &mut Flags) -> Option<Vec<TOKEN<String>>> {
    let mut header_depth = 0;
    let mut iter = line.chars();
    let mut prev = iter.clone();
    while iter.next().unwrap_or(' ') == '#' {
        header_depth += 1;
        prev.next();
    }
    if header_depth > 0 && header_depth <= 7 {
        let mut initial : TOKEN<String>;
        if header_depth == 1 {initial = TOKEN::HEADER1(true)}
        else if header_depth == 2 {initial = TOKEN::HEADER2(true)}
        else if header_depth == 3 {initial = TOKEN::HEADER3(true)}
        else if header_depth == 4 {initial = TOKEN::HEADER4(true)}
        else if header_depth == 5 {initial = TOKEN::HEADER5(true)}
        else if header_depth == 6 {initial = TOKEN::HEADER6(true)}
        else {return None}
        
        let mut tokens : Vec<TOKEN<String>> = vec!(initial.clone());
        tokens.append(&mut tokenize_emphasis(prev.as_str().trim_start(), flags));
        
        if header_depth == 1 {initial = TOKEN::HEADER1(false)}
        else if header_depth == 2 {initial = TOKEN::HEADER2(false)}
        else if header_depth == 3 {initial = TOKEN::HEADER3(false)}
        else if header_depth == 4 {initial = TOKEN::HEADER4(false)}
        else if header_depth == 5 {initial = TOKEN::HEADER5(false)}
        else if header_depth == 6 {initial = TOKEN::HEADER6(false)}
        tokens.push(initial);
        
        return Some(tokens);

    }
    return None;
}

fn tokenize_horizontalline<'a>(line: &'a str) -> Option<Vec<TOKEN<String>>> {
    let mut chars = line.chars();
    let mut this_char = chars.next();
    while this_char != None {
        let new_char = chars.next();
        if new_char == None {
            break;
        }
        if this_char != new_char && !new_char.unwrap().is_whitespace() {
            return None;
        }
        this_char = new_char;
    };

    let this_char = this_char.unwrap();
    if this_char == '=' || this_char == '-' || this_char == '#' || this_char == '*' {
        return Some(vec!(TOKEN::HORIZONTALLINE));
    };
    return None;
}

// TODO MULTI <p></p> MESS
// TODO PROPER CSS
fn tokenize_blockquote<'a>(line: &'a str) -> Option<Vec<TOKEN<String>>> {
    let mut chars = line.trim_start().chars();
    let mut quotecounter = 0;
    loop {
        match chars.next() {
            None => break,
            Some(c) => {
                if c == '>' {
                    quotecounter += 1;
                } else {
                    break;
                }
            }
        }
    }
    if quotecounter == 0 {
        return None;
    }
    let mut tokens : Vec<TOKEN<String>> = vec!();
    for _ in 0..quotecounter {
        tokens.push(TOKEN::BLOCKQUOTE(true));
    }
    let text = chars.as_str();
    tokens.push(TOKEN::TEXT(text.trim_start().to_string()));
    if text.to_string().ends_with("  ") {
        tokens.push(TOKEN::LINEBREAK);
    }
    for _ in 0..quotecounter {
        tokens.push(TOKEN::BLOCKQUOTE(false));
    }
    return Some(tokens);
}


fn tokenize_unorderedlist<'a>(line: &'a str, flags: &mut Flags) -> Option<Vec<TOKEN<String>>> {
    let mut iter = line.chars();
    let c = iter.next().unwrap();
    let n = iter.next().unwrap();
    if (c == '-' || c == '+' || c == '*' || c =='–') && n == ' ' {
        let mut tokens : Vec<TOKEN<String>> = vec!(TOKEN::LISTUNORDERED(true), TOKEN::LISTITEM(true));
        tokens.append(&mut tokenize_emphasis(iter.as_str().trim_start(), flags));
        tokens.push(TOKEN::LISTITEM(false));
        tokens.push(TOKEN::LISTUNORDERED(false));
        return Some(tokens);

    }
    return None;
}

fn tokenize_orderedlist<'a>(line: &'a str, flags: &mut Flags) -> Option<Vec<TOKEN<String>>> {
    let halves = line.split_once(".");
    if halves != None {
        let halves = halves.unwrap();
        if halves.0.parse::<f64>().is_ok() {
            if halves.1.chars().next().unwrap() == ' ' {
                let mut tokens : Vec<TOKEN<String>> = vec!(TOKEN::LISTORDERED(true), TOKEN::LISTITEM(true));
                tokens.append(&mut tokenize_emphasis(halves.1.trim_start(), flags));
                tokens.push(TOKEN::LISTITEM(false));
                tokens.push(TOKEN::LISTORDERED(false));
                return Some(tokens);
            }
        }
    }
    return None;
}

fn tokenize_code<'a>(line: &'a str, flags: &mut Flags) -> Option<Vec<TOKEN<String>>> {
    if line.starts_with("```") {
        flags.in_multilinecode = !flags.in_multilinecode;
        return Some(vec!(TOKEN::CODE(flags.in_multilinecode)));
    } else if line.starts_with("    ") || line.starts_with("\t") {
        return Some(vec!(
            TOKEN::CODE(true), 
            TOKEN::TEXT(line.trim_start().to_string()), 
            TOKEN::LINEBREAK,
            TOKEN::CODE(false)));
    }
    return None;
}

fn tokenize_image<'a>(line: &'a str) -> Option<Vec<TOKEN<String>>> {
    if line.starts_with("!") {
        if line.contains("(") && line.contains(")") && line.contains("[") && line.contains("]") {
            return Some(vec!(TOKEN::IMAGE{caption: line.to_string(), link: "www.google.com".to_string()}));
        }
    }
    return None;
}


/*fn check_table_split<'a>(line: &'a str) -> Option<Vec<TOKEN<String>>> {
    let changedline = line.trim().replace(" ", "").replace("|", "").replace("-", "").replace("–", "").replace(":", "");
    if changedline.len() == 0 {
        return Some(("TABLE SPLIT", LINETYPE::LtTableSplit));
    }
    return None;
}


fn tokenize_table<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    let trimmedline = line.trim();
    if trimmedline.starts_with("|") && trimmedline.ends_with("|") {
        return Some((trimmedline, LINETYPE::LtTable));
    }
    return None;
}*/
// TODO TABLES


// TODO macro?
fn implement_tokens(tokens: Option<Vec<TOKEN<String>>>, list : &mut Vec<TOKEN<String>>) -> bool {
    match tokens {
        Some(mut x) => {list.append(&mut x); true},
        None => false
    }
}


fn fix_multilines(tokens: Vec<TOKEN<String>>) -> Vec<TOKEN<String>> {
    let oldtokens = tokens.clone();
    let mut tokens : Vec<TOKEN<String>> = vec!();
    let mut i = 0;
    loop {
        let mut fixed = true;
        while i < oldtokens.len() - 1 {
            if let TOKEN::BLOCKQUOTE(f) = oldtokens[i] {
                if let TOKEN::BLOCKQUOTE(s) = oldtokens[i+1] {
                    if f == false && s == true {
                        i += 2;
                        fixed = false;
                        continue;
                    }
                }
            }
            if let TOKEN::BLOCKCODE(f) = oldtokens[i] {
                if let TOKEN::BLOCKCODE(s) = oldtokens[i+1] {
                    if f == false && s == true {
                        i += 2;
                        fixed = false;
                        continue;
                    }
                }
            }
            if let TOKEN::LISTORDERED(f) = oldtokens[i] {
                if let TOKEN::LISTORDERED(s) = oldtokens[i+1] {
                    if f == false && s == true {
                        i += 2;
                        fixed = false;
                        continue;
                    }
                }
            }
            if let TOKEN::LISTUNORDERED(f) = oldtokens[i] {
                if let TOKEN::LISTUNORDERED(s) = oldtokens[i+1] {
                    if f == false && s == true {
                        i += 2;
                        fixed = false;
                        continue;
                    }
                }
            }
            tokens.push(oldtokens[i].clone());
            i += 1;
        }
        if fixed {
            break;
        }
    }
    tokens.push(oldtokens[i].clone());
    return tokens;
}



// TODO backslash escaping
pub fn analyze(lines: &Vec<&str>) -> Vec<TOKEN<String>> {
    //let (line_types, line_contents) = analyze_lines(lines);
    //let line_tokens = create_tokens(&line_contents, &line_types);
    let mut flags = Flags {
        in_bold: false,
        in_italic: false,
        in_inlinecode: false,
        in_multilinecode: false
    };
    let mut tokens : Vec<TOKEN<String>> = vec!();
    for line in lines {
        if line.trim().len() == 0 {tokens.push(TOKEN::EMPTYLINE);continue}
        let trimline = line.trim();
        if implement_tokens(tokenize_header(trimline, &mut flags), &mut tokens) {continue} 
        if implement_tokens(tokenize_horizontalline(trimline), &mut tokens) {continue}
        if implement_tokens(tokenize_blockquote(line), &mut tokens) {continue}
        if implement_tokens(tokenize_unorderedlist(trimline, &mut flags), &mut tokens) {continue}
        if implement_tokens(tokenize_orderedlist(trimline, &mut flags), &mut tokens) {continue}
        if implement_tokens(tokenize_code(line, &mut flags), &mut tokens) {continue}
        if implement_tokens(tokenize_image(trimline), &mut tokens) {continue}
        implement_tokens(Some(tokenize_emphasis(trimline, &mut flags)), &mut tokens);
        
        if line.ends_with("  ") || flags.in_multilinecode {
            tokens.push(TOKEN::LINEBREAK)
        }
    }


    // Fix Indent code and blockquotes
    let tokens = fix_multilines(tokens);

    //println!("{tokens:?}");
    

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
   
    return tokens;
    //println!("{:?}", line_contents);
}
