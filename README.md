# totoon

**totoon** - Convert any data format to TOON (Token-Oriented Object Notation)

TOON is a compact data format that reduces token usage by 30-60% compared to JSON when interfacing with Large Language Models (LLMs).

[![GitHub](https://img.shields.io/github/license/bug4fix/totoon)](https://github.com/bug4fix/totoon)
[![GitHub stars](https://img.shields.io/github/stars/bug4fix/totoon)](https://github.com/bug4fix/totoon)

## What is TOON?

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

## Language Support

totoon is available for multiple programming languages:

- ✅ **[Python](python/README.md)** - `pip install totoon`
- ✅ **[JavaScript/TypeScript](js/README.md)** - `npm install totoon`
- ✅ **[Go](go/README.md)** - `go get github.com/bug4fix/totoon/go@v0.1.1`
- ✅ **[Rust](rust/README.md)** - Add `totoon = "0.1.1"` to `Cargo.toml`

## Supported Formats

- ✅ JSON
- ✅ YAML
- ✅ XML
- 🔄 CSV (coming soon)
- 🔄 TOML (coming soon)

## Quick Start

### Python

```bash
pip install totoon
```

```python
from totoon import to_toon

data = {"users": [{"name": "Alice", "age": 30}]}
print(to_toon(data))
```

See [Python README](python/README.md) for more details.

### JavaScript/TypeScript

```bash
npm install totoon
```

```typescript
import { toToon } from 'totoon';

const data = { users: [{ name: "Alice", age: 30 }] };
console.log(toToon(data));
```

See [JavaScript/TypeScript README](js/README.md) for more details.

### Go

```bash
go get github.com/bug4fix/totoon/go@v0.1.1
```

```go
import "github.com/bug4fix/totoon/go"

data := map[string]interface{}{
    "users": []interface{}{
        map[string]interface{}{"name": "Alice", "age": 30},
    },
}
fmt.Println(totoon.ToToon(data))
```

See [Go README](go/README.md) for more details.

### Rust

Add to `Cargo.toml`:
```toml
[dependencies]
totoon = "0.1.1"
```

```rust
use totoon::to_toon;
use serde_json::json;

let data = json!({"users": [{"name": "Alice", "age": 30}]});
println!("{}", to_toon(&data));
```

See [Rust README](rust/README.md) for more details.

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
