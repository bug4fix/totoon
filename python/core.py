"""
Core TOON conversion functions
"""

from typing import Any, Union


def to_toon(data: Any, indent: int = 2) -> str:
    """
    Convert Python data structures to TOON format.
    
    Args:
        data: Python data structure (dict, list, str, int, float, bool, None)
        indent: Number of spaces for indentation (default: 2)
    
    Returns:
        TOON formatted string
    
    Examples:
        >>> to_toon({"name": "Alice", "age": 30})
        'name: Alice\\nage: 30'
        
        >>> to_toon([{"name": "Alice"}, {"name": "Bob"}])
        'name\\nAlice\\nBob'
    """
    if data is None:
        return "null"
    
    if isinstance(data, bool):
        return "true" if data else "false"
    
    if isinstance(data, (int, float)):
        return str(data)
    
    if isinstance(data, str):
        # Escape special characters if needed
        if any(char in data for char in ['\n', '\t', ':', '|']):
            return f'"{data}"'
        return data
    
    if isinstance(data, list):
        return _list_to_toon(data, indent, 0)
    
    if isinstance(data, dict):
        return _dict_to_toon(data, indent, 0)
    
    # Fallback: convert to string
    return str(data)


def from_toon(toon_str: str) -> Any:
    """
    Convert TOON format string to Python data structures.
    
    Args:
        toon_str: TOON formatted string
    
    Returns:
        Python data structure
    
    Note:
        TOON parsing is not yet implemented. This function will be available
        in a future release.
    """
    raise NotImplementedError("TOON parsing is not yet implemented")


def _dict_to_toon(data: dict, indent: int, level: int) -> str:
    """Convert dictionary to TOON format."""
    if not data:
        return "{}"
    
    lines = []
    prefix = " " * (indent * level)
    
    for key, value in data.items():
        key_str = str(key)
        
        if isinstance(value, (dict, list)) and value:
            # Complex nested structure
            if isinstance(value, dict):
                lines.append(f"{prefix}{key_str}:")
                lines.append(_dict_to_toon(value, indent, level + 1))
            else:  # list
                # Check if it's a list of objects (tabular format)
                if value and isinstance(value[0], dict):
                    lines.append(_list_of_objects_to_toon(key_str, value, indent, level))
                else:
                    lines.append(f"{prefix}{key_str}:")
                    lines.append(_list_to_toon(value, indent, level + 1))
        else:
            # Simple value
            value_str = _value_to_toon(value, indent, level + 1)
            lines.append(f"{prefix}{key_str}: {value_str}")
    
    return "\n".join(lines)


def _list_to_toon(data: list, indent: int, level: int) -> str:
    """Convert list to TOON format."""
    if not data:
        return "[]"
    
    # Check if it's a list of objects (use tabular format)
    if data and isinstance(data[0], dict):
        return _list_of_objects_to_toon("", data, indent, level)
    
    # Simple list
    lines = []
    prefix = " " * (indent * level)
    for item in data:
        value_str = _value_to_toon(item, indent, level)
        lines.append(f"{prefix}- {value_str}")
    
    return "\n".join(lines)


def _list_of_objects_to_toon(key: str, data: list, indent: int, level: int) -> str:
    """
    Convert list of objects to TOON tabular format.
    This is TOON's key feature - compact representation of uniform arrays.
    """
    if not data or not isinstance(data[0], dict):
        return _list_to_toon(data, indent, level)
    
    lines = []
    prefix = " " * (indent * level)
    
    # Get all unique keys from all objects, preserving insertion order
    # Use dict to maintain order (Python 3.7+)
    all_keys_dict = {}
    for obj in data:
        for obj_key in obj.keys():
            all_keys_dict[obj_key] = None
    all_keys = list(all_keys_dict.keys())
    
    if not all_keys:
        return "[]"
    
    # Header row (keys)
    if key:
        lines.append(f"{prefix}{key}:")
        prefix = " " * (indent * (level + 1))
    
    header = " | ".join(all_keys)
    lines.append(f"{prefix}{header}")
    
    # Separator
    separator = " | ".join(["---"] * len(all_keys))
    lines.append(f"{prefix}{separator}")
    
    # Data rows
    for obj in data:
        row_values = []
        for k in all_keys:
            value = obj.get(k, "")
            value_str = _value_to_toon(value, 0, 0)
            # Handle values with pipes or newlines
            if "|" in value_str or "\n" in value_str:
                value_str = f'"{value_str}"'
            row_values.append(value_str)
        row = " | ".join(row_values)
        lines.append(f"{prefix}{row}")
    
    return "\n".join(lines)


def _value_to_toon(value: Any, indent: int, level: int) -> str:
    """Convert a single value to TOON string representation."""
    if value is None:
        return "null"
    
    if isinstance(value, bool):
        return "true" if value else "false"
    
    if isinstance(value, (int, float)):
        return str(value)
    
    if isinstance(value, str):
        # Escape if contains special characters
        if any(char in value for char in ['\n', '\t', ':', '|', '"']):
            # Simple escaping - in production, use proper escaping
            escaped = value.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n')
            return f'"{escaped}"'
        return value
    
    if isinstance(value, dict):
        return "\n" + _dict_to_toon(value, indent, level)
    
    if isinstance(value, list):
        return "\n" + _list_to_toon(value, indent, level)
    
    return str(value)

