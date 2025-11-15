//! totoon - Convert any data format to TOON (Token-Oriented Object Notation)
//!
//! TOON is a compact data format that reduces token usage by 30-60% compared to JSON
//! when interfacing with Large Language Models (LLMs).

use serde_json::Value;
use std::collections::HashMap;

/// Convert a serde_json::Value to TOON format string
///
/// # Examples
///
/// ```
/// use totoon::to_toon;
/// use serde_json::json;
///
/// let data = json!({
///     "name": "Alice",
///     "age": 30
/// });
///
/// let toon = to_toon(&data);
/// println!("{}", toon);
/// ```
pub fn to_toon(value: &Value) -> String {
    to_toon_with_indent(value, 2, 0)
}

/// Convert a serde_json::Value to TOON format with custom indentation
pub fn to_toon_with_indent(value: &Value, indent: usize, level: usize) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => escape_string(s),
        Value::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }
            // Check if it's a list of objects (tabular format)
            if let Some(Value::Object(_)) = arr.first() {
                if arr.iter().all(|v| v.is_object()) {
                    return list_of_objects_to_toon("", arr, indent, level);
                }
            }
            list_to_toon(arr, indent, level)
        }
        Value::Object(obj) => dict_to_toon(obj, indent, level),
    }
}

/// Convert JSON string to TOON format
///
/// # Examples
///
/// ```
/// use totoon::json_to_toon;
///
/// let json_str = r#"{"name": "Alice", "age": 30}"#;
/// let toon = json_to_toon(json_str).unwrap();
/// println!("{}", toon);
/// ```
pub fn json_to_toon(json_str: &str) -> Result<String, serde_json::Error> {
    let value: Value = serde_json::from_str(json_str)?;
    Ok(to_toon(&value))
}

fn dict_to_toon(obj: &serde_json::Map<String, Value>, indent: usize, level: usize) -> String {
    if obj.is_empty() {
        return "{}".to_string();
    }

    let mut lines = Vec::new();
    let prefix = " ".repeat(indent * level);

    for (key, value) in obj {
        match value {
            Value::Object(inner_obj) if !inner_obj.is_empty() => {
                lines.push(format!("{}{}:", prefix, key));
                lines.push(dict_to_toon(inner_obj, indent, level + 1));
            }
            Value::Array(arr) if !arr.is_empty() => {
                // Check if it's a list of objects
                if let Some(Value::Object(_)) = arr.first() {
                    if arr.iter().all(|v| v.is_object()) {
                        lines.push(list_of_objects_to_toon(key, arr, indent, level));
                        continue;
                    }
                }
                lines.push(format!("{}{}:", prefix, key));
                lines.push(list_to_toon(arr, indent, level + 1));
            }
            _ => {
                let value_str = value_to_toon(value, indent, level + 1);
                lines.push(format!("{}{}: {}", prefix, key, value_str));
            }
        }
    }

    lines.join("\n")
}

fn list_to_toon(arr: &[Value], indent: usize, level: usize) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    }

    let mut lines = Vec::new();
    let prefix = " ".repeat(indent * level);

    for item in arr {
        let value_str = value_to_toon(item, indent, level);
        lines.push(format!("{}- {}", prefix, value_str));
    }

    lines.join("\n")
}

