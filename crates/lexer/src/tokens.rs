use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(Identifier), // Carries the 10-char fixed-size struct
    Symbol(Symbol),
    Literal(Literal),
    EOF, // Standard "stop" signal
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    //todo: Analyse if Cow of str could be better
    String(String), // For "text inside quotes"
    Char(char),     // For 'a'
    Bool(bool),     // For true/false
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub data: [u8; 10],
    pub len: usize,
}

impl Identifier {
    // Return &'static str for the literal error message
    pub fn try_from_str(s: &str) -> Result<Self, &'static str> {
        if s.len() > 10 {
            return Err("Identifier exceeds 10 characters");
        }
        let mut data = [0u8; 10];
        data[..s.len()].copy_from_slice(s.as_bytes());
        Ok(Identifier { data, len: s.len() })
    }

    pub fn as_str(&self) -> &str {
        // Safe because we validated the input in try_from_str
        std::str::from_utf8(&self.data[..self.len]).unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    If,
    Else,
    While,
    For,
    Let,
    Fn,
    Return,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "for" => Some(Keyword::For),
            "let" => Some(Keyword::Let),
            "fn" => Some(Keyword::Fn),
            "return" => Some(Keyword::Return),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Symbol {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
}

impl Symbol {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Symbol::Plus),
            '-' => Some(Symbol::Minus),
            '*' => Some(Symbol::Multiply),
            '/' => Some(Symbol::Divide),
            '=' => Some(Symbol::Equal),
            '(' => Some(Symbol::LParen),
            ')' => Some(Symbol::RParen),
            '{' => Some(Symbol::LBrace),
            '}' => Some(Symbol::RBrace),
            ';' => Some(Symbol::Semicolon),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_keyword_test() {
        assert_eq!(Keyword::from_str("if"), Some(Keyword::If));
    }

    #[test]
    fn lookup_symbol_test() {
        assert_eq!(Symbol::from_char('/'), Some(Symbol::Divide));
    }

    #[test]
    fn identifier_exceeds_characters_test() {
        assert_eq!(
            Identifier::try_from_str("letletleltleltlet"),
            Err("Identifier exceeds 10 characters")
        );
    }

    #[test]
    fn identifier_display_test() {
        let id = Identifier::try_from_str("my_var").unwrap();
        assert_eq!(format!("{}", id), "my_var");
    }
}
