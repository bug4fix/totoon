"""
Tests for core TOON conversion functionality
"""

from totoon.core import to_toon


def test_simple_dict():
    """Test converting a simple dictionary."""
    data = {"name": "Alice", "age": 30}
    result = to_toon(data)
    assert "name: Alice" in result
    assert "age: 30" in result


def test_nested_dict():
    """Test converting nested dictionaries."""
    data = {
        "user": {
            "name": "Alice",
            "details": {
                "age": 30,
                "city": "NYC"
            }
        }
    }
    result = to_toon(data)
    assert "user:" in result
    assert "name: Alice" in result
    assert "details:" in result
    assert "age: 30" in result


def test_list_of_objects():
    """Test converting list of objects (tabular format)."""
    data = [
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25}
    ]
    result = to_toon(data)
    # Should use tabular format: [count]{field1,field2}:
    assert "[2]{" in result or "[2]{name,age}:" in result
    assert "Alice" in result
    assert "Bob" in result
    assert "," in result  # Comma-separated values


def test_simple_list():
    """Test converting simple list."""
    data = [1, 2, 3]
    result = to_toon(data)
    assert "- 1" in result
    assert "- 2" in result
    assert "- 3" in result


def test_primitives():
    """Test converting primitive values."""
    assert to_toon(None) == "null"
    assert to_toon(True) == "true"
    assert to_toon(False) == "false"
    assert to_toon(42) == "42"
    assert to_toon(3.14) == "3.14"
    assert to_toon("hello") == "hello"


def test_string_escaping():
    """Test string escaping for special characters."""
    data = {"message": "Hello\nWorld"}
    result = to_toon(data)
    assert '"' in result  # Should be quoted


def test_complex_structure():
    """Test converting complex nested structure."""
    data = {
        "users": [
            {"name": "Alice", "age": 30, "active": True},
            {"name": "Bob", "age": 25, "active": False}
        ],
        "metadata": {
            "count": 2,
            "timestamp": "2024-01-01"
        }
    }
    result = to_toon(data)
    assert "users[" in result  # Should have users[count]{fields}:
    assert "metadata:" in result
    assert "count: 2" in result
    assert "," in result  # Comma-separated values

