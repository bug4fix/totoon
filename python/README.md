# totoon (Python)

**totoon** - Convert any data format to TOON (Token-Oriented Object Notation) for Python

## Installation

```bash
pip install totoon
```

## Usage

### Convert Python dict to TOON

```python
from totoon import to_toon

# Convert Python dict to TOON
data = {
    "users": [
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25}
    ]
}

toon_output = to_toon(data)
print(toon_output)
```

Output:
```
users[2]{name,age}:
  Alice,30
  Bob,25
```

### Convert from JSON

```python
from totoon import json_to_toon

json_str = '{"name": "Alice", "age": 30}'
toon_output = json_to_toon(json_str)
print(toon_output)
```

### Convert from YAML

```python
from totoon import yaml_to_toon

yaml_str = """
users:
  - name: Alice
    age: 30
  - name: Bob
    age: 25
"""
toon_output = yaml_to_toon(yaml_str)
print(toon_output)
```

### Convert from XML

```python
from totoon import xml_to_toon

xml_str = '<root><name>Alice</name><age>30</age></root>'
toon_output = xml_to_toon(xml_str)
print(toon_output)
```

## API

### `to_toon(data: Any, indent: int = 2) -> str`

Convert Python data structures to TOON format.

- `data`: Python value (dict, list, str, int, float, bool, None)
- `indent`: Number of spaces for indentation (default: 2)
- Returns: TOON formatted string

### `json_to_toon(json_str: str) -> str`

Convert JSON string to TOON format.

### `yaml_to_toon(yaml_str: str) -> str`

Convert YAML string to TOON format.

### `xml_to_toon(xml_str: str) -> str`

Convert XML string to TOON format.

### `from_toon(toon_str: str) -> Any`

Convert TOON string back to Python data structures.

**Note**: TOON parsing is not yet implemented. This function will raise `NotImplementedError`.

## Requirements

- Python >= 3.8
- PyYAML >= 6.0

## License

MIT

