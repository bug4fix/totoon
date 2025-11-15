package totoon

import (
	"strings"
	"testing"
)

func TestToToon_SimpleObject(t *testing.T) {
	data := map[string]interface{}{
		"name": "Alice",
		"age":  30,
	}
	result := ToToon(data)
	if !strings.Contains(result, "name: Alice") {
		t.Errorf("Expected 'name: Alice' in result, got: %s", result)
	}
	if !strings.Contains(result, "age: 30") {
		t.Errorf("Expected 'age: 30' in result, got: %s", result)
	}
}

func TestToToon_NestedObject(t *testing.T) {
	data := map[string]interface{}{
		"user": map[string]interface{}{
			"name": "Alice",
			"details": map[string]interface{}{
				"age":  30,
				"city": "NYC",
			},
		},
	}
	result := ToToon(data)
	if !strings.Contains(result, "user:") {
		t.Errorf("Expected 'user:' in result, got: %s", result)
	}
	if !strings.Contains(result, "name: Alice") {
		t.Errorf("Expected 'name: Alice' in result, got: %s", result)
	}
	if !strings.Contains(result, "details:") {
		t.Errorf("Expected 'details:' in result, got: %s", result)
	}
	if !strings.Contains(result, "age: 30") {
		t.Errorf("Expected 'age: 30' in result, got: %s", result)
	}
}

func TestToToon_ListOfObjects(t *testing.T) {
	data := []interface{}{
		map[string]interface{}{"name": "Alice", "age": 30},
		map[string]interface{}{"name": "Bob", "age": 25},
	}
	result := ToToon(data)
	if !strings.Contains(result, "name | age") {
		t.Errorf("Expected 'name | age' in result, got: %s", result)
	}
	if !strings.Contains(result, "Alice") {
		t.Errorf("Expected 'Alice' in result, got: %s", result)
	}
	if !strings.Contains(result, "Bob") {
		t.Errorf("Expected 'Bob' in result, got: %s", result)
	}
}

func TestToToon_SimpleList(t *testing.T) {
	data := []interface{}{1, 2, 3}
	result := ToToon(data)
	if !strings.Contains(result, "- 1") {
		t.Errorf("Expected '- 1' in result, got: %s", result)
	}
	if !strings.Contains(result, "- 2") {
		t.Errorf("Expected '- 2' in result, got: %s", result)
	}
	if !strings.Contains(result, "- 3") {
		t.Errorf("Expected '- 3' in result, got: %s", result)
	}
}

func TestToToon_Primitives(t *testing.T) {
	if ToToon(nil) != "null" {
		t.Errorf("Expected 'null', got: %s", ToToon(nil))
	}
	if ToToon(true) != "true" {
		t.Errorf("Expected 'true', got: %s", ToToon(true))
	}
	if ToToon(false) != "false" {
		t.Errorf("Expected 'false', got: %s", ToToon(false))
	}
	if ToToon(42) != "42" {
		t.Errorf("Expected '42', got: %s", ToToon(42))
	}
	if ToToon(3.14) != "3.14" {
		t.Errorf("Expected '3.14', got: %s", ToToon(3.14))
	}
	if ToToon("hello") != "hello" {
		t.Errorf("Expected 'hello', got: %s", ToToon("hello"))
	}
}

func TestToToon_StringEscaping(t *testing.T) {
	data := map[string]interface{}{
		"message": "Hello\nWorld",
	}
	result := ToToon(data)
	if !strings.Contains(result, "\"") {
		t.Errorf("Expected quoted string in result, got: %s", result)
	}
	if !strings.Contains(result, "Hello") {
		t.Errorf("Expected 'Hello' in result, got: %s", result)
	}
}

func TestToToon_ComplexStructure(t *testing.T) {
	data := map[string]interface{}{
		"users": []interface{}{
			map[string]interface{}{"name": "Alice", "age": 30, "active": true},
			map[string]interface{}{"name": "Bob", "age": 25, "active": false},
		},
		"metadata": map[string]interface{}{
			"count":     2,
			"timestamp": "2024-01-01",
		},
	}
	result := ToToon(data)
	if !strings.Contains(result, "users:") {
		t.Errorf("Expected 'users:' in result, got: %s", result)
	}
	if !strings.Contains(result, "name | age | active") {
		t.Errorf("Expected 'name | age | active' in result, got: %s", result)
	}
	if !strings.Contains(result, "metadata:") {
		t.Errorf("Expected 'metadata:' in result, got: %s", result)
	}
	if !strings.Contains(result, "count: 2") {
		t.Errorf("Expected 'count: 2' in result, got: %s", result)
	}
}

func TestToToon_EmptyObject(t *testing.T) {
	data := map[string]interface{}{}
	result := ToToon(data)
	if result != "{}" {
		t.Errorf("Expected '{}', got: %s", result)
	}
}

func TestToToon_EmptyArray(t *testing.T) {
	data := []interface{}{}
	result := ToToon(data)
	if result != "[]" {
		t.Errorf("Expected '[]', got: %s", result)
	}
}

func TestJSONToToon(t *testing.T) {
	jsonStr := `{"name": "Alice", "age": 30}`
	result, err := JSONToToon(jsonStr)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if !strings.Contains(result, "name: Alice") {
		t.Errorf("Expected 'name: Alice' in result, got: %s", result)
	}
	if !strings.Contains(result, "age: 30") {
		t.Errorf("Expected 'age: 30' in result, got: %s", result)
	}
}

func TestJSONToToon_Complex(t *testing.T) {
	jsonStr := `{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}`
	result, err := JSONToToon(jsonStr)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if !strings.Contains(result, "users:") {
		t.Errorf("Expected 'users:' in result, got: %s", result)
	}
	if !strings.Contains(result, "name | age") {
		t.Errorf("Expected 'name | age' in result, got: %s", result)
	}
}

