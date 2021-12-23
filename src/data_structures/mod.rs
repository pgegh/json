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

//! Data-structures for different JSON components

mod j_whitespace;
mod j_number;
mod j_value;
mod j_object;

pub use j_whitespace::JWhitespace;
pub use j_number::JNumber;
pub use j_value::JValue;
pub use j_object::JObject;


/// Serialize the implementing data-structure.
pub trait Serialize {
    /// Creates a serialization of the implementing data-structure as a JSON string with
    /// minimal whitespace characters.
    fn serialize(&self) -> String;
}

/// Creates a serialization of a [`String`] object as a JSON string with minimal
/// whitespace characters.
fn serialize_string(string: &String) -> String {
    let mut s = String::new();
    s.push('\"');
    for c in string.chars() {
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
