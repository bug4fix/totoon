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
    let mut prefix = " ".repeat(indent * level);

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

    // Header row
    if !key.is_empty() {
        lines.push(format!("{}{}:", prefix, key));
        prefix = " ".repeat(indent * (level + 1));
    }

    let header = all_keys.join(" | ");
    lines.push(format!("{}{}", prefix, header));

    // Separator
    let separator = all_keys.iter().map(|_| "---").collect::<Vec<_>>().join(" | ");
    lines.push(format!("{}{}", prefix, separator));

    // Data rows
    let empty_value = Value::String(String::new());
    for item in arr {
        if let Value::Object(obj) = item {
            let mut row_values = Vec::new();
            for k in &all_keys {
                let value = obj.get(k).unwrap_or(&empty_value);
                let mut value_str = value_to_toon(value, 0, 0);
                // Handle values with pipes or newlines
                if value_str.contains('|') || value_str.contains('\n') {
                    value_str = format!("\"{}\"", value_str);
                }
                row_values.push(value_str);
            }
            let row = row_values.join(" | ");
            lines.push(format!("{}{}", prefix, row));
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
    // Check if string needs escaping
    let needs_escaping = s.chars().any(|c| matches!(c, '\n' | '\t' | ':' | '|' | '"'));

    if !needs_escaping {
        return s.to_string();
    }

    // Escape the string
    let mut result = String::with_capacity(s.len() + 2);
    result.push('"');
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
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
        // Keys are sorted alphabetically in Rust (age comes before name)
        assert!(result.contains("age | name") || result.contains("name | age"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
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
        assert!(result.contains("users:"));
        // Keys are sorted alphabetically (active, age, name)
        assert!(result.contains("active") && result.contains("age") && result.contains("name"));
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
        assert!(result.contains("users:"));
        // Keys are sorted alphabetically
        assert!(result.contains("age") && result.contains("name"));
    }
}

