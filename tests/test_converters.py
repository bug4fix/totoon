"""
Tests for format converters
"""

import pytest
from totoon.converters import json_to_toon, yaml_to_toon


def test_json_string():
    """Test converting JSON string."""
    json_str = '{"name": "Alice", "age": 30}'
    result = json_to_toon(json_str)
    assert "name: Alice" in result
    assert "age: 30" in result


def test_json_dict():
    """Test converting Python dict (treated as JSON-like)."""
    data = {"name": "Alice", "age": 30}
    result = json_to_toon(data)
    assert "name: Alice" in result
    assert "age: 30" in result


def test_yaml_string():
    """Test converting YAML string."""
    try:
        yaml_str = "name: Alice\nage: 30"
        result = yaml_to_toon(yaml_str)
        assert "name: Alice" in result
        assert "age: 30" in result
    except ImportError:
        pytest.skip("PyYAML not installed")


def test_yaml_dict():
    """Test converting Python dict (treated as YAML-like)."""
    try:
        data = {"name": "Alice", "age": 30}
        result = yaml_to_toon(data)
        assert "name: Alice" in result
        assert "age: 30" in result
    except ImportError:
        pytest.skip("PyYAML not installed")

