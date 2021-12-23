/// Serialize the implementing data-structure.
pub trait Serialize {
    /// Creates a serialization of the implementing data-structure as a JSON string with
    /// minimal whitespace characters.
    fn serialize(&self) -> String;
}

/// Creates a serialization of a [`String`] object as a JSON string with minimal
/// whitespace characters.
pub fn serialize_string(string: &String) -> String {
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
