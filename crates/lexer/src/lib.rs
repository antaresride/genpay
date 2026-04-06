use std::str::{CharIndices, FromStr};
use syntax::{Keyword, Literal, Symbol, Token, report_error};

pub fn file_reader(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

pub fn next_token<'a>(source: &'a str, iter: &mut CharIndices<'a>) -> Option<Token<'a>> {
    let mut peek = iter.clone().next();

    loop {
        // Skip whitespace
        while let Some((_, ch)) = peek {
            if ch.is_whitespace() {
                report_error(
                    "main.gp",
                    source,
                    "unexpected end of file",
                    &Token::Keyword(Keyword::Let, "let", 0, 3),
                );
                peek = iter.next();
            } else {
                break;
            }
        }

        let (start, ch) = peek?;
        peek = iter.next();

        // Check single-char symbols first
        if let Some(sym) = Symbol::from_char(ch) {
            let end = peek.map(|(p, _)| p).unwrap_or(source.len());
            return Some(Token::Symbol(sym, start, end));
        }

        match ch {
            // String literal: "content"
            '"' => {
                let slice_start = start;

                while let Some((idx, c)) = peek {
                    peek = iter.next();
                    if c == '"' {
                        let end = peek.map(|(p, _)| p).unwrap_or(source.len());
                        let inner_start = (slice_start + 1).min(idx);
                        let slice = &source[inner_start..idx];
                        return Some(Token::Literal(Literal::Str(slice), start, end));
                    }
                }
                return None;
            }

            // Identifier or Keyword
            ch if ch.is_alphabetic() || ch == '_' => {
                let slice_start = start;

                while let Some((_, c)) = peek {
                    if c.is_alphabetic() || c == '_' || c.is_numeric() {
                        peek = iter.next();
                    } else {
                        break;
                    }
                }

                let end = peek.map(|(p, _)| p).unwrap_or(source.len());
                let slice = &source[slice_start..end];

                match slice {
                    "true" => return Some(Token::Literal(Literal::Bool(true), start, end)),
                    "false" => return Some(Token::Literal(Literal::Bool(false), start, end)),
                    _ => {}
                }

                if let Ok(kw) = Keyword::from_str(slice) {
                    return Some(Token::Keyword(kw, slice, start, end));
                }

                return Some(Token::Identifier(slice, start, end));
            }

            // Number: integer or float
            ch if ch.is_numeric() => {
                let slice_start = start;
                let mut is_float = false;

                while let Some((_, c)) = peek {
                    if c.is_numeric() {
                        peek = iter.next();
                    } else if c == '.' && !is_float {
                        let next_is_digit = iter
                            .clone()
                            .next()
                            .is_some_and(|(_, next_c)| next_c.is_numeric());

                        if !next_is_digit {
                            break;
                        }

                        is_float = true;
                        peek = iter.next();
                    } else {
                        break;
                    }
                }

                let end = peek.map(|(p, _)| p).unwrap_or(source.len());
                let slice = &source[slice_start..end];

                let literal = if is_float {
                    Literal::Float(slice.parse().unwrap_or(0.0))
                } else {
                    Literal::Int(slice.parse().unwrap_or(0))
                };

                return Some(Token::Literal(literal, start, end));
            }

            _ => continue,
        }
    }
}
