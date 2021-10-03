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
    /// let s = JString::new("Hello world!").unwrap();
    /// assert_eq!("Hello world!".to_string(), s.to_string());
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
