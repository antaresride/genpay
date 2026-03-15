use std::{fs, io};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Lexer<'a> {
    // The iterator points directly to the heap memory owned by your 'content' String
    chars: Peekable<Chars<'a>>,
    pos: usize,
}
impl <'a>Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            // The iterator points directly to the heap memory owned by your 'content' String
            chars: source.chars().peekable(),
            pos: 0,
        }
    }
    // Reads a file from disk and returns the string or an error
    pub fn file_reader(path: &'a str) ->io::Result<String> {
        fs::read_to_string(path)
    }

    pub fn read_from_slices(file: &'a str)->&'a str{
        file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_creation_test() {
        let file = "../../a.pay";
        let source = "let x = 10f;";
        let reader = Lexer::file_reader(file);
        let content = reader.expect("Failed to read file - check if the path is correct");
        println!("File read successfully! Content: {:?}", content);
        Lexer::read_from_slices(source);
    }
}
