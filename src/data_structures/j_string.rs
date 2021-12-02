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
use std::hash::{Hash, Hasher};
use crate::data_structures::Serialize;

/// A string is a sequence of Unicode code points wrapped with quotation marks (U+0022). All code
/// points may be placed within the quotation marks except for the code points that must be
/// escaped: quotation mark (U+0022), reverse solidus (U+005C), and the control characters
/// U+0000 to U+001F. There are two-character escape sequence representations of some characters.
#[derive(Debug, Clone, Eq)]
pub struct JString {
    value: String,
}

impl JString {
    /// Creates a new JSON string object.
    ///
    /// ```
    /// # use json::data_structures::JString;
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

impl Serialize for JString {
    /// Creates a serialization of the [`JString`] data-structure as a JSON string with
    /// minimal whitespace characters.
    /// ```
    /// # use json::data_structures::{JString, Serialize};
    /// let j_string : JString = JString::new("\"Hello world!\"\n").unwrap();
    /// assert_eq!(j_string.serialize(), "\"\\\"Hello world!\\\"\\n\"".to_string());
    /// ```
    fn serialize(&self) -> String {
        let mut s = String::new();
        s.push('\"');
        for c in self.value.chars() {
            if c == 0x000A as char {
                s.push('\\');
                s.push('n');
            } else if c == 0x0009 as char {
                s.push('\\');
                s.push('t');
            } else if c == 0x0022 as char {
                s.push('\\');
                s.push('\"');
            } else if c == 0x005C as char {
                s.push('\\');
                s.push('\\');
            } else if c == 0x002F as char {
                s.push('\\');
                s.push('/');
            } else if c == 0x0008 as char {
                s.push('\\');
                s.push('b');
            } else if c == 0x000C as char {
                s.push('\\');
                s.push('f');
            } else if c == 0x000D as char {
                s.push('\\');
                s.push('r');
            } else {
                s.push(c);
            }
        }
        s.push('\"');
        s
    }
}

impl PartialEq for JString {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Hash for JString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[cfg(test)]
mod test {
    use crate::data_structures::{JString, Serialize};

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

    #[test]
    fn test_serialization() {
        let mut n = JString::new("🚗 this is a car!").unwrap();
        assert_eq!("\"🚗 this is a car!\"".to_string(), n.serialize());

        let mut s = String::new();
        s.push_str("Hello world ");
        s.push(0x0022 as char);
        s.push_str("quotation ");
        s.push(0x005C as char);
        s.push_str("back-slash ");
        s.push(0x002F as char);
        s.push_str("slash ");
        s.push(0x0008 as char);
        s.push_str("backspace ");
        s.push(0x000C as char);
        s.push_str("form-feed ");
        s.push(0x000A as char);
        s.push_str("line-feed ");
        s.push(0x000D as char);
        s.push_str("carriage-return ");
        s.push(0x0009 as char);
        s.push_str("tab.");

        n = JString::new(&s).unwrap();
        assert_eq!("\"Hello world \\\"quotation \\\\back-slash \\/slash \\bback\
        space \\fform-feed \\nline-feed \\rcarriage-return \\ttab.\"".to_string(), n.serialize());
    }
}
