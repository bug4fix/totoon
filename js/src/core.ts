/**
 * Core TOON conversion functions
 */

export type ToonValue = string | number | boolean | null | ToonObject | ToonArray;
export type ToonObject = { [key: string]: ToonValue };
export type ToonArray = ToonValue[];

/**
 * Convert JavaScript data structures to TOON format.
 * 
 * @param data - JavaScript data structure (object, array, primitive)
 * @param indent - Number of spaces for indentation (default: 2)
 * @returns TOON formatted string
 * 
 * @example
 * ```typescript
 * toToon({ name: "Alice", age: 30 })
 * // Returns: "name: Alice\nage: 30"
 * 
 * toToon([{ name: "Alice" }, { name: "Bob" }])
 * // Returns: "name\n---\nAlice\nBob"
 * ```
 */
export function toToon(data: ToonValue, indent: number = 2): string {
  if (data === null) {
    return 'null';
  }

  if (typeof data === 'boolean') {
    return data ? 'true' : 'false';
  }

  if (typeof data === 'number') {
    return String(data);
  }

  if (typeof data === 'string') {
    // Escape special characters if needed
    if (/[\n\t:|"]/.test(data)) {
      const escaped = data
        .replace(/\\/g, '\\\\')
        .replace(/"/g, '\\"')
        .replace(/\n/g, '\\n');
      return `"${escaped}"`;
    }
    return data;
  }

  if (Array.isArray(data)) {
    return listToToon(data, indent, 0);
  }

  if (typeof data === 'object') {
    return dictToToon(data, indent, 0);
  }

  // Fallback: convert to string
  return String(data);
}

/**
 * Convert TOON format string to JavaScript data structures.
 * 
 * @param toonStr - TOON formatted string
 * @returns JavaScript data structure
 * 
 * @note TOON parsing is not yet implemented. This function will be available
 *       in a future release.
 */
export function fromToon(toonStr: string): ToonValue {
  throw new Error('TOON parsing is not yet implemented');
}

function dictToToon(data: ToonObject, indent: number, level: number): string {
  const keys = Object.keys(data);
  if (keys.length === 0) {
    return '{}';
  }

  const lines: string[] = [];
  const prefix = ' '.repeat(indent * level);

  for (const key of keys) {
    const value = data[key];

    if (
      (typeof value === 'object' && value !== null) &&
      ((Array.isArray(value) && value.length > 0) || (!Array.isArray(value) && Object.keys(value).length > 0))
    ) {
      // Complex nested structure
      if (Array.isArray(value)) {
        // Check if it's a list of objects (tabular format)
        if (value.length > 0 && typeof value[0] === 'object' && value[0] !== null && !Array.isArray(value[0])) {
          lines.push(listOfObjectsToToon(key, value as ToonObject[], indent, level));
        } else {
          lines.push(`${prefix}${key}:`);
          lines.push(listToToon(value, indent, level + 1));
        }
      } else {
        lines.push(`${prefix}${key}:`);
        lines.push(dictToToon(value as ToonObject, indent, level + 1));
      }
    } else {
      // Simple value
      const valueStr = valueToToon(value, indent, level + 1);
      lines.push(`${prefix}${key}: ${valueStr}`);
    }
  }

  return lines.join('\n');
}

function listToToon(data: ToonArray, indent: number, level: number): string {
  if (data.length === 0) {
    return '[]';
  }

  // Check if it's a list of objects (use tabular format)
  if (data.length > 0 && typeof data[0] === 'object' && data[0] !== null && !Array.isArray(data[0])) {
    return listOfObjectsToToon('', data as ToonObject[], indent, level);
  }

  // Simple list
  const lines: string[] = [];
  const prefix = ' '.repeat(indent * level);
  for (const item of data) {
    const valueStr = valueToToon(item, indent, level);
    lines.push(`${prefix}- ${valueStr}`);
  }

  return lines.join('\n');
}

function listOfObjectsToToon(
  key: string,
  data: ToonObject[],
  indent: number,
  level: number
): string {
  if (data.length === 0 || typeof data[0] !== 'object' || data[0] === null || Array.isArray(data[0])) {
    return listToToon(data as ToonArray, indent, level);
  }

  const lines: string[] = [];
  let prefix = ' '.repeat(indent * level);

  // Get all unique keys from all objects, preserving insertion order
  const allKeysDict: { [key: string]: boolean } = {};
  for (const obj of data) {
    for (const objKey of Object.keys(obj)) {
      allKeysDict[objKey] = true;
    }
  }
  const allKeys = Object.keys(allKeysDict);

  if (allKeys.length === 0) {
    return '[]';
  }

  // Header row (keys)
  if (key) {
    lines.push(`${prefix}${key}:`);
    prefix = ' '.repeat(indent * (level + 1));
  }

  const header = allKeys.join(' | ');
  lines.push(`${prefix}${header}`);

  // Separator
  const separator = allKeys.map(() => '---').join(' | ');
  lines.push(`${prefix}${separator}`);

  // Data rows
  for (const obj of data) {
    const rowValues: string[] = [];
    for (const k of allKeys) {
      const value = obj[k] ?? '';
      let valueStr = valueToToon(value, 0, 0);
      // Handle values with pipes or newlines
      if (valueStr.includes('|') || valueStr.includes('\n')) {
        valueStr = `"${valueStr}"`;
      }
      rowValues.push(valueStr);
    }
    const row = rowValues.join(' | ');
    lines.push(`${prefix}${row}`);
  }

  return lines.join('\n');
}

function valueToToon(value: ToonValue, indent: number, level: number): string {
  if (value === null) {
    return 'null';
  }

  if (typeof value === 'boolean') {
    return value ? 'true' : 'false';
  }

  if (typeof value === 'number') {
    return String(value);
  }

  if (typeof value === 'string') {
    // Escape if contains special characters
    if (/[\n\t:|"]/.test(value)) {
      const escaped = value
        .replace(/\\/g, '\\\\')
        .replace(/"/g, '\\"')
        .replace(/\n/g, '\\n');
      return `"${escaped}"`;
    }
    return value;
  }

  if (Array.isArray(value)) {
    return '\n' + listToToon(value, indent, level);
  }

  if (typeof value === 'object') {
    return '\n' + dictToToon(value, indent, level);
  }

  return String(value);
}

