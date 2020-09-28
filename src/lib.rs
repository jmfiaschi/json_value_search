extern crate serde_json;

use regex::Regex;
use serde_json::Value;
use std::io;

/// Trait used to search elements into Json Value
pub trait Search {
    /// Search elements by path
    fn search(self, path: &str) -> io::Result<Option<Value>>;
    /// Search elements by a list of fields.
    fn search_by_fields(&self, fields: Vec<&str>) -> io::Result<Option<Value>>;
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
            Value::Array(_) | Value::Object(_) => self.search_by_fields(fields),
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
    /// assert_eq!(Some(value_expected_with_wildcard),value.clone().search_by_fields(vec!["field_A","*"]).unwrap());
    ///
    /// let value_expected_for_specific_field: Value = serde_json::from_str(r#"["value_B"]"#).unwrap();
    /// assert_eq!(Some(value_expected_for_specific_field),value.clone().search_by_fields(vec!["field_A","*","field.B"]).unwrap());
    ///
    /// let value_expected_for_specific_index: Value = serde_json::from_str(r#"{"field.B":"value_B"}"#).unwrap();
    /// assert_eq!(Some(value_expected_for_specific_index),value.clone().search_by_fields(vec!["field_A","0"]).unwrap());
    ///
    /// let value_expected_with_regex: Value = serde_json::from_str(r#"["value_B","value_C"]"#).unwrap();
    /// assert_eq!(Some(value_expected_with_regex),value.clone().search_by_fields(vec!["field_A","*","field.+"]).unwrap());
    /// ```
    /// # Examples: Find the same elements in an object or array return the same value.
    /// ```
    /// use json_value_search::Search;
    /// use serde_json::Value;
    ///
    /// let value1: Value = serde_json::from_str(r#"[{"array1":[{"field1":"value1"},{"field2":"value2"}]}]"#).unwrap();
    /// let value2: Value = serde_json::from_str(r#"{"array2":[{"field1":"value1"},{"field2":"value2"}]}"#).unwrap();
    ///
    /// let result1 = value1.search_by_fields(vec!["*","array*","*"]).unwrap();
    /// let result2 = value2.search_by_fields(vec!["array*","*"]).unwrap();
    /// assert_eq!(result1, result2);
    /// ```
    /// # Examples: Not found an element return None.
    /// ```
    /// use json_value_search::Search;
    /// use serde_json::Value;
    ///
    /// let value: Value = serde_json::from_str(r#"{"field_A":[{"field.B":"value_B"},{"field_C":"value_C"}]}"#).unwrap();
    ///
    /// let result = value.clone().search_by_fields(vec!["not_found"]);
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
    /// let result = value.clone().search_by_fields(vec!["["]);
    /// match result {
    ///     Ok(_) => panic!("Should return an error"),
    ///     Err(e) => ()
    /// };
    /// ```
    fn search_by_fields(&self, fields: Vec<&str>) -> io::Result<Option<Value>> {
        if fields.is_empty() {
            return Ok(Some(self.clone()));
        }

        let mut fields = fields.clone();
        let field = fields.remove(0);

        let search_by_number_value = search_by_number(&self, field, fields.clone())?;
        if let Some(_) = search_by_number_value {
            return Ok(search_by_number_value);
        }

        let search_by_wildcard_value = search_by_str(&self, field, fields.clone())?;
        if let Some(_) = search_by_wildcard_value {
            return Ok(search_by_wildcard_value);
        }

        Ok(None)
    }
}

fn search_by_number(
    value: &Value,
    current_field: &str,
    fields: Vec<&str>,
) -> io::Result<Option<Value>> {
    match (value, current_field.parse::<usize>()) {
        (Value::Array(v), Ok(index)) => match v.get(index) {
            Some(value) => value.search_by_fields(fields),
            None => Ok(None),
        },
        (_, _) => Ok(None),
    }
}

fn search_by_str(
    value: &Value,
    current_field: &str,
    fields: Vec<&str>,
) -> io::Result<Option<Value>> {
    match (&value, current_field) {
        (Value::Array(_), "*") => {
            return value.search_by_fields(fields);
        }
        (Value::Array(vec), _) => {
            let mut searched_array: Vec<Value> = Vec::default();
            for value_tmp in vec {
                let searched_value_option =
                    search_by_str(value_tmp, current_field, fields.clone())?;

                match searched_value_option {
                    Some(Value::Array(searched_value)) => {
                        searched_array.append(&mut searched_value.clone())
                    }
                    Some(searched_value) => searched_array.push(searched_value),
                    None => (),
                };
            }
            Ok(match searched_array.is_empty() {
                true => None,
                false => Some(Value::Array(searched_array)),
            })
        }
        (Value::Object(m), _) => {
            let re = Regex::new(current_field)
                .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
            for (key, value_tmp) in m {
                if re.is_match(key.as_str()) {
                    return value_tmp.search_by_fields(fields);
                }
            }
            Ok(None)
        }
        (_, _) => Ok(None),
    }
}
