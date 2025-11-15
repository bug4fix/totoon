import { toToon } from '../core';

describe('toToon', () => {
  test('converts simple object', () => {
    const data = { name: 'Alice', age: 30 };
    const result = toToon(data);
    expect(result).toContain('name: Alice');
    expect(result).toContain('age: 30');
  });

  test('converts nested object', () => {
    const data = {
      user: {
        name: 'Alice',
        details: {
          age: 30,
          city: 'NYC',
        },
      },
    };
    const result = toToon(data);
    expect(result).toContain('user:');
    expect(result).toContain('name: Alice');
    expect(result).toContain('details:');
    expect(result).toContain('age: 30');
  });

  test('converts list of objects to tabular format', () => {
    const data = [
      { name: 'Alice', age: 30 },
      { name: 'Bob', age: 25 },
    ];
    const result = toToon(data);
    expect(result).toContain('[2]{');
    expect(result).toContain('Alice');
    expect(result).toContain('Bob');
    expect(result).toContain(','); // Comma-separated values
  });

  test('converts simple list', () => {
    const data = [1, 2, 3];
    const result = toToon(data);
    expect(result).toContain('- 1');
    expect(result).toContain('- 2');
    expect(result).toContain('- 3');
  });

  test('converts primitives', () => {
    expect(toToon(null)).toBe('null');
    expect(toToon(true)).toBe('true');
    expect(toToon(false)).toBe('false');
    expect(toToon(42)).toBe('42');
    expect(toToon(3.14)).toBe('3.14');
    expect(toToon('hello')).toBe('hello');
  });

  test('handles string escaping for special characters', () => {
    const data = { message: 'Hello\nWorld' };
    const result = toToon(data);
    expect(result).toContain('"');
    expect(result).toContain('Hello');
    expect(result).toContain('World');
  });

  test('converts complex nested structure', () => {
    const data = {
      users: [
        { name: 'Alice', age: 30, active: true },
        { name: 'Bob', age: 25, active: false },
      ],
      metadata: {
        count: 2,
        timestamp: '2024-01-01',
      },
    };
    const result = toToon(data);
    expect(result).toContain('users[');
    expect(result).toContain(','); // Comma-separated values
    expect(result).toContain('metadata:');
    expect(result).toContain('count: 2');
  });

  test('handles empty objects', () => {
    const data = {};
    const result = toToon(data);
    expect(result).toBe('{}');
  });

  test('handles empty arrays', () => {
    const data: any[] = [];
    const result = toToon(data);
    expect(result).toBe('[]');
  });

  test('handles mixed arrays', () => {
    const data = [1, 'hello', true, null];
    const result = toToon(data);
    expect(result).toContain('- 1');
    expect(result).toContain('- hello');
    expect(result).toContain('- true');
    expect(result).toContain('- null');
  });
});

