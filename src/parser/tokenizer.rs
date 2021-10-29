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


use std::str::Chars;

fn tokenize_str(s: &str) -> Result<Vec<Token>, String> {
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
                    '"' => tokens.push(get_string(&mut chars)),
                    '0'..='9' | '-' => {
                        let mut result = match get_number(&mut chars, c) {
                            Ok(t) => t,
                            Err(s) => return Err(s)
                        };
                        tokens.append(&mut result);
                    }
                    _ => return Err(format!("Invalid char \'{}\' ({:#06x})", c, c as usize))
                }
            }
            None => break,
        }
    }
    Ok(tokens)
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

fn get_string(chars: &mut Chars) -> Token {
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
            None => break
        }
    }

    Token::String(string)
}


#[derive(Debug)]
enum Token {
    String(String),
    Number(String),
    CurlyBracketOpen,
    CurlyBracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    Colon,
    Comma,
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
            _ => false
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::tokenizer::{Token, tokenize_str};

    #[test]
    fn test_tokenize_str_curly() {
        assert_eq!(vec![Token::CurlyBracketOpen], tokenize_str("{").unwrap());
        assert_eq!(vec![Token::CurlyBracketClose], tokenize_str("}").unwrap());
        assert_eq!(vec![Token::CurlyBracketOpen, Token::CurlyBracketClose], tokenize_str("{}").unwrap());
        assert_eq!(vec![Token::CurlyBracketOpen, Token::CurlyBracketClose], tokenize_str(" \t{\n } ").unwrap());
    }

    #[test]
    fn test_tokenize_str_square() {
        assert_eq!(vec![Token::SquareBracketOpen], tokenize_str("[").unwrap());
        assert_eq!(vec![Token::SquareBracketClose], tokenize_str("]").unwrap());
        assert_eq!(vec![Token::SquareBracketOpen, Token::SquareBracketClose], tokenize_str("[]").unwrap());
        assert_eq!(vec![Token::SquareBracketOpen, Token::SquareBracketClose], tokenize_str(" \t[\n ] ").unwrap());
    }

    #[test]
    fn test_tokenize_str_colon() {
        assert_eq!(vec![Token::Colon], tokenize_str(":").unwrap());
        assert_eq!(vec![Token::Colon], tokenize_str("\n \t \t:  \n").unwrap());
    }

    #[test]
    fn test_tokenize_str_comma() {
        assert_eq!(vec![Token::Comma], tokenize_str(",").unwrap());
        assert_eq!(vec![Token::Comma], tokenize_str("\n \t \t,  \n").unwrap());
    }

    #[test]
    fn test_tokenize_str_string() {
        assert_eq!(vec![Token::String("hello world".to_string())], tokenize_str("\"hello world\"").unwrap());
        assert_eq!(vec![Token::String("hello \\\" world".to_string())], tokenize_str("\"hello \\\" world\"").unwrap());
        assert_eq!(vec![Token::String("hello world".to_string())], tokenize_str("\n \t \t\"hello world\"  \n").unwrap());
    }

    #[test]
    fn test_tokenize_str_number() {
        assert_eq!(vec![Token::Number("0.013e10".to_string())], tokenize_str("0.013e10").unwrap());
        assert_eq!(vec![Token::Number("00E.-0+13e10".to_string())], tokenize_str("00E.-0+13e10").unwrap());
        assert_eq!(vec![Token::Number("0.013".to_string()), Token::Number("0e10".to_string())], tokenize_str("\n\t0.013 0e10").unwrap());
    }

    #[test]
    fn test_tokenize_str() {
        assert_eq!(vec![
            Token::CurlyBracketOpen,
            Token::String("key".to_string()),
            Token::Colon,
            Token::SquareBracketOpen,
            Token::Number("10".to_string()),
            Token::Comma,
            Token::Number("10e20".to_string()),
            Token::SquareBracketClose,
            Token::CurlyBracketClose,
        ], tokenize_str("{\n\t\"key\" : [10,10e20]\n}").unwrap());
        assert_eq!(Err("Invalid char \'+\' (0x002b)".to_string()), tokenize_str("+10"));
        assert_eq!(Err("Invalid char \'d\' (0x0064)".to_string()), tokenize_str("1d0"));
    }
}