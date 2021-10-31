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

use std::slice::Iter;
use std::str::FromStr;
use crate::data_structures::{JNumber, JObject, JString, JValue};
use crate::parser::tokenizer::{tokenize, Token};


pub fn parse(json_string: &str) -> Result<JValue, String> {
    let tokens = tokenize(json_string)?;
    let mut tokens_itr = tokens.iter();
    get_jvalue(&mut tokens_itr)
}

fn get_jvalue(tokens_itr: &mut Iter<Token>) -> Result<JValue, String> {
    let result = tokens_itr.next();
    match result {
        Some(Token::CurlyBracketOpen) => get_jobject(tokens_itr),
        Some(Token::SquareBracketOpen) => get_jarray(tokens_itr),
        Some(Token::Number(n)) => Ok(JValue::Number(JNumber::from_str(n)?)),
        Some(Token::String(s)) => Ok(JValue::String(JString::new(s)?)),
        Some(Token::True) => Ok(JValue::Boolean(true)),
        Some(Token::False) => Ok(JValue::Boolean(false)),
        Some(Token::Null) => Ok(JValue::Null),
        Some(invalid_token) => Err(format!("Invalid token '{}'", invalid_token)),
        None => Err("No Token Found".to_string())
    }
}

fn get_jarray(tokens_itr: &mut Iter<Token>) -> Result<JValue, String> {
    let mut vec: Vec<JValue> = Vec::new();
    loop {
        match tokens_itr.next() {
            Some(Token::CurlyBracketOpen) => vec.push(get_jobject(tokens_itr)?),
            Some(Token::SquareBracketOpen) => vec.push(get_jarray(tokens_itr)?),
            Some(Token::String(s)) => vec.push(JValue::String(JString::new(s)?)),
            Some(Token::Number(n)) => vec.push(JValue::Number(JNumber::from_str(n)?)),
            Some(Token::True) => vec.push(JValue::Boolean(true)),
            Some(Token::False) => vec.push(JValue::Boolean(false)),
            Some(Token::Null) => vec.push(JValue::Null),
            Some(Token::SquareBracketClose) => return Ok(JValue::Array(vec)),
            Some(t) => return Err(format!("Invalid JSON array! Invalid token: {}", t)),
            None => return Err(format!("Invalid JSON array! Missing a closing square bracket \"]\""))
        }
        match tokens_itr.next() {
            Some(Token::Comma) => continue,
            Some(Token::SquareBracketClose) => return Ok(JValue::Array(vec)),
            Some(t) => return Err(format!("Invalid JSON array! Invalid token: {}", t)),
            None => return Err(format!("Invalid JSON array! Missing a closing square bracket \"]\""))
        }
    }
}

fn get_jobject(tokens_itr: &mut Iter<Token>) -> Result<JValue, String> {
    let mut obj = JObject::new();
    loop {
        let key = match tokens_itr.next() {
            Some(Token::String(s)) => JString::new(s)?,
            Some(Token::CurlyBracketClose) => return Ok(JValue::Object(obj)),
            Some(t) => return Err(format!("Invalid JSON object! Invalid token:  {}", t)),
            None => return
                Err(format!("Invalid JSON object! Missing a closing curly bracket \"}}\""))
        };
        match tokens_itr.next() {
            Some(Token::Colon) => (),
            Some(t) => return
                Err(format!("Invalid JSON object! Invalid token: {} instead of \":\"", t)),
            None => return Err(format!("Invalid JSON object! Missing a colon \":\""))
        };
        match obj.insert(key.clone(), get_jvalue(tokens_itr)?) {
            Some(_) => return
                Err(format!("Invalid JSON object: the key {} is not unique", key)),
            None => ()
        }
    }
}