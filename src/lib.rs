extern crate serde_json;

use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref RE_CACHE: Mutex<HashMap<String, Regex>> = Mutex::new(HashMap::new());
}

fn get_or_create_regex(pattern: &str) -> io::Result<Regex> {
    let mut cache = RE_CACHE.lock().unwrap();

    if let Some(re) = cache.get(pattern) {
        return Ok(re.clone());
    }

    let re = Regex::new(pattern).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    cache.insert(pattern.to_string(), re.clone());

    Ok(re)
}

/// Trait used to search elements into Json Value
pub trait Search {
    /// Search elements by path
    fn search(self, path: &str) -> io::Result<Option<Value>>;
    /// Search elements by a list of fields.
    fn search_by_fields(&self, fields: &[&str]) -> io::Result<Option<Value>>;
}

impl Search for serde_json::Value {
    /// # Examples: Find elements in an object.
    /// ```
    /// use json_value_search::Search;
    /// use serde_json::Value;
    ///
    /// let value: Value = serde_json::from_str(r#"{"field_A":[{"field.B":"value_B"},{"field_C":"value_C"}]}"#).unwrap();
    ///
    /// let value_expected_with_wildcard: Value = serde_json::from_str(r#"[{"field.B":"value_B"},{"field_C":"value_C"}]"#).unwrap();
    /// assert_eq!(Some(value_expected_with_wildcard),value.clone().search(r#"/field_A/*"#).unwrap());
    ///
    /// let value_expected_for_specific_field: Value = serde_json::from_str(r#"["value_B"]"#).unwrap();
    /// assert_eq!(Some(value_expected_for_specific_field),value.clone().search(r#"/field_A/*/field.B"#).unwrap());
    ///
    /// let value_expected_for_specific_index: Value = serde_json::from_str(r#"{"field.B":"value_B"}"#).unwrap();
    /// assert_eq!(Some(value_expected_for_specific_index),value.clone().search(r#"/field_A/0"#).unwrap());
    ///
    /// let value_expected_with_regex: Value = serde_json::from_str(r#"["value_B","value_C"]"#).unwrap();
    /// assert_eq!(Some(value_expected_with_regex),value.clone().search(r#"/field_A/*/field.+"#).unwrap());
    /// ```
    /// # Examples: Find the same elements in an object or array return the same value.
    /// ```
    /// use json_value_search::Search;
    /// use serde_json::Value;
    ///
    /// let value1: Value = serde_json::from_str(r#"[{"array1":[{"field1":"value1"},{"field2":"value2"}]}]"#).unwrap();
    /// let value2: Value = serde_json::from_str(r#"{"array2":[{"field1":"value1"},{"field2":"value2"}]}"#).unwrap();
    ///
    /// let result1 = value1.search(r#"/*/array*/*"#).unwrap();
    /// let result2 = value2.search(r#"/array*/*"#).unwrap();
    /// assert_eq!(result1, result2);
    /// ```
    /// # Examples: Not found an element return None.
    /// ```
    /// use json_value_search::Search;
    /// use serde_json::Value;
    ///
    /// let value: Value = serde_json::from_str(r#"{"field_A":[{"field.B":"value_B"},{"field_C":"value_C"}]}"#).unwrap();
    ///
    /// let result = value.clone().search(r#"/not_found/*"#);
    /// match result {
    ///     Ok(None) => (),
    ///     Ok(Some(_)) => panic!("Should return None"),
    ///     Err(e) => panic!(format!("Should not be in error, {}", e))
    /// };
    /// ```
    /// # Examples: Write a bad regex return an error.
    /// ```
    /// use json_value_search::Search;
    /// use serde_json::Value;
    ///
    /// let value: Value = serde_json::from_str(r#"{"field_A":[{"field.B":"value_B"},{"field_C":"value_C"}]}"#).unwrap();
    ///
    /// let result = value.clone().search(r#"/["#);
    /// match result {
    ///     Ok(_) => panic!("Should return an error"),
    ///     Err(e) => ()
    /// };
    /// ```
    fn search(self, path: &str) -> io::Result<Option<Value>> {
        let fields: Vec<&str> = path.split("/").skip(1).collect();

        match self {
            Value::Array(_) | Value::Object(_) => self.search_by_fields(&fields),
            _ => Ok(None),
        }
    }
    /// # Examples: Find elements in an object.
    /// ```
    /// use json_value_search::Search;
    /// use serde_json::Value;
    ///
    /// let value: Value = serde_json::from_str(r#"{"field_A":[{"field.B":"value_B"},{"field_C":"value_C"}]}"#).unwrap();
    ///
    /// let value_expected_with_wildcard: Value = serde_json::from_str(r#"[{"field.B":"value_B"},{"field_C":"value_C"}]"#).unwrap();
    /// assert_eq!(Some(value_expected_with_wildcard),value.clone().search_by_fields(&vec!["field_A","*"]).unwrap());
    ///
    /// let value_expected_for_specific_field: Value = serde_json::from_str(r#"["value_B"]"#).unwrap();
    /// assert_eq!(Some(value_expected_for_specific_field),value.clone().search_by_fields(&vec!["field_A","*","field.B"]).unwrap());
    ///
    /// let value_expected_for_specific_index: Value = serde_json::from_str(r#"{"field.B":"value_B"}"#).unwrap();
    /// assert_eq!(Some(value_expected_for_specific_index),value.clone().search_by_fields(&vec!["field_A","0"]).unwrap());
    ///
    /// let value_expected_with_regex: Value = serde_json::from_str(r#"["value_B","value_C"]"#).unwrap();
    /// assert_eq!(Some(value_expected_with_regex),value.clone().search_by_fields(&vec!["field_A","*","field.+"]).unwrap());
    /// ```
    /// # Examples: Find the same elements in an object or array return the same value.
    /// ```
    /// use json_value_search::Search;
    /// use serde_json::Value;
    ///
    /// let value1: Value = serde_json::from_str(r#"[{"array1":[{"field1":"value1"},{"field2":"value2"}]}]"#).unwrap();
    /// let value2: Value = serde_json::from_str(r#"{"array2":[{"field1":"value1"},{"field2":"value2"}]}"#).unwrap();
    ///
    /// let result1 = value1.search_by_fields(&vec!["*","array*","*"]).unwrap();
    /// let result2 = value2.search_by_fields(&vec!["array*","*"]).unwrap();
    /// assert_eq!(result1, result2);
    /// ```
    /// # Examples: Not found an element return None.
    /// ```
    /// use json_value_search::Search;
    /// use serde_json::Value;
    ///
    /// let value: Value = serde_json::from_str(r#"{"field_A":[{"field.B":"value_B"},{"field_C":"value_C"}]}"#).unwrap();
    ///
    /// let result = value.clone().search_by_fields(&vec!["not_found"]);
    /// match result {
    ///     Ok(None) => (),
    ///     Ok(Some(_)) => panic!("Should return None"),
    ///     Err(e) => panic!(format!("Should not be in error, {}", e))
    /// };
    /// ```
    /// # Examples: Write a bad regex return an error.
    /// ```
    /// use json_value_search::Search;
    /// use serde_json::Value;
    ///
    /// let value: Value = serde_json::from_str(r#"{"field_A":[{"field.B":"value_B"},{"field_C":"value_C"}]}"#).unwrap();
    ///
    /// let result = value.clone().search_by_fields(&vec!["["]);
    /// match result {
    ///     Ok(_) => panic!("Should return an error"),
    ///     Err(e) => ()
    /// };
    /// ```
    fn search_by_fields(&self, fields: &[&str]) -> io::Result<Option<Value>> {
        if fields.is_empty() {
            return Ok(Some(self.clone()));
        }

        let field = fields[0];
        let remaining_fields = &fields[1..];

        let index = field.parse::<usize>().ok();
        if let Some(value) = search_by_number(self, index, remaining_fields)? {
            return Ok(Some(value));
        }

        if let Some(value) = search_by_str(self, field, remaining_fields)? {
            return Ok(Some(value));
        }

        Ok(None)
    }
}

