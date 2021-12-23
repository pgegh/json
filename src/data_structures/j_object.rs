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

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::data_structures::{JValue, Serialize, serialize_string};

/// An object is an unordered set of name/value pairs.
/// An object begins with '{' left brace and ends with '}' right brace.
/// Each name is followed by ':' colon and the name/value pairs are separated by ',' comma.
#[derive(Debug, Clone)]
pub struct JObject {
    value: HashMap<String, JValue>,
    size: usize,
}

impl JObject {
    /// Creates new empty JObject
    ///
    /// ```
    /// # use json::data_structures::JObject;
    /// let object = JObject::new();
    /// assert_eq!("{}".to_string(), object.to_string());
    /// ```
    pub fn new() -> JObject {
        JObject {
            value: HashMap::new(),
            size: 0,
        }
    }

    /// Returns the number of elements in the JObject
    ///
    /// ```
    /// # use json::data_structures::{JObject, JString, JValue};
    /// let mut obj = JObject::new();
    /// obj.insert(JString::new("key").unwrap(), JValue::Boolean(true));
    /// assert_eq!("{key : true,}".to_string(), obj.to_string());
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    /// Inserts a key-value pair into the object.
    ///
    /// If the object did not have this key present, [`None`] is returned.
    ///
    /// If the object did have this key present, the value is updated, and the old
    /// value is returned.
    ///
    /// ```
    /// # use json::data_structures::{JObject, JString, JValue};
    /// let mut obj = JObject::new();
    /// assert_eq!(0, obj.len());
    /// obj.insert(JString::new("key").unwrap(), JValue::Boolean(true));
    /// assert_eq!("{key : true,}".to_string(), obj.to_string());
    /// assert_eq!(1, obj.len());
    /// ```
    pub fn insert(&mut self, k: String, v: JValue) -> Option<JValue> {
        match self.value.insert(k, v) {
            Some(old_v) => Some(old_v),
            None => {
                self.size += 1;
                None
            }
        }
    }

    /// Removes a key from the object, returning the value at the key if the key
    /// was previously in the object. Otherwise will return [`None`].
    ///
    /// ```
    /// # use json::data_structures::{JObject, JString, JValue};
    /// let mut obj = JObject::new();
    /// let s = JString::new("key").unwrap();
    /// obj.insert(s.clone(), JValue::Boolean(false));
    /// assert_eq!(1, obj.len());
    ///
    /// obj.remove(&s);
    /// assert_eq!(0, obj.len());
    /// assert_eq!(None, obj.remove(&s));
    /// ```
    pub fn remove(&mut self, k: &String) -> Option<JValue> {
        match self.value.remove(k) {
            Some(v) => {
                self.size -= 1;
                Some(v)
            }
            None => None,
        }
    }

    /// Gets a reference to the value if the key exists in the object. Otherwise returns [`None`].
    ///
    /// ```
    /// # use json::data_structures::{JObject, JString, JValue};
    /// let mut obj = JObject::new();
    /// let s = JString::new("key").unwrap();
    /// obj.insert(s.clone(), JValue::Boolean(false));
    /// assert_eq!("false".to_string(), obj.get(&s).unwrap().to_string());
    /// ```
    pub fn get(&self, k: &String) -> Option<&JValue> {
        self.value.get(k)
    }

    /// Gets a mutable reference to the value if the key exists in the object.
    /// Otherwise returns [`None`].
    ///
    /// ```
    /// # use json::data_structures::{JObject, JString, JValue};
    /// let mut obj = JObject::new();
    /// let s = JString::new("key").unwrap();
    /// obj.insert(s.clone(), JValue::Boolean(false));
    /// *obj.get_mut(&s).unwrap() = JValue::Boolean(true);
    /// assert_eq!("true".to_string(), obj.get(&s).unwrap().to_string());
    /// ```
    pub fn get_mut(&mut self, k: &String) -> Option<&mut JValue> {
        self.value.get_mut(k)
    }
}


impl Display for JObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for (s, v) in &self.value {
            result.push_str(&s.to_string());
            result.push_str(" : ");
            result.push_str(&format!("{}", v));
            result.push_str(",");
        }
        write!(f, "{{{}}}", result)
    }
}

