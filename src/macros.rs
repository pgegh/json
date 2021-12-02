/// Construct a json::data_structure::JValue from a &str.
///
/// Improvement: use tokens instead of &str.
///
/// ```
/// # use json::data_structures::{JObject, JString, JValue};
/// # use json::json;
/// let mut j_object = JObject::new();
/// let key = JString::new("key").unwrap();
/// let value = JValue::String(JString::new("value").unwrap());
/// j_object.insert(key, value);
///
/// let j0 = JValue::Object(j_object);
///
/// let j1 = json!("{\"key\":\"value\"}");
///
/// assert_eq!(j0, j1);
/// ```
#[macro_export]
macro_rules! json {
    // Hide distracting implementation details from the generated rustdoc.
    ($json:tt) => {
        {
            let result = $crate::parser::parse($json);
            match result {
                Ok(j_value) => j_value,
                Err(e) => panic!("{}", e)
            }
        }
    };
}