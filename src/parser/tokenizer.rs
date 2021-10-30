// Copyright © 2021 Hovig Manjikian
//
// This file is part of json.
//
// json is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// json is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with json.  If not, see <https://www.gnu.org/licenses/>.

//! A lexical analyzer for JSON


use std::fmt::{Display, Formatter};
use std::str::Chars;

pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut chars = s.chars();
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        match chars.next() {
            Some(c) => {
                match c {
                    '{' => tokens.push(Token::CurlyBracketOpen),
                    '}' => tokens.push(Token::CurlyBracketClose),
                    '[' => tokens.push(Token::SquareBracketOpen),
                    ']' => tokens.push(Token::SquareBracketClose),
                    ':' => tokens.push(Token::Colon),
                    ',' => tokens.push(Token::Comma),
                    whitespace if whitespace == 0x0020 as char
                        || whitespace == 0x000A as char
                        || whitespace == 0x000D as char
                        || whitespace == 0x0009 as char => (),
                    '"' => tokens.push(get_string(&mut chars)?),
                    '0'..='9' | '-' => tokens.append(&mut get_number(&mut chars, c)?),
                    't' => tokens.push(get_true(&mut chars)?),
                    'f' => tokens.push(get_false(&mut chars)?),
                    'n' => tokens.push(get_null(&mut chars)?),
                    _ => return Err(format!("Invalid char \'{}\' ({:#06x})", c, c as usize))
                }
            }
            None => break,
        }
    }
    Ok(tokens)
}

fn get_true(chars: &mut Chars) -> Result<Token, String> {
    let err = "Invalid token ";
    match chars.next() {
        Some('r') => match chars.next() {
            Some('u') => match chars.next() {
                Some('e') => Ok(Token::True),
                Some(x) => return Err(format!("{}\"tru{}\"", err, x)),
                None => return Err(format!("{}\"tru\"", err)),
            },
            Some(x) => return Err(format!("{}\"tr{}\"", err, x)),
            None => return Err(format!("{}\"tr\"", err)),
        },
        Some(x) => return Err(format!("{}\"t{}\"", err, x)),
        None => return Err(format!("{}\"t\"", err)),
    }
}

fn get_false(chars: &mut Chars) -> Result<Token, String> {
    let err = "Invalid token ";
    match chars.next() {
        Some('a') => match chars.next() {
            Some('l') => match chars.next() {
                Some('s') => match chars.next() {
                    Some('e') => Ok(Token::False),
                    Some(x) => return Err(format!("{}\"fals{}\"", err, x)),
                    None => return Err(format!("{}\"fals\"", err)),
                },
                Some(x) => return Err(format!("{}\"fal{}\"", err, x)),
                None => return Err(format!("{}\"fal\"", err)),
            },
            Some(x) => return Err(format!("{}\"fa{}\"", err, x)),
            None => return Err(format!("{}\"fa\"", err)),
        },
        Some(x) => return Err(format!("{}\"f{}\"", err, x)),
        None => return Err(format!("{}\"f\"", err)),
    }
}

fn get_null(chars: &mut Chars) -> Result<Token, String> {
    let err = "Invalid token ";
    match chars.next() {
        Some('u') => match chars.next() {
            Some('l') => match chars.next() {
                Some('l') => Ok(Token::Null),
                Some(x) => return Err(format!("{}\"nul{}\"", err, x)),
                None => return Err(format!("{}\"nul\"", err)),
            },
            Some(x) => return Err(format!("{}\"nu{}\"", err, x)),
            None => return Err(format!("{}\"nu\"", err)),
        },
        Some(x) => return Err(format!("{}\"n{}\"", err, x)),
        None => return Err(format!("{}\"n\"", err)),
    }
}

fn get_number(chars: &mut Chars, first_char: char) -> Result<Vec<Token>, String> {
    let mut string = String::new();
    string.push(first_char);
    loop {
        match chars.next() {
            Some(c) => {
                match c {
                    '0'..='9' | '.' | '-' | '+' | 'e' | 'E' => string.push(c),
                    whitespace if whitespace == 0x0020 as char
                        || whitespace == 0x000A as char
                        || whitespace == 0x000D as char
                        || whitespace == 0x0009 as char => break,
                    ',' => return Ok(vec![Token::Number(string), Token::Comma]),
                    ']' => return Ok(vec![Token::Number(string), Token::SquareBracketClose]),
                    '}' => return Ok(vec![Token::Number(string), Token::CurlyBracketClose]),
                    _ => return Err(format!("Invalid char \'{}\' ({:#06x})", c, c as usize))
                }
            }
            None => break
        }
    }
    Ok(vec![Token::Number(string)])
}

fn get_string(chars: &mut Chars) -> Result<Token, String> {
    let mut string = String::new();
    let mut last_char = '"';
    loop {
        match chars.next() {
            Some(c) => {
                match c {
                    '"' => {
                        if last_char == '\\' {
                            last_char = c;
                            string.push(c);
                        } else {
                            break;
                        }
                    }
                    _ => {
                        last_char = c;
                        string.push(c);
                    }
                }
            }
            None => return Err(format!("Invalid string token at the end of file!"))
        }
    }
    Ok(Token::String(string))
}


