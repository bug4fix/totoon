/**
 * Format converters for various input formats to TOON
 */

import * as yaml from 'js-yaml';
import { parseString } from 'xml2js';
import { promisify } from 'util';
import { toToon, ToonValue } from './core';

const parseXml = promisify(parseString);

/**
 * Convert JSON data to TOON format.
 * 
 * @param data - JSON string, object, or path to JSON file (Node.js only)
 * @returns TOON formatted string
 * 
 * @example
 * ```typescript
 * jsonToToon('{"name": "Alice", "age": 30}')
 * // Returns: "name: Alice\nage: 30"
 * 
 * jsonToToon({ name: "Alice", age: 30 })
 * // Returns: "name: Alice\nage: 30"
 * ```
 */
export function jsonToToon(data: string | object): string {
  let parsed: ToonValue;

  if (typeof data === 'string') {
    parsed = JSON.parse(data);
  } else {
    parsed = data as ToonValue;
  }

  return toToon(parsed);
}

/**
 * Convert YAML data to TOON format.
 * 
 * @param data - YAML string or object
 * @returns TOON formatted string
 * 
 * @example
 * ```typescript
 * yamlToToon('name: Alice\nage: 30')
 * // Returns: "name: Alice\nage: 30"
 * ```
 */
export function yamlToToon(data: string | object): string {
  let parsed: ToonValue;

  if (typeof data === 'string') {
    parsed = yaml.load(data) as ToonValue;
  } else {
    parsed = data as ToonValue;
  }

  return toToon(parsed);
}

/**
 * Convert XML data to TOON format.
 * 
 * @param data - XML string
 * @returns Promise that resolves to TOON formatted string
 * 
 * @example
 * ```typescript
 * await xmlToToon('<root><name>Alice</name><age>30</age></root>')
 * // Returns: "root:\n  name: Alice\n  age: 30"
 * ```
 */
export async function xmlToToon(data: string): Promise<string> {
  const parsed = await parseXml(data);
  return toToon(xmlElementToDict(parsed));
}

function xmlElementToDict(element: any): ToonValue {
  if (typeof element !== 'object' || element === null) {
    return element;
  }

  // Handle arrays
  if (Array.isArray(element)) {
    return element.map(xmlElementToDict);
  }

  const result: { [key: string]: ToonValue } = {};

  // Add attributes
  if (element.$) {
    Object.assign(result, element.$);
  }

  // Process child elements
  for (const [key, value] of Object.entries(element)) {
    if (key === '$') continue; // Skip attributes

    if (Array.isArray(value)) {
      if (value.length === 1) {
        const child = value[0];
        if (typeof child === 'object' && child !== null && Object.keys(child).length === 0) {
          // Empty element with text
          result[key] = '';
        } else if (typeof child === 'string') {
          result[key] = child;
        } else {
          result[key] = xmlElementToDict(child);
        }
      } else {
        result[key] = value.map(xmlElementToDict);
      }
    } else if (typeof value === 'object' && value !== null) {
      result[key] = xmlElementToDict(value);
    } else {
      result[key] = value;
    }
  }

  return result;
}

