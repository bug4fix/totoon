# totoon (Go)

**totoon** - Convert any data format to TOON (Token-Oriented Object Notation) for Go

## Installation

In your Go project, run:

```bash
go get github.com/bug4fix/totoon/go@v0.1.0
```

**Note**: You must be in a Go module (a directory with a `go.mod` file). If you don't have one, initialize it first:

```bash
go mod init your-project-name
go get github.com/bug4fix/totoon/go@v0.1.0
```

## Usage

```go
package main

import (
	"fmt"
	"github.com/bug4fix/totoon/go"
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
users[2]{name,age}:
  Alice,30
  Bob,25
```

### Convert from JSON

```go
import "github.com/bug4fix/totoon/go"

jsonStr := `{"name": "Alice", "age": 30}`
toonOutput, err := totoon.JSONToToon(jsonStr)
if err != nil {
	panic(err)
}
fmt.Println(toonOutput)
```

## Quick Test

To quickly test the package, create a test file:

```bash
# Create a new directory
mkdir test-totoon
cd test-totoon

# Initialize a Go module
go mod init test-totoon

# Get the package
go get github.com/bug4fix/totoon/go@v0.1.0

# Create a test file
cat > main.go << 'EOF'
package main

import (
	"fmt"
	"github.com/bug4fix/totoon/go"
)

func main() {
	data := map[string]interface{}{
		"users": []interface{}{
			map[string]interface{}{"name": "Alice", "age": 30},
		},
	}
	fmt.Println(totoon.ToToon(data))
}
EOF

# Run it
go run main.go
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
