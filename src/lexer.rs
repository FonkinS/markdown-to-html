#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TOKEN<T> {
    TEXT(T),
    BOLD(bool),
    ITALIC(bool),
    STRIKETHROUGH(bool),
    CODE(bool),
    HIGHLIGHT(bool),
    SUBSCRIPT(bool),
    SUPERSCRIPT(bool),
    BLOCKCODE(bool),
    LINEBREAK,
    TABLE(bool),
    TABLEROW(bool),
    TABLEDATACELL(bool),
    TABLEHEADERCELL(bool),
    LINK(bool, String),
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
    in_strikethrough: bool,
    in_inlinecode: bool,
    in_multilinecode: bool,
    in_table_header: bool,
    in_highlight: bool,
    in_superscript: bool,
    in_subscript: bool,
}


fn push_text<'a>(tokens: &mut Vec<TOKEN<String>>, phrase: String) -> String {
    if phrase.len() != 0 {
        tokens.push(TOKEN::TEXT(phrase.replace("⌘", "__").replace("⎋", "**")
                    .replace("⏎", "_").replace("↩︎", "*")
                    .replace("⇢","~").replace("→", "^")
                    .replace("⏏︎", "~~").replace("⇥", "[")
                    .replace("⇤", "](").replace("⚙︎", "==")
                ));
    }
    return String::new();
}

fn tokenize_emphasis(line: &str, flags: &mut Flags) -> Vec<TOKEN<String>> {
    let mut tokens : Vec<TOKEN<String>> = vec!();
    let line = line.replace("\\__", "⌘").replace("\\**", "⎋")
                    .replace("\\_", "⏎").replace("\\*", "↩︎")
                    .replace("\\~", "⇢").replace("\\^", "→")
                    .replace("\\~~", "⏏︎").replace("\\[", "⇥")
                    .replace("\\](", "⇤").replace("\\==", "⚙︎");
    let line = line.replace("_", "*").replace("**", "⇧").replace("~~", "⌤").replace("](", "⌬").replace("==", "⌆"); // ⇧ is bold;⌤ is striketrhogu ⌬ is middle of link
    let mut chars = line.chars();
    let mut c = chars.next();
    let mut phrase = String::new();

    let mut inlink = false;
    let mut old_tokens : Vec<TOKEN<String>> = vec!();

    while !c.is_none() {
        let char = c.unwrap();
        if inlink {
            if char == ')' {
                inlink = false;
                old_tokens.push(TOKEN::LINK(true, phrase.clone()));
                old_tokens.append(&mut tokens);
                old_tokens.push(TOKEN::LINK(false, phrase.clone()));
                tokens = old_tokens.clone();
                old_tokens = vec!();
                phrase = String::new();
            } else {
                phrase.push(char);
            }
        } else if char == '*' {
            phrase = push_text(&mut tokens, phrase); // returns empty
            flags.in_italic = !flags.in_italic;
            tokens.push(TOKEN::ITALIC(flags.in_italic)); 
        } else if char == '⇧' {
            phrase = push_text(&mut tokens, phrase); // returns empty
            flags.in_bold = !flags.in_bold;
            tokens.push(TOKEN::BOLD(flags.in_bold)); 
        } else if char == '`' {
            phrase = push_text(&mut tokens, phrase); // returns empty
            flags.in_inlinecode = !flags.in_inlinecode;
            tokens.push(TOKEN::CODE(flags.in_inlinecode)); 
        } else if char == '⌆' {
            phrase = push_text(&mut tokens, phrase); // returns empty
            flags.in_highlight = !flags.in_highlight;
            tokens.push(TOKEN::HIGHLIGHT(flags.in_highlight)); 
        } else if char == '~' {
            phrase = push_text(&mut tokens, phrase); // returns empty
            flags.in_subscript = !flags.in_subscript;
            tokens.push(TOKEN::SUBSCRIPT(flags.in_subscript)); 
        } else if char == '^' {
            phrase = push_text(&mut tokens, phrase); // returns empty
            flags.in_superscript = !flags.in_superscript;
            tokens.push(TOKEN::SUPERSCRIPT(flags.in_superscript)); 
        } else if char == '⌤' {
            phrase = push_text(&mut tokens, phrase); // returns empty
            flags.in_strikethrough = !flags.in_strikethrough;
            tokens.push(TOKEN::STRIKETHROUGH(flags.in_strikethrough)); 
        } else if char == '[' && !flags.in_inlinecode && !flags.in_multilinecode {
            phrase = push_text(&mut tokens, phrase); // returns empty
            old_tokens = tokens.clone();
            tokens = vec!();
        } else if char == '⌬' {
            inlink = true;
            phrase = push_text(&mut tokens, phrase); // returns empty
        } else {
            phrase.push(char);
        }

        c = chars.next();
    }
    push_text(&mut tokens, phrase);

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
    tokens.push(TOKEN::TEXT(text.trim().to_string()));
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
    let c = iter.next();
    if c.is_none() {return None;}
    let c = c.unwrap();
    let n = iter.next();
    if n.is_none() {return None;}
    let n = n.unwrap();
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
        return Some(vec!(TOKEN::BLOCKCODE(flags.in_multilinecode)));
    } else if line.starts_with("    ") || line.starts_with("\t") {
        return Some(vec!(
            TOKEN::BLOCKCODE(true), 
            TOKEN::TEXT(line.trim_start().to_string()), 
            TOKEN::LINEBREAK,
            TOKEN::BLOCKCODE(false)));
    }
    return None;
}

