use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Keyword(Keyword, &'a str, usize, usize),
    Identifier(&'a str, usize, usize),
    Symbol(Symbol, usize, usize),
    Literal(Literal<'a>, usize, usize),
    EOF(usize),
}

impl<'a> Token<'a> {
    #[inline]
    pub fn span(&self) -> (usize, usize) {
        match self {
            Token::Keyword(_, _, s, e)
            | Token::Identifier(_, s, e)
            | Token::Symbol(_, s, e)
            | Token::Literal(_, s, e) => (*s, *e),
            Token::EOF(s) => (*s, *s),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    Int(i64),
    Float(f64),
    Str(&'a str),
    Char(char),
    Bool(bool),
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
    True,
    False,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "while" => Ok(Keyword::While),
            "for" => Ok(Keyword::For),
            "let" => Ok(Keyword::Let),
            "fn" => Ok(Keyword::Fn),
            "return" => Ok(Keyword::Return),
            _ => Err(()),
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
