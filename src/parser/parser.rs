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
use crate::data_structures::{JNumber, JValue};
use crate::parser::tokenizer::{tokenize_str, Token};


pub fn parse(json_string: &str) -> Result<JValue, String> {
    let tokens = tokenize_str(json_string);
    let mut tokens_itr = match tokens {
        Ok(t) => t,
        Err(s) => return Err(s)
    }.iter();

    let result = get_jvalue(&mut tokens_itr);
    match tokens_itr.next() {
        Some(t) => Err(format!("Invalid token")),
        None => result
    }
}

fn get_jvalue(tokens_itr: &mut Iter<Token>) -> Result<JValue, String> {
    let result = tokens_itr.next();
    match result {
        Some(token) => match token {
            Token::CurlyBracketOpen => get_jobject(tokens_itr),
            Token::SquareBracketOpen => get_jarray(tokens_itr),
            Token::Number(n) =>JNumber::from_str(n),
            Token::

        }
        None => Err("No Token Found".to_string())
    }
}