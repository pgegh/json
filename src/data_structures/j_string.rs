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

use std::fmt::{Display, Formatter};

/// A string is a sequence of Unicode code points wrapped with quotation marks (U+0022). All code
/// points may be placed within the quotation marks except for the code points that must be
/// escaped: quotation mark (U+0022), reverse solidus (U+005C), and the control characters
/// U+0000 to U+001F. There are two-character escape sequence representations of some characters.
#[derive(Debug, Clone)]
pub struct JString {
    value: String,
}

impl JString {
    /// Creates a new JSON string object.
    ///
    /// ```
    /// use json::data_structures::JString;
    ///
    /// let s = JString::new("🚗 this is a car!").unwrap();
    /// assert_eq!("🚗 this is a car!".to_string(), s.to_string());
    ///
    /// // An illegal string
    /// let mut illegal_s = "Hello world".to_string();
    /// illegal_s.push(0x0006 as char);
    /// let error = "The string contains an illegal char (0x0006)".to_string();
    /// assert_eq!(Err(error), JString::new(&illegal_s));
    /// ```
    pub fn new(str: &str) -> Result<JString, String> {
        for c in str.chars() {
            if (c as u32) < 0x0008
                || (c as u32) == 0x000B
                || ((c as u32) > 0x000D && (c as u32) < 0x0020) {
                return Err(format!("The string contains an illegal char ({:#06X})", c as u32));
            }
        }
        Ok(JString { value: str.to_string() })
    }
}

impl Display for JString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for JString {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(test)]
mod test {
    use crate::data_structures::JString;

    #[test]
    fn test_legal_string() {
        let s = JString::new("🚗 this is a car!").unwrap();
        assert_eq!("🚗 this is a car!".to_string(), s.to_string());
    }

    #[test]
    fn test_illegal_string() {
        let mut illegal_s = "Hello world".to_string();
        illegal_s.push(0x0006 as char);
        let error = "The string contains an illegal char (0x0006)".to_string();
        assert_eq!(Err(error), JString::new(&illegal_s));
    }
}