#[derive(Debug)]
pub enum Token {
    String(String),
    Number(String),
    CurlyBracketOpen,
    CurlyBracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    Colon,
    Comma,
    Null,
    True,
    False,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::String(s1), Token::String(s2)) => s1 == s2,
            (Token::Number(n1), Token::Number(n2)) => n1 == n2,
            (Token::CurlyBracketOpen, Token::CurlyBracketOpen) => true,
            (Token::CurlyBracketClose, Token::CurlyBracketClose) => true,
            (Token::SquareBracketOpen, Token::SquareBracketOpen) => true,
            (Token::SquareBracketClose, Token::SquareBracketClose) => true,
            (Token::Colon, Token::Colon) => true,
            (Token::Comma, Token::Comma) => true,
            (Token::True, Token::True) => true,
            (Token::False, Token::False) => true,
            (Token::Null, Token::Null) => true,
            _ => false
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::String(s) => write!(f, "{}", s),
            Token::Number(n) => write!(f, "{}", n),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Null => write!(f, "null"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::SquareBracketOpen => write!(f, "["),
            Token::SquareBracketClose => write!(f, "]"),
            Token::CurlyBracketOpen => write!(f, "{{"),
            Token::CurlyBracketClose => write!(f, "}}"),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::tokenizer::{Token, tokenize};

    #[test]
    fn test_tokenize_curly() {
        assert_eq!(vec![Token::CurlyBracketOpen], tokenize("{").unwrap());
        assert_eq!(vec![Token::CurlyBracketClose], tokenize("}").unwrap());
        assert_eq!(vec![Token::CurlyBracketOpen, Token::CurlyBracketClose], tokenize("{}").unwrap());
        assert_eq!(vec![Token::CurlyBracketOpen, Token::CurlyBracketClose], tokenize(" \t{\n } ").unwrap());
    }

    #[test]
    fn test_tokenize_square() {
        assert_eq!(vec![Token::SquareBracketOpen], tokenize("[").unwrap());
        assert_eq!(vec![Token::SquareBracketClose], tokenize("]").unwrap());
        assert_eq!(vec![Token::SquareBracketOpen, Token::SquareBracketClose], tokenize("[]").unwrap());
        assert_eq!(vec![Token::SquareBracketOpen, Token::SquareBracketClose], tokenize(" \t[\n ] ").unwrap());
    }

    #[test]
    fn test_tokenize_colon() {
        assert_eq!(vec![Token::Colon], tokenize(":").unwrap());
        assert_eq!(vec![Token::Colon], tokenize("\n \t \t:  \n").unwrap());
    }

    #[test]
    fn test_tokenize_comma() {
        assert_eq!(vec![Token::Comma], tokenize(",").unwrap());
        assert_eq!(vec![Token::Comma], tokenize("\n \t \t,  \n").unwrap());
    }

    #[test]
    fn test_tokenize_string() {
        assert_eq!(vec![Token::String("hello world".to_string())], tokenize("\"hello world\"").unwrap());
        assert_eq!(vec![Token::String("hello \\\" world".to_string())], tokenize("\"hello \\\" world\"").unwrap());
        assert_eq!(vec![Token::String("hello world".to_string())], tokenize("\n \t \t\"hello world\"  \n").unwrap());
        assert_eq!(Err("Invalid string token at the end of file!".to_string()), tokenize("\"hello world"));
    }

    #[test]
    fn test_tokenize_number() {
        assert_eq!(vec![Token::Number("0.013e10".to_string())], tokenize("0.013e10").unwrap());
        assert_eq!(vec![Token::Number("00E.-0+13e10".to_string())], tokenize("00E.-0+13e10").unwrap());
        assert_eq!(vec![Token::Number("0.013".to_string()), Token::Number("0e10".to_string())], tokenize("\n\t0.013 0e10").unwrap());
    }

    #[test]
    fn test_tokenize_true() {
        assert_eq!(vec![Token::True], tokenize("true").unwrap());
        assert_eq!(vec![Token::True], tokenize("\n \t \ttrue  \n").unwrap());
    }

    #[test]
    fn test_tokenize_false() {
        assert_eq!(vec![Token::False], tokenize("false").unwrap());
        assert_eq!(vec![Token::False], tokenize("\n \t \tfalse  \n").unwrap());
    }

    #[test]
    fn test_tokenize_null() {
        assert_eq!(vec![Token::Null], tokenize("null").unwrap());
        assert_eq!(vec![Token::Null], tokenize("\n \t \tnull  \n").unwrap());
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(vec![
            Token::CurlyBracketOpen,
            Token::String("key".to_string()),
            Token::Colon,
            Token::SquareBracketOpen,
            Token::True,
            Token::Comma,
            Token::Number("10".to_string()),
            Token::Comma,
            Token::Number("10e20".to_string()),
            Token::SquareBracketClose,
            Token::CurlyBracketClose,
        ], tokenize("{\n\t\"key\" : [true, 10,10e20]\n}").unwrap());
        assert_eq!(Err("Invalid char \'+\' (0x002b)".to_string()), tokenize("+10"));
        assert_eq!(Err("Invalid char \'d\' (0x0064)".to_string()), tokenize("1d0"));
        assert_eq!(Err("Invalid token \"tru \"".to_string()), tokenize("tru "));
        assert_eq!(Err("Invalid token \"nul\"".to_string()), tokenize("nul"));
    }
}