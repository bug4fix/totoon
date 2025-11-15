# totoon (JavaScript/TypeScript)

**totoon** - Convert any data format to TOON (Token-Oriented Object Notation) for JavaScript/TypeScript

## Installation

```bash
npm install totoon
# or
yarn add totoon
# or
pnpm add totoon
```

## Usage

### TypeScript/JavaScript

```typescript
import { toToon, jsonToToon, yamlToToon, xmlToToon } from 'totoon';

// Convert JavaScript object to TOON
const data = {
  users: [
    { name: "Alice", age: 30 },
    { name: "Bob", age: 25 }
  ]
};

const toonOutput = toToon(data);
console.log(toonOutput);
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

```typescript
import { jsonToToon } from 'totoon';

const jsonStr = '{"name": "Alice", "age": 30}';
const toonOutput = jsonToToon(jsonStr);
console.log(toonOutput);
```

### Convert from YAML

```typescript
import { yamlToToon } from 'totoon';

const yamlStr = `
users:
  - name: Alice
    age: 30
  - name: Bob
    age: 25
`;
const toonOutput = yamlToToon(yamlStr);
console.log(toonOutput);
```

### Convert from XML (async)

```typescript
import { xmlToToon } from 'totoon';

const xmlStr = '<root><name>Alice</name><age>30</age></root>';
const toonOutput = await xmlToToon(xmlStr);
console.log(toonOutput);
```

## API

### `toToon(data: ToonValue, indent?: number): string`

Convert JavaScript data structures to TOON format.

- `data`: JavaScript value (object, array, string, number, boolean, null)
- `indent`: Number of spaces for indentation (default: 2)
- Returns: TOON formatted string

### `jsonToToon(data: string | object): string`

Convert JSON string or object to TOON format.

### `yamlToToon(data: string | object): string`

Convert YAML string or object to TOON format.

### `xmlToToon(data: string): Promise<string>`

Convert XML string to TOON format (async).

## License

MIT