fn list_of_objects_to_toon(
    key: &str,
    arr: &[Value],
    indent: usize,
    level: usize,
) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    }

    let mut lines = Vec::new();
    let prefix = " ".repeat(indent * level);

    // Collect all unique keys from all objects
    let mut seen_keys = HashMap::new();

    for item in arr {
        if let Value::Object(obj) = item {
            for k in obj.keys() {
                seen_keys.insert(k.clone(), true);
            }
        }
    }

    if seen_keys.is_empty() {
        return "[]".to_string();
    }

    // Sort keys for consistent output (HashMap doesn't preserve order)
    let mut all_keys: Vec<String> = seen_keys.keys().cloned().collect();
    all_keys.sort();

    // Header format: key[count]{field1,field2,field3}:
    let count = arr.len();
    let fields = all_keys.join(",");
    if !key.is_empty() {
        lines.push(format!("{}{}[{}]{{{}}}:", prefix, key, count, fields));
    } else {
        lines.push(format!("{}[{}]{{{}}}:", prefix, count, fields));
    }

    // Data rows: comma-separated values with 2 spaces indentation
    let data_prefix = "  "; // Two spaces for data rows
    let empty_value = Value::String(String::new());
    for item in arr {
        if let Value::Object(obj) = item {
            let mut row_values = Vec::new();
            for k in &all_keys {
                let value = obj.get(k).unwrap_or(&empty_value);
                let value_str = match value {
                    Value::Array(arr_val) => {
                        if arr_val.is_empty() {
                            "[]".to_string()
                        } else if let Some(Value::Object(_)) = arr_val.first() {
                            // Array of objects: use compact inline tabular format
                            let mut nested_keys_map = HashMap::new();
                            for nested_item in arr_val {
                                if let Value::Object(nested_obj) = nested_item {
                                    for nk in nested_obj.keys() {
                                        nested_keys_map.insert(nk.clone(), true);
                                    }
                                }
                            }
                            let mut nested_keys: Vec<String> = nested_keys_map.keys().cloned().collect();
                            nested_keys.sort();
                            let nested_fields = nested_keys.join(",");
                            let nested_count = arr_val.len();
                            
                            // Build compact data rows separated by semicolons
                            let mut nested_rows = Vec::new();
                            for nested_item in arr_val {
                                if let Value::Object(nested_obj) = nested_item {
                                    let mut nested_row_values = Vec::new();
                                    for nk in &nested_keys {
                                        let nv = nested_obj.get(nk).unwrap_or(&empty_value);
                                        let mut nv_str = value_to_toon(nv, 0, 0);
                                        if nv_str.contains(',') || nv_str.contains(';') || nv_str.contains(':') {
                                            nv_str = format!("\"{}\"", nv_str);
                                        }
                                        nested_row_values.push(nv_str);
                                    }
                                    nested_rows.push(nested_row_values.join(","));
                                }
                            }
                            format!("[{}]{{{}}}:{}", nested_count, nested_fields, nested_rows.join(";"))
                        } else {
                            // Array of primitives: use bracket notation
                            let items: Vec<String> = arr_val.iter().map(|v| value_to_toon(v, 0, 0)).collect();
                            format!("[{}]", items.join(","))
                        }
                    }
                    Value::Object(nested_obj) => {
                        // Nested object: use compact key:value format
                        let mut nested_items = Vec::new();
                        let mut nested_keys: Vec<String> = nested_obj.keys().cloned().collect();
                        nested_keys.sort();
                        for nk in nested_keys {
                            let nv = nested_obj.get(&nk).unwrap_or(&empty_value);
                            let mut nv_str = value_to_toon(nv, 0, 0);
                            if nv_str.contains(',') || nv_str.contains(':') {
                                nv_str = format!("\"{}\"", nv_str);
                            }
                            nested_items.push(format!("{}:{}", nk, nv_str));
                        }
                        format!("{{{}}}", nested_items.join(","))
                    }
                    _ => {
                        let mut value_str = value_to_toon(value, 0, 0);
                        // Handle values with commas, newlines, colons, or semicolons
                        // Only quote if not already quoted and contains special chars
                        if !(value_str.starts_with('"') && value_str.ends_with('"')) {
                            if value_str.contains(',') || value_str.contains('\n') || value_str.contains(':') || value_str.contains(';') {
                                // Escape quotes if present
                                if value_str.contains('"') {
                                    value_str = value_str.replace('"', "\\\"");
                                }
                                value_str = format!("\"{}\"", value_str);
                            }
                        }
                        value_str
                    }
                };
                row_values.push(value_str);
            }
            let row = row_values.join(",");
            lines.push(format!("{}{}", data_prefix, row));
        }
    }

    lines.join("\n")
}

