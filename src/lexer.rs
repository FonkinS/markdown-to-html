#[allow(dead_code)]
enum _TOKEN {
    TOKEN,
    BOLD,
    ITALIC,
    CODE,
}

#[allow(dead_code)]
enum LINETYPE {
    LtP,
    LtH1,
    LtH2,
    LtH3,
    LtH4,
    LtH5,
    LtH6,
    LtEmpty,
    LtNone,
    LtLine,
    LtQuote,
    LtUL,
    LtOL,
    LtCodeChange,
    LtCodeIndent,
    LtImage,
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

pub fn analyze(lines: &Vec<&str>) {
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
        implement(line_check::check_paragraph(line), &mut lc, &mut lt);
    }

    assert!(lc.len() == lines.len());
    assert!(lc.len() == lt.len());

    for l in 0..lines.len() {

    }

    println!("{} vs {}", lc.len(), lines.len());
    println!("{lc:?}");
}
