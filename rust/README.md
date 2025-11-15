# totoon (Rust)

**totoon** - Convert any data format to TOON (Token-Oriented Object Notation) for Rust

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
totoon = "0.1.0"
serde_json = "1.0"
```

## Usage

```rust
use totoon::to_toon;
use serde_json::json;

fn main() {
    // Convert JSON value to TOON
    let data = json!({
        "users": [
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25}
        ]
    });

    let toon_output = to_toon(&data);
    println!("{}", toon_output);
}
```

Output:
```
users:
  name | age
  --- | ---
  Alice | 30
  Bob | 25
```

### Convert from JSON String

```rust
use totoon::json_to_toon;

let json_str = r#"{"name": "Alice", "age": 30}"#;
let toon_output = json_to_toon(json_str).unwrap();
println!("{}", toon_output);
```

## API

### `to_toon(value: &Value) -> String`

Convert a `serde_json::Value` to TOON format.

### `to_toon_with_indent(value: &Value, indent: usize, level: usize) -> String`

Convert a `serde_json::Value` to TOON format with custom indentation.

### `json_to_toon(json_str: &str) -> Result<String, serde_json::Error>`

Convert JSON string to TOON format.

## License

MIT

