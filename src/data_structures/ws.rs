/// A data-structure that represents whitespace. Whitespace is any sequence of one or more of
/// the following code points: character tabulation (U+0009), line feed (U+000A), carriage return
/// (U+000D), and space (U+0020).
#[derive(Debug, Clone)]
pub struct Ws {
    value: String,
}

impl Ws {
    /// Creates a new Ws struct that contains only 0x20, 0x0A, 0x0D, and 0x09 characters.
    ///
    /// ```
    /// use json::data_structures::Ws;
    ///
    /// let mut white_space = String::new();
    /// white_space.push(0x0020 as char);
    /// white_space.push(0x000A as char);
    /// white_space.push(0x000D as char);
    /// white_space.push(0x0009 as char);
    ///
    /// let ws = Ws::new(white_space.clone()).unwrap();
    ///
    /// assert_eq!(white_space, ws.to_string());
    ///
    /// // Testing with an illegal string
    /// white_space.push(0x000B as char);
    ///
    /// assert_eq!(Err("The string contains illegal characters"), Ws::new(white_space));
    /// ```
    pub fn new(white_space: String) -> Result<Ws, &'static str> {
        if white_space.is_empty() {
            Err("The string is empty")
        } else if white_space.as_bytes().iter()
            .all(|x| *x == 0x20 || *x == 0x0A || *x == 0x0D || *x == 0x09) {
            Ok(Ws { value: white_space })
        } else {
            Err("The string contains illegal characters")
        }
    }
}

impl PartialEq for Ws {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// Implements formatting
impl std::fmt::Display for Ws {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
