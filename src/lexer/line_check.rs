
use crate::lexer::LINETYPE;
/*pub (super) fn check_header<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    let mut header_depth = 0;
    let mut iter = line.chars();
    let mut prev = iter.clone();
    while iter.next().unwrap_or(' ') == '#' {
        header_depth += 1;
        prev.next();
    }
    if header_depth > 0 && header_depth <= 7 {
        let line_type : LINETYPE;
        if header_depth == 1 {line_type = LINETYPE::LtH1}
        else if header_depth == 2 {line_type = LINETYPE::LtH2}
        else if header_depth == 3 {line_type = LINETYPE::LtH3}
        else if header_depth == 4 {line_type = LINETYPE::LtH4}
        else if header_depth == 5 {line_type = LINETYPE::LtH5}
        else if header_depth == 6 {line_type = LINETYPE::LtH6}
        else {return None}
        return Some((prev.as_str().trim_start(), line_type));
    }
    return None;
}

/*pub (super) fn check_header_line<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    let mut line = line.trim();
    let mut chars = line.chars();
    let mut this_char = chars.next();
    while this_char != None {
        let new_char = chars.next();
        if this_char != new_char {
            return None;
        }
    };

    if this_char.unwrap() == '=' {
        return Some(("", LINETYPE::LtPrevH1));
    };
    if this_char.unwrap() == '-' {
        return Some(("", LINETYPE::LtPrevH2));
    };
}*/


pub (super) fn check_line<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
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
        return Some(("LINE", LINETYPE::LtLine));
    };
    return None;
}


pub (super) fn check_empty<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    if line.len() == 0 {
        return Some(("EMPTY", LINETYPE::LtEmpty));
    }
    return None;
}

pub (super) fn check_blockquote<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    let mut chars = line.chars();
    if chars.next().unwrap() == '>' {
        return Some((chars.as_str().trim(), LINETYPE::LtQuote));
    }
    return None;
}

pub (super) fn check_unorderedlist<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    let mut iter = line.chars();
    let c = iter.next().unwrap();
    let n = iter.next().unwrap();
    if (c == '-' || c == '+' || c == '*' || c =='–') && n == ' ' {
        return Some((iter.as_str().trim_start(), LINETYPE::LtUL));
    }
    return None;
}

pub (super) fn check_orderedlist<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    let halves = line.split_once(".");
    if halves != None {
        let halves = halves.unwrap();
        if halves.0.parse::<f64>().is_ok() {
            if halves.1.chars().next().unwrap() == ' ' {
                return Some((halves.1.trim_start(), LINETYPE::LtOL));
            }
        }
    }
    return None;
}

pub (super) fn check_code_change<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    if line.starts_with("```") {
        return Some(("CODE", LINETYPE::LtCodeChange));
    }
    return None;
}

pub (super) fn check_code_indent<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    if line.starts_with("    ") || line.starts_with("\t") {
        return Some((line.trim_start(), LINETYPE::LtCodeIndent));
    }
    return None;
}

pub (super) fn check_image<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    if line.starts_with("!") {
        if line.contains("(") && line.contains(")") && line.contains("[") && line.contains("]") {
            return Some((line, LINETYPE::LtImage));
        }
    }
    return None;
}

pub (super) fn check_paragraph<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    return Some((line, LINETYPE::LtP));
}

// TODO Table text alignment
pub (super) fn check_table_split<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    let changedline = line.trim().replace(" ", "").replace("|", "").replace("-", "").replace("–", "").replace(":", "");
    if changedline.len() == 0 {
        return Some(("TABLE SPLIT", LINETYPE::LtTableSplit));
    }
    return None;
}

pub (super) fn check_table<'a>(line: &'a str) -> Option<(&'a str, LINETYPE)> {
    let trimmedline = line.trim();
    if trimmedline.starts_with("|") && trimmedline.ends_with("|") {
        return Some((trimmedline, LINETYPE::LtTable));
    }
    return None;
}*/