fn value_to_toon(value: &Value, indent: usize, level: usize) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => escape_string(s),
        Value::Array(arr) => "\n".to_string() + &list_to_toon(arr, indent, level),
        Value::Object(obj) => "\n".to_string() + &dict_to_toon(obj, indent, level),
    }
}

fn escape_string(s: &str) -> String {
    // Only escape actual control characters (newlines, tabs, etc.)
    // Let the caller decide if quoting is needed for other special chars
    let has_control_chars = s.chars().any(|c| matches!(c, '\n' | '\t' | '\r'));

    if !has_control_chars {
        return s.to_string();
    }

    // Escape control characters
    let mut result = String::with_capacity(s.len() + 2);
    result.push('"');
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result.push('"');
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_object() {
        let data = json!({
            "name": "Alice",
            "age": 30
        });
        let result = to_toon(&data);
        assert!(result.contains("name: Alice"));
        assert!(result.contains("age: 30"));
    }

    #[test]
    fn test_nested_object() {
        let data = json!({
            "user": {
                "name": "Alice",
                "details": {
                    "age": 30,
                    "city": "NYC"
                }
            }
        });
        let result = to_toon(&data);
        assert!(result.contains("user:"));
        assert!(result.contains("name: Alice"));
        assert!(result.contains("details:"));
        assert!(result.contains("age: 30"));
    }

    #[test]
    fn test_list_of_objects() {
        let data = json!([
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25}
        ]);
        let result = to_toon(&data);
        // Should have [2]{fields}: format
        assert!(result.contains("[2]{"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
        assert!(result.contains(",")); // Comma-separated values
    }

    #[test]
    fn test_simple_list() {
        let data = json!([1, 2, 3]);
        let result = to_toon(&data);
        assert!(result.contains("- 1"));
        assert!(result.contains("- 2"));
        assert!(result.contains("- 3"));
    }

    #[test]
    fn test_primitives() {
        assert_eq!(to_toon(&Value::Null), "null");
        assert_eq!(to_toon(&json!(true)), "true");
        assert_eq!(to_toon(&json!(false)), "false");
        assert_eq!(to_toon(&json!(42)), "42");
        assert_eq!(to_toon(&json!(3.14)), "3.14");
        assert_eq!(to_toon(&json!("hello")), "hello");
    }

    #[test]
    fn test_string_escaping() {
        let data = json!({
            "message": "Hello\nWorld"
        });
        let result = to_toon(&data);
        assert!(result.contains("\""));
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));
    }

    #[test]
    fn test_complex_structure() {
        let data = json!({
            "users": [
                {"name": "Alice", "age": 30, "active": true},
                {"name": "Bob", "age": 25, "active": false}
            ],
            "metadata": {
                "count": 2,
                "timestamp": "2024-01-01"
            }
        });
        let result = to_toon(&data);
        assert!(result.contains("users["));
        assert!(result.contains(",")); // Comma-separated values
        assert!(result.contains("metadata:"));
        assert!(result.contains("count: 2"));
    }

    #[test]
    fn test_empty_object() {
        let data = json!({});
        let result = to_toon(&data);
        assert_eq!(result, "{}");
    }

    #[test]
    fn test_empty_array() {
        let data = json!([]);
        let result = to_toon(&data);
        assert_eq!(result, "[]");
    }

    #[test]
    fn test_json_to_toon() {
        let json_str = r#"{"name": "Alice", "age": 30}"#;
        let result = json_to_toon(json_str).unwrap();
        assert!(result.contains("name: Alice"));
        assert!(result.contains("age: 30"));
    }

    #[test]
    fn test_json_to_toon_complex() {
        let json_str = r#"{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}"#;
        let result = json_to_toon(json_str).unwrap();
        assert!(result.contains("users["));
        assert!(result.contains(",")); // Comma-separated values
    }
}