fn tokenize_image<'a>(line: &'a str) -> Option<Vec<TOKEN<String>>> {
    if line.starts_with("!") {
        if line.contains("(") && line.contains(")") && line.contains("[") && line.contains("]") {
            return Some(vec!(TOKEN::IMAGE{
                caption: 
                    line.split("[").collect::<Vec<&str>>()[1].split("]").collect::<Vec<&str>>()[0].to_string(),
                link: 
                    line.split("(").collect::<Vec<&str>>()[1].split(")").collect::<Vec<&str>>()[0].to_string()
            }));
        }
    }
    return None;
}


fn tokenize_table<'a>(line: &'a str, flags: &mut Flags) -> Option<Vec<TOKEN<String>>> {
    let trimmedline = line.trim();
    if !trimmedline.starts_with("|") || !trimmedline.ends_with("|") {
        return None;
    }

    
    let mut is_split = true;
    for c in trimmedline.chars() {
        if c != '-' && c != '–' && c != '|' && !c.is_whitespace() && c != ':' {
            is_split = false;
            break;
        }
    }
    if is_split {
        flags.in_table_header = false;
        return Some(vec!());
    }

    let mut tokens : Vec<TOKEN<String>> = vec!(TOKEN::TABLE(true), TOKEN::TABLEROW(true));

    let cells : Vec<&str> = trimmedline.split("|").collect();
    for c in 1..cells.len()-1 {
        if flags.in_table_header {
            tokens.push(TOKEN::TABLEHEADERCELL(true));
        } else {
            tokens.push(TOKEN::TABLEDATACELL(true));
        }
        tokens.append(&mut tokenize_emphasis(&mut cells[c].trim(), flags));
        if flags.in_table_header {
            tokens.push(TOKEN::TABLEHEADERCELL(false));
        } else {
            tokens.push(TOKEN::TABLEDATACELL(false));
        }
    }



    tokens.push(TOKEN::TABLEROW(false));
    tokens.push(TOKEN::TABLE(false));
    return Some(tokens);
}
// TODO TABLE CENTERING


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
            if let TOKEN::TABLE(f) = oldtokens[i] {
                if let TOKEN::TABLE(s) = oldtokens[i+1] {
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


fn interpret_emojis(tokens: Vec<TOKEN<String>>) -> Vec<TOKEN<String>> {
    let mut emojis : Vec<(&str, &str)> = vec!();
    {
        let emojis_src : Vec<&str> = include_str!("../assets/emojis.txt").split("\n").collect();
        for l in emojis_src {
            let e : Vec<&str> = l.split(",").collect();
            if e.len() != 2 {break}
            emojis.push((e[0], e[1]));
        }
    }
    let mut new_tokens : Vec<TOKEN<String>> = vec!();
    for t in tokens {
        match t {
            TOKEN::TEXT(s) => {
                let mut t = s.replace("\\:", "⏏︎")/*.replace(": ", "⍝")*/.replace("\\", "").replace("⌦", "\\");
                for e in &emojis {
                    t = t.replace(e.0, e.1);
                }
                new_tokens.push(TOKEN::TEXT(t.replace("⏏︎", ":")/*.replace("⍝", ": ")*/.replace("🅱️🅰️Ⓜ️", "")));
            },
            _ => new_tokens.push(t),
        }
    }
    return new_tokens;
}


fn tokenize(lines: &Vec<&str>) -> Vec<TOKEN<String>> {
    let mut flags = Flags {
        in_bold: false,
        in_italic: false,
        in_strikethrough: false,
        in_inlinecode: false,
        in_multilinecode: false,
        in_table_header: false,
        in_highlight: false,
        in_subscript: false,
        in_superscript: false,
    };
    let mut tokens : Vec<TOKEN<String>> = vec!();
    for line in lines {
        let line = line.replace("\\\\", "⌦");
        if line.trim().len() == 0 && !flags.in_multilinecode {tokens.push(TOKEN::EMPTYLINE);continue}
        let trimline = line.trim();
        if implement_tokens(tokenize_orderedlist(trimline, &mut flags), &mut tokens) {continue}
        if implement_tokens(tokenize_unorderedlist(trimline, &mut flags), &mut tokens) {continue}
        if implement_tokens(tokenize_code(&line, &mut flags), &mut tokens) {continue}
        if !flags.in_multilinecode {
            if implement_tokens(tokenize_table(trimline, &mut flags), &mut tokens) {continue}
            flags.in_table_header = true;
            if implement_tokens(tokenize_header(trimline, &mut flags), &mut tokens) {continue} 
            if implement_tokens(tokenize_horizontalline(trimline), &mut tokens) {continue}
            if implement_tokens(tokenize_blockquote(&line), &mut tokens) {continue}
            if implement_tokens(tokenize_image(trimline), &mut tokens) {continue}
            implement_tokens(Some(tokenize_emphasis(trimline, &mut flags)), &mut tokens);
        } else  {
            tokens.push(TOKEN::TEXT(line.clone()));
        }
        
        if line.ends_with("  ") || flags.in_multilinecode {
            tokens.push(TOKEN::LINEBREAK)
        }
    }
    return tokens;
}


// TODO REGEX
pub fn analyze(lines: &Vec<&str>) -> Vec<TOKEN<String>> {
    let tokens = tokenize(lines);
    let tokens = fix_multilines(tokens);
    let tokens = interpret_emojis(tokens);

    return tokens;
}
