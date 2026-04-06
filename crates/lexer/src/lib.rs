use std::str::FromStr;
use syntax::*;

pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        while let Some(&(start, ch)) = self.chars.peek() {
            match ch {
                c if c.is_whitespace() => {
                    self.chars.next();
                    continue;
                }
                c if Symbol::from_char(c).is_some() => {
                    let sym = Symbol::from_char(c).unwrap();
                    self.chars.next();
                    return Some(Token::Symbol(sym, start, self.peek_pos()));
                }
                '"' => {
                    self.chars.next();
                    let content_start = start + 1;
                    while let Some(&(idx, c)) = self.chars.peek() {
                        if c == '"' {
                            let slice = &self.source[content_start..idx];
                            self.chars.next();
                            return Some(Token::Literal(
                                Literal::Str(slice),
                                start,
                                self.peek_pos(),
                            ));
                        }
                        self.chars.next();
                    }
                    return None;
                }
                c if c.is_alphabetic() || c == '_' => {
                    while let Some(&(_, next_c)) = self.chars.peek() {
                        if next_c.is_alphanumeric() || next_c == '_' {
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    let end = self.peek_pos();
                    let slice = &self.source[start..end];
                    return Some(match slice {
                        "true" => Token::Literal(Literal::Bool(true), start, end),
                        "false" => Token::Literal(Literal::Bool(false), start, end),
                        _ => {
                            if let Ok(kw) = Keyword::from_str(slice) {
                                Token::Keyword(kw, slice, start, end)
                            } else {
                                Token::Identifier(slice, start, end)
                            }
                        }
                    });
                }
                c if c.is_numeric() => {
                    let mut is_float = false;
                    while let Some(&(idx, next_c)) = self.chars.peek() {
                        if next_c.is_numeric() {
                            self.chars.next();
                        } else if next_c == '.' && !is_float {
                            if let Some(b) = self.source.as_bytes().get(idx + 1)
                                && b.is_ascii_digit()
                            {
                                is_float = true;
                                self.chars.next();
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    let end = self.peek_pos();
                    let slice = &self.source[start..end];
                    let lit = if is_float {
                        Literal::Float(slice.parse().unwrap_or(0.0))
                    } else {
                        Literal::Int(slice.parse().unwrap_or(0))
                    };
                    return Some(Token::Literal(lit, start, end));
                }
                _ => {
                    self.chars.next();
                    continue;
                }
            }
        }
        None
    }

    #[inline]
    fn peek_pos(&mut self) -> usize {
        self.chars
            .peek()
            .map(|(i, _)| *i)
            .unwrap_or(self.source.len())
    }
}

// This MUST be present for 'for token in lexer' to work in main.rs
impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
