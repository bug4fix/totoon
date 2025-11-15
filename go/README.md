# totoon (Go)

**totoon** - Convert any data format to TOON (Token-Oriented Object Notation) for Go

## Installation

```bash
go get github.com/bug4fix/totoon
```

## Usage

```go
package main

import (
	"fmt"
	"github.com/bug4fix/totoon"
)

func main() {
	// Convert Go map to TOON
	data := map[string]interface{}{
		"users": []interface{}{
			map[string]interface{}{"name": "Alice", "age": 30},
			map[string]interface{}{"name": "Bob", "age": 25},
		},
	}

	toonOutput := totoon.ToToon(data)
	fmt.Println(toonOutput)
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

### Convert from JSON

```go
import "github.com/bug4fix/totoon"

jsonStr := `{"name": "Alice", "age": 30}`
toonOutput, err := totoon.JSONToToon(jsonStr)
if err != nil {
	panic(err)
}
fmt.Println(toonOutput)
```

## API

### `ToToon(data ToonValue) string`

Convert Go value to TOON format.

### `ToToonWithIndent(data ToonValue, indent int) string`

Convert Go value to TOON format with custom indentation.

### `JSONToToon(jsonStr string) (string, error)`

Convert JSON string to TOON format.

## License

MIT

