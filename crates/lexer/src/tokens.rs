pub enum Token {
    Keyword(Keyword),
    Identifier,
    Symbol(Symbol),
}

impl Identifier {
    pub fn try_from_str(s: &str) -> Result<Self, &str> {
        if s.len() > 10 {
            return Err("Identifier exceeds 10 characters");
        }
        let mut data = [0u8; 10];
        data[..s.len()].copy_from_slice(s.as_bytes());
        Ok(Identifier { data, len: s.len() })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Identifier {
    pub data: [u8; 10],
    pub len: usize,
}
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
            (Err("Identifier exceeds 10 characters"))
        );
    }
}
