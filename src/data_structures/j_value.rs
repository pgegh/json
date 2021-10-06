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
use crate::data_structures::{JString, Number};

/// A value can be a string, or a number, or true or false or null, or an
/// object or an array.
///
/// ```
/// use json::data_structures::JValue;
///
/// let b: JValue = JValue::Boolean(true);
/// assert_eq!("true".to_string(), b.to_string());
///
/// let n: JValue = JValue::Null;
/// assert_eq!("null".to_string(), n.to_string());
///
/// assert_ne!(b, n);
/// ```
#[derive(Debug, Clone)]
pub enum JValue {
    JString(JString),
    Number(Number),
    Boolean(bool),
    Null,
}

impl Display for JValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JValue::JString(s) => write!(f, "{}", s),
            JValue::Number(n) => write!(f, "{}", n),
            JValue::Boolean(b) => write!(f, "{}", b),
            JValue::Null => write!(f, "null")
        }
    }
}

impl PartialEq for JValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JValue::JString(s1), JValue::JString(s2)) => s1 == s2,
            (JValue::Number(n1), JValue::Number(n2)) => n1 == n2,
            (JValue::Boolean(b1), JValue::Boolean(b2)) => b1 == b2,
            (JValue::Null, JValue::Null) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::data_structures::{JString, JValue, Number};

    #[test]
    fn test_valid_j_string() {
        let s1: JValue = JValue::JString(JString::new("Hello World!").unwrap());
        assert_eq!("Hello World!".to_string(), s1.to_string());
        let s2: JValue = JValue::JString(JString::new("Hello World!").unwrap());
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_valid_number() {
        let n1 = JValue::Number(Number::from_str("90.010").unwrap());
        assert_eq!("90.010".to_string(), n1.to_string());
        let n2 = JValue::Number(Number::from_str("90.010").unwrap());
        assert_eq!(n1, n2);
    }

    #[test]
    fn test_valid_boolean() {
        let b1 = JValue::Boolean(true);
        assert_eq!("true".to_string(), b1.to_string());
        let b2 = JValue::Boolean(false);
        assert_eq!("false".to_string(), b2.to_string());
        assert_ne!(b1, b2);
        let b3 = JValue::Boolean(false);
        assert_eq!(b2, b3);
    }

    #[test]
    fn test_valid_null() {
        let x1 = JValue::Null;
        assert_eq!("null".to_string(), x1.to_string());
        let x2 = JValue::Null;
        assert_eq!("null".to_string(), x2.to_string());
        assert_eq!(x1, x2);
    }
}