impl PartialEq for JObject {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Serialize for JObject {
    /// Creates a serialization of [`JObject`] data-structure as a JSON string with
    /// minimal whitespace characters.
    /// ```
    /// # use json::data_structures::{JObject, JString, JValue, Serialize};
    /// let mut j_object : JObject = JObject::new();
    /// let key : JString = JString::new("key").unwrap();
    /// j_object.insert(key, JValue::Null);
    ///
    /// assert_eq!(j_object.serialize(), "{\"key\":null}")
    /// ```
    fn serialize(&self) -> String {
        let mut result = String::new();
        result.push('{');
        for (i, (s, v)) in self.value.iter().enumerate() {
            result.push_str(&serialize_string(s));
            result.push_str(":");
            result.push_str(&v.serialize());
            if i < self.size - 1 {
                result.push_str(",");
            }
        }
        result.push('}');
        result
    }
}


#[cfg(test)]
mod test {
    use crate::data_structures::{JObject, JValue, Serialize};

    #[test]
    fn test_empty_object() {
        let obj = JObject::new();
        assert_eq!(0, obj.len());
    }

    #[test]
    fn test_inserting() {
        let mut obj = JObject::new();
        let k1 = "key1".to_string();
        let k2 = "key2".to_string();
        assert_eq!(0, obj.len());
        obj.insert(k1.clone(), JValue::Boolean(true));
        assert_eq!("{key1 : true,}".to_string(), obj.to_string());
        assert_eq!(1, obj.len());
        obj.insert(k2.clone(), JValue::Null);
        assert_eq!(2, obj.len());
    }

    #[test]
    fn test_remove() {
        let mut obj = JObject::new();
        let k1 = "key1".to_string();
        let k2 = "key2".to_string();
        assert_eq!(0, obj.len());
        assert_eq!(None, obj.remove(&k1));
        obj.insert(k1.clone(), JValue::Boolean(true));
        obj.insert(k2.clone(), JValue::Null);
        assert_eq!(2, obj.len());
        assert_eq!(JValue::Boolean(true), obj.remove(&k1).unwrap());
        assert_eq!(JValue::Null, obj.remove(&k2).unwrap());
        assert_eq!(0, obj.len());
    }

    #[test]
    fn test_get() {
        let mut obj = JObject::new();
        let k1 = "key1".to_string();
        let k2 = "key2".to_string();
        obj.insert(k1.clone(), JValue::Boolean(true));
        assert_eq!(JValue::Boolean(true), *obj.get(&k1).unwrap());
        assert_eq!(None, obj.get(&k2));
    }

    #[test]
    fn test_get_mut() {
        let mut obj = JObject::new();
        let k1 = "key1".to_string();
        let k2 = "key2".to_string();
        obj.insert(k1.clone(), JValue::Boolean(true));
        assert_eq!(JValue::Boolean(true), *obj.get(&k1).unwrap());
        assert_eq!(None, obj.get_mut(&k2));
        *obj.get_mut(&k1).unwrap() = JValue::Null;
        assert_eq!(JValue::Null, *obj.get(&k1).unwrap());
    }

    #[test]
    fn test_eq() {
        let mut obj1 = JObject::new();
        let mut obj2 = JObject::new();
        let mut obj3 = JObject::new();
        let mut obj4 = JObject::new();
        let k1 = "key1".to_string();
        let k2 = "key2".to_string();
        assert_eq!(obj1, obj2);
        obj1.insert(k1.clone(), JValue::Boolean(true));
        obj1.insert(k2.clone(), JValue::Null);
        assert_ne!(obj1, obj2);
        obj2.insert(k1.clone(), JValue::Boolean(true));
        obj2.insert(k2.clone(), JValue::Null);
        assert_eq!(obj1, obj2);
        obj3.insert(k2.clone(), JValue::Null);
        obj3.insert(k1.clone(), JValue::Boolean(true));
        assert_eq!(obj1, obj3);
        obj4.insert(k1.clone(), JValue::Boolean(false));
        obj4.insert(k2.clone(), JValue::Null);
        assert_ne!(obj1, obj4);
    }

    #[test]
    fn test_serialization() {
        let mut obj = JObject::new();
        let k1 = "key1".to_string();
        let k2 = "key2".to_string();
        assert_eq!("{}".to_string(), obj.serialize());
        obj.insert(k1.clone(), JValue::Boolean(true));
        assert_eq!("{\"key1\":true}".to_string(), obj.serialize());
        obj.insert(k2.clone(), JValue::Boolean(false));
        assert!("{\"key1\":true,\"key2\":false}".to_string() == obj.serialize()
            || "{\"key2\":false,\"key1\":true}".to_string() == obj.serialize());
    }
}
