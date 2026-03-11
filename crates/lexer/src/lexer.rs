#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}
impl Lexer {
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            pos: 0,
        }
    }
    // it will return  Result
    pub fn file_reader(file: &'static str) -> &'static str {
        file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_creation_test() {
        let source = "let x = 10f;";
        let _ = Lexer::file_reader(source);
    }
}
