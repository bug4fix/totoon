package totoon

import (
	"encoding/json"
	"fmt"
	"strings"
)

// ToonValue represents any value that can be converted to TOON format
type ToonValue interface{}

// ToToon converts a Go value to TOON format string
func ToToon(data ToonValue) string {
	return toToon(data, 2, 0)
}

// ToToonWithIndent converts a Go value to TOON format with custom indentation
func ToToonWithIndent(data ToonValue, indent int) string {
	return toToon(data, indent, 0)
}

// JSONToToon converts JSON string to TOON format
func JSONToToon(jsonStr string) (string, error) {
	var data interface{}
	if err := json.Unmarshal([]byte(jsonStr), &data); err != nil {
		return "", err
	}
	return ToToon(data), nil
}

func toToon(data ToonValue, indent int, level int) string {
	if data == nil {
		return "null"
	}

	switch v := data.(type) {
	case bool:
		if v {
			return "true"
		}
		return "false"
	case int, int8, int16, int32, int64, uint, uint8, uint16, uint32, uint64, float32, float64:
		return fmt.Sprintf("%v", v)
	case string:
		return escapeString(v)
	case []interface{}:
		return listToToon(v, indent, level)
	case map[string]interface{}:
		return dictToToon(v, indent, level)
	case []map[string]interface{}:
		// Convert to []interface{} for processing
		list := make([]interface{}, len(v))
		for i, item := range v {
			list[i] = item
		}
		return listToToon(list, indent, level)
	default:
		// Try to convert to JSON and back to handle custom types
		jsonBytes, err := json.Marshal(data)
		if err != nil {
			return fmt.Sprintf("%v", data)
		}
		var converted interface{}
		if err := json.Unmarshal(jsonBytes, &converted); err != nil {
			return fmt.Sprintf("%v", data)
		}
		return toToon(converted, indent, level)
	}
}

func dictToToon(data map[string]interface{}, indent int, level int) string {
	if len(data) == 0 {
		return "{}"
	}

	var lines []string
	prefix := strings.Repeat(" ", indent*level)

	for key, value := range data {
		keyStr := key

		// Check if value is complex
		isComplex := false
		var isListOfObjects bool

		switch val := value.(type) {
		case map[string]interface{}:
			isComplex = len(val) > 0
		case []interface{}:
			isComplex = len(val) > 0
			if len(val) > 0 {
				_, isListOfObjects = val[0].(map[string]interface{})
			}
		case []map[string]interface{}:
			isComplex = len(val) > 0
			isListOfObjects = true
		}

		if isComplex {
			if isListOfObjects {
				// Convert to []interface{} for listOfObjectsToToon
				var list []interface{}
				switch val := value.(type) {
				case []interface{}:
					list = val
				case []map[string]interface{}:
					list = make([]interface{}, len(val))
					for i, item := range val {
						list[i] = item
					}
				}
				lines = append(lines, listOfObjectsToToon(keyStr, list, indent, level))
			} else if _, ok := value.(map[string]interface{}); ok {
				lines = append(lines, fmt.Sprintf("%s%s:", prefix, keyStr))
				lines = append(lines, dictToToon(value.(map[string]interface{}), indent, level+1))
			} else {
				lines = append(lines, fmt.Sprintf("%s%s:", prefix, keyStr))
				lines = append(lines, listToToon(value.([]interface{}), indent, level+1))
			}
		} else {
			valueStr := valueToToon(value, indent, level+1)
			lines = append(lines, fmt.Sprintf("%s%s: %s", prefix, keyStr, valueStr))
		}
	}

	return strings.Join(lines, "\n")
}

func listToToon(data []interface{}, indent int, level int) string {
	if len(data) == 0 {
		return "[]"
	}

	// Check if it's a list of objects (use tabular format)
	if len(data) > 0 {
		if _, ok := data[0].(map[string]interface{}); ok {
			return listOfObjectsToToon("", data, indent, level)
		}
	}

	// Simple list
	var lines []string
	prefix := strings.Repeat(" ", indent*level)
	for _, item := range data {
		valueStr := valueToToon(item, indent, level)
		lines = append(lines, fmt.Sprintf("%s- %s", prefix, valueStr))
	}

	return strings.Join(lines, "\n")
}