#[inline]
fn search_by_number(
    value: &Value,
    index: Option<usize>,
    fields: &[&str],
) -> io::Result<Option<Value>> {
    match (value, index) {
        (Value::Array(v), Some(index)) => match v.get(index) {
            Some(value) => value.search_by_fields(fields),
            None => Ok(None),
        },
        (_, _) => Ok(None),
    }
}

#[inline]
fn search_by_str(value: &Value, current_field: &str, fields: &[&str]) -> io::Result<Option<Value>> {
    match (value, current_field) {
        (Value::Array(_), "*") => value.search_by_fields(fields),

        (Value::Array(vec), _) => {
            let mut searched_array: Vec<Value> = Vec::with_capacity(vec.len());
            for value_tmp in vec {
                if let Some(searched_value) = search_by_str(value_tmp, current_field, fields)? {
                    match searched_value {
                        Value::Array(mut inner_values) => searched_array.append(&mut inner_values),
                        _ => searched_array.push(searched_value),
                    }
                }
            }
            if searched_array.is_empty() {
                Ok(None)
            } else {
                Ok(Some(Value::Array(searched_array)))
            }
        }

        (Value::Object(m), _) => {
            let re = get_or_create_regex(current_field)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
            for (key, value_tmp) in m {
                if re.is_match(key.as_str()) {
                    return value_tmp.search_by_fields(fields);
                }
            }
            Ok(None)
        }

        _ => Ok(None),
    }
}
