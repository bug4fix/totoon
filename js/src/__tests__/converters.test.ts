import { jsonToToon, yamlToToon } from '../converters';

describe('jsonToToon', () => {
  test('converts JSON string', () => {
    const jsonStr = '{"name": "Alice", "age": 30}';
    const result = jsonToToon(jsonStr);
    expect(result).toContain('name: Alice');
    expect(result).toContain('age: 30');
  });

  test('converts JavaScript object', () => {
    const data = { name: 'Alice', age: 30 };
    const result = jsonToToon(data);
    expect(result).toContain('name: Alice');
    expect(result).toContain('age: 30');
  });

  test('converts complex JSON structure', () => {
    const jsonStr = '{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}';
    const result = jsonToToon(jsonStr);
    expect(result).toContain('users[');
    expect(result).toContain(',');
  });
});

describe('yamlToToon', () => {
  test('converts YAML string', () => {
    const yamlStr = 'name: Alice\nage: 30';
    const result = yamlToToon(yamlStr);
    expect(result).toContain('name: Alice');
    expect(result).toContain('age: 30');
  });

  test('converts JavaScript object', () => {
    const data = { name: 'Alice', age: 30 };
    const result = yamlToToon(data);
    expect(result).toContain('name: Alice');
    expect(result).toContain('age: 30');
  });
});