func listOfObjectsToToon(key string, data []interface{}, indent int, level int) string {
	if len(data) == 0 {
		return "[]"
	}

	// Verify first element is an object
	if _, ok := data[0].(map[string]interface{}); !ok {
		return listToToon(data, indent, level)
	}

	var lines []string
	prefix := strings.Repeat(" ", indent*level)

	// Get all unique keys from all objects, preserving order
	allKeysMap := make(map[string]bool)
	var allKeys []string

	for _, item := range data {
		if obj, ok := item.(map[string]interface{}); ok {
			for k := range obj {
				if !allKeysMap[k] {
					allKeysMap[k] = true
					allKeys = append(allKeys, k)
				}
			}
		}
	}

	if len(allKeys) == 0 {
		return "[]"
	}

	// Header format: key[count]{field1,field2,field3}:
	count := len(data)
	fields := strings.Join(allKeys, ",")
	if key != "" {
		lines = append(lines, fmt.Sprintf("%s%s[%d]{%s}:", prefix, key, count, fields))
	} else {
		lines = append(lines, fmt.Sprintf("%s[%d]{%s}:", prefix, count, fields))
	}

	// Data rows: comma-separated values with 2 spaces indentation
	dataPrefix := "  " // Two spaces for data rows
	for _, item := range data {
		obj, ok := item.(map[string]interface{})
		if !ok {
			continue
		}

		rowValues := make([]string, len(allKeys))
		for i, k := range allKeys {
			value := ""
			if v, exists := obj[k]; exists {
				// Handle nested structures specially
				switch val := v.(type) {
				case []interface{}:
					if len(val) > 0 {
						if _, isObj := val[0].(map[string]interface{}); isObj {
							// Array of objects: use compact inline tabular format
							nestedKeysMap := make(map[string]bool)
							var nestedKeys []string
							for _, nestedItem := range val {
								if nestedObj, ok := nestedItem.(map[string]interface{}); ok {
									for nk := range nestedObj {
										if !nestedKeysMap[nk] {
											nestedKeysMap[nk] = true
											nestedKeys = append(nestedKeys, nk)
										}
									}
								}
							}
							nestedFields := strings.Join(nestedKeys, ",")
							nestedCount := len(val)
							
							// Build compact data rows separated by semicolons
							var nestedRows []string
							for _, nestedItem := range val {
								if nestedObj, ok := nestedItem.(map[string]interface{}); ok {
									var nestedRowValues []string
									for _, nk := range nestedKeys {
										nv := ""
										if nvVal, exists := nestedObj[nk]; exists {
											nv = valueToToon(nvVal, 0, 0)
											if strings.Contains(nv, ",") || strings.Contains(nv, ";") || strings.Contains(nv, ":") {
												nv = fmt.Sprintf(`"%s"`, nv)
											}
										}
										nestedRowValues = append(nestedRowValues, nv)
									}
									nestedRows = append(nestedRows, strings.Join(nestedRowValues, ","))
								}
							}
							value = fmt.Sprintf("[%d]{%s}:%s", nestedCount, nestedFields, strings.Join(nestedRows, ";"))
						} else {
							// Array of primitives: use bracket notation
							items := make([]string, len(val))
							for j, item := range val {
								items[j] = valueToToon(item, 0, 0)
							}
							value = fmt.Sprintf("[%s]", strings.Join(items, ","))
						}
					} else {
						value = "[]"
					}
				case map[string]interface{}:
					// Nested object: use compact key:value format
					var nestedItems []string
					for nk, nv := range val {
						nvStr := valueToToon(nv, 0, 0)
						if strings.Contains(nvStr, ",") || strings.Contains(nvStr, ":") {
							nvStr = fmt.Sprintf(`"%s"`, nvStr)
						}
						nestedItems = append(nestedItems, fmt.Sprintf("%s:%s", nk, nvStr))
					}
					value = fmt.Sprintf("{%s}", strings.Join(nestedItems, ","))
				default:
					value = valueToToon(v, 0, 0)
					// Handle values with commas, newlines, colons, or semicolons
					// Only quote if not already quoted and contains special chars
					if !(strings.HasPrefix(value, `"`) && strings.HasSuffix(value, `"`)) {
						if strings.Contains(value, ",") || strings.Contains(value, "\n") || strings.Contains(value, ":") || strings.Contains(value, ";") {
							// Escape quotes if present
							if strings.Contains(value, `"`) {
								value = strings.ReplaceAll(value, `"`, `\"`)
							}
							value = fmt.Sprintf(`"%s"`, value)
						}
					}
				}
			}
			rowValues[i] = value
		}
		row := strings.Join(rowValues, ",")
		lines = append(lines, fmt.Sprintf("%s%s", dataPrefix, row))
	}

	return strings.Join(lines, "\n")
}

func valueToToon(value ToonValue, indent int, level int) string {
	if value == nil {
		return "null"
	}

	switch v := value.(type) {
	case bool:
		if v {
			return "true"
		}
		return "false"
	case int, int8, int16, int32, int64, uint, uint8, uint16, uint32, uint64, float32, float64:
		return fmt.Sprintf("%v", v)
	case string:
		return escapeString(v)
	case []interface{}:
		return "\n" + listToToon(v, indent, level)
	case map[string]interface{}:
		return "\n" + dictToToon(v, indent, level)
	default:
		// Try JSON conversion for custom types
		jsonBytes, err := json.Marshal(value)
		if err != nil {
			return fmt.Sprintf("%v", value)
		}
		var converted interface{}
		if err := json.Unmarshal(jsonBytes, &converted); err != nil {
			return fmt.Sprintf("%v", value)
		}
		return valueToToon(converted, indent, level)
	}
}

func escapeString(s string) string {
	// Only escape actual control characters (newlines, tabs, etc.)
	// Let the caller decide if quoting is needed for other special chars
	needsEscaping := false
	for _, char := range s {
		if char == '\n' || char == '\t' || char == '\r' {
			needsEscaping = true
			break
		}
	}

	if !needsEscaping {
		return s
	}

	// Escape control characters
	var builder strings.Builder
	builder.WriteRune('"')
	for _, char := range s {
		switch char {
		case '\\':
			builder.WriteString("\\\\")
		case '"':
			builder.WriteString("\\\"")
		case '\n':
			builder.WriteString("\\n")
		case '\r':
			builder.WriteString("\\r")
		case '\t':
			builder.WriteString("\\t")
		default:
			builder.WriteRune(char)
		}
	}
	builder.WriteRune('"')
	return builder.String()
}

