#[allow(dead_code)]

pub mod lexer {
    enum _TOKEN {
        TOKEN,
        BOLD,
        ITALIC,
        CODE,
    }

    enum LINETYPE {
        LtP,
        LtH1,
        LtH2,
        LtH3,
        LtH4,
        LtH5,
        LtH6,
    }

    pub fn analyze(lines: &Vec<&str>) {
        let mut line_types : Vec<LINETYPE> = vec!(); 
        let mut line_contents : Vec<&str> = vec!();
        for l in 0..lines.len() {
            let line = lines[l].trim_start();
            let mut chars = line.chars();

            // Header
            let mut header_depth = 0;
            while chars.next().unwrap_or(' ') == '#' {
                header_depth += 1;
            }
            if header_depth > 0 && header_depth <= 7 {
                if header_depth == 1 {line_types.push(LINETYPE::LtH1);}
                else if header_depth == 2 {line_types.push(LINETYPE::LtH2);}
                else if header_depth == 3 {line_types.push(LINETYPE::LtH3);}
                else if header_depth == 4 {line_types.push(LINETYPE::LtH4);}
                else if header_depth == 5 {line_types.push(LINETYPE::LtH5);}
                else if header_depth == 6 {line_types.push(LINETYPE::LtH6);}
                line_contents.push(chars.as_str());
            }
        }

        println!("{line_contents:?}");
    }
}
