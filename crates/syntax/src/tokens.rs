use ariadne::{Color, Label, Report, ReportKind, Source};
use std::{fmt, str::FromStr};

pub fn report_error(filename: &str, source: &str, msg: &str, token: &Token) {
    let (start, end) = token.span();

    // Ensure non-zero width for EOF and single-char positions
    let range = start..end.max(start + 1);

    Report::build(ReportKind::Error, filename, start)
        .with_message("Syntax Error")
        .with_label(
            Label::new((filename, range))
                .with_message(msg)
                .with_color(Color::Red),
        )
        .finish()
        .print((filename, Source::from(source)))
        .unwrap_or_else(|e| eprintln!("Failed to print error report: {}", e));
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Key(Keyword, usize, usize),
    Identifier(Identifier<'a>, usize, usize), // Carries the 10-char fixed-size struct
    Symb(Symbol, usize, usize),
    Lit(Literal<'a>, usize, usize),
    EOF(usize), // Standard "stop" signal
}

impl<'a> Token<'a> {
    #[inline]
    pub fn span(&self) -> (usize, usize) {
        match self {
            Token::Key(_, s, e)
            | Token::Identifier(_, s, e)
            | Token::Symb(_, s, e)
            | Token::Lit(_, s, e) => (*s, *e),
            Token::EOF(s) => (*s, *s),
        }
    }
}

impl<'a> fmt::Display for Identifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    Int(i64),
    Float(f64),
    Str(&'a str), // For "text inside quotes"
    Char(char),   // For 'a'
    Bool(bool),   // For true/false
}

#[derive(Debug, PartialEq)]
pub struct Identifier<'a> {
    pub name: &'a str,
}

impl<'a> Identifier<'a> {
    pub fn try_from_str(s: &'a str) -> Result<Self, &'static str> {
        if s.len() > 10 {
            return Err("Identifier exceeds 10 characters");
        }
        Ok(Identifier { name: s })
    }
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
        assert_eq!(Keyword::from_str("if").ok(), Some(Keyword::If));
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

    #[test]
    fn ariade_report() {
        let source = "let x = 42";
        let token = Token::Key(Keyword::Let, 0, 3);
        report_error("main.gp", source, "unexpected keyword here", &token);
    }

    #[test]
    fn end_of_line_report() {
        let source = "";
        let token = Token::EOF(0);
        report_error("main.gp", source, "unexpected keyword here", &token);
    }
}
