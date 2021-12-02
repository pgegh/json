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

use crate::data_structures::Serialize;

/// A data-structure that represents whitespace. Whitespace is any sequence of one or more of
/// the following code points: character tabulation (U+0009), line feed (U+000A), carriage return
/// (U+000D), and space (U+0020).
#[derive(Debug, Clone)]
pub struct JWhitespace {
    value: String,
}

impl JWhitespace {
    /// Creates a new Ws struct that contains only 0x20, 0x0A, 0x0D, and 0x09 characters.
    ///
    /// ```
    /// # use json::data_structures::JWhitespace;
    /// let mut whitespace = String::new();
    /// whitespace.push(0x0020 as char);
    /// whitespace.push(0x000A as char);
    /// whitespace.push(0x000D as char);
    /// whitespace.push(0x0009 as char);
    ///
    /// let ws = JWhitespace::new(&whitespace).unwrap();
    ///
    /// assert_eq!(whitespace, ws.to_string());
    ///
    /// // Testing with an illegal string
    /// whitespace.push(0x000B as char);
    ///
    /// assert_eq!(Err("The string contains illegal characters"), JWhitespace::new(&whitespace));
    /// ```
    pub fn new(whitespace: &str) -> Result<JWhitespace, &'static str> {
        if whitespace.is_empty() {
            Err("The string is empty")
        } else if whitespace.chars().all(|x| x == 0x0020 as char
            || x == 0x000A as char
            || x == 0x000D as char
            || x == 0x0009 as char) {
            Ok(JWhitespace { value: whitespace.to_string() })
        } else {
            Err("The string contains illegal characters")
        }
    }
}

impl PartialEq for JWhitespace {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// Implements formatting
impl std::fmt::Display for JWhitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Serialize for JWhitespace {
    fn serialize(&self) -> String {
        " ".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::{JWhitespace, Serialize};

    #[test]
    fn test_empty_string() {
        assert_eq!(Err("The string is empty"), JWhitespace::new(""));
    }

    #[test]
    fn test_illegal_string() {
        let mut whitespace = String::new();
        whitespace.push(0x20 as char);
        whitespace.push(0x0A as char);
        whitespace.push(0x0B as char);
        assert_eq!(Err("The string contains illegal characters"), JWhitespace::new(&whitespace));
    }

    #[test]
    fn test_legal_string() {
        let mut whitespace = String::new();
        whitespace.push(0x20 as char);
        whitespace.push(0x0A as char);
        whitespace.push(0x0D as char);
        whitespace.push(0x09 as char);
        let ws = JWhitespace::new(&whitespace).unwrap();
        assert_eq!(whitespace, ws.to_string());
    }

    #[test]
    fn test_serialization(){
        let mut ws = JWhitespace::new(" ").unwrap();
        assert_eq!(" ".to_string(), ws.serialize());

        ws = JWhitespace::new("  \n  \t ").unwrap();
        assert_eq!(" ".to_string(), ws.serialize());
    }
}
