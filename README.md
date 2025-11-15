# totoon

**totoon** - Convert any data format to TOON (Token-Oriented Object Notation)

TOON is a compact data format that reduces token usage by 30-60% compared to JSON when interfacing with Large Language Models (LLMs).

[![GitHub](https://img.shields.io/github/license/bug4fix/totoon)](https://github.com/bug4fix/totoon)
[![GitHub stars](https://img.shields.io/github/stars/bug4fix/totoon)](https://github.com/bug4fix/totoon)

## Features

- 🚀 Convert JSON, YAML, XML, and more to TOON format
- 📦 Multi-language SDK support (starting with Python)
- ⚡ High-performance conversion
- 🔧 Easy to integrate
- 📚 Comprehensive documentation

## Installation

### Python

```bash
pip install totoon
```

### Go

In your Go project (must be in a directory with `go.mod`):

```bash
go get github.com/bug4fix/totoon/go@v0.1.0
```

If you don't have a Go module yet, initialize one first:
```bash
go mod init your-project-name
go get github.com/bug4fix/totoon/go@v0.1.0
```

### JavaScript/TypeScript

```bash
npm install totoon
```

### Rust

Add to your `Cargo.toml`:
```toml
[dependencies]
totoon = "0.1.0"
```

## Quick Start

### Python

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

## Why TOON?

TOON (Token-Oriented Object Notation) is designed specifically for LLM interactions:

- **30-60% token reduction** compared to JSON
- **Tabular format** for arrays of objects (common in LLM data)
- **Human-readable** while being compact
- **Efficient** for API calls to LLMs

### Example Comparison

**JSON:**
```json
{
  "users": [
    {"name": "Alice", "age": 30},
    {"name": "Bob", "age": 25}
  ]
}
```
*~80 tokens*

**TOON:**
```
users[2]{name,age}:
  Alice,30
  Bob,25
```
*~35 tokens (56% reduction!)*

## Supported Formats

- ✅ JSON
- ✅ YAML
- ✅ XML
- 🔄 CSV (coming soon)
- 🔄 TOML (coming soon)

## Language Support

- ✅ Python
- ✅ JavaScript/TypeScript
- ✅ Go
- ✅ Rust

## Repository

- **GitHub**: https://github.com/bug4fix/totoon
- **Issues**: https://github.com/bug4fix/totoon/issues
- **Discussions**: https://github.com/bug4fix/totoon/discussions

## License

MIT License - See [LICENSE](LICENSE) file for details

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md).


## Business Model

This project follows an **Open Source + Enterprise SaaS** model:
- **Open Source**: Core SDKs are free and open source
- **Enterprise**: Cloud API, advanced features, and support available for enterprise customers (coming soon)

