
enum _LINETYPE<T> {
    LtP(T),
    LtH(u8, T),
}

pub mod lexer {
    pub fn analyze(lines: &Vec<&str>) {
        let mut lines = lines.clone();
        for l in 0..lines.len() {
            lines[l] = lines[l].trim();
        }

        println!("{lines:?}");
    }
}
