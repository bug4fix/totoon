"""
Core TOON conversion functions
"""

from typing import Any


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
    Format: key[count]{field1,field2,field3}:
              value1,value2,value3
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
    
    # Header format: key[count]{field1,field2,field3}:
    if key:
        count = len(data)
        fields = ",".join(all_keys)
        header = f"{prefix}{key}[{count}]{{{fields}}}:"
        lines.append(header)
    else:
        count = len(data)
        fields = ",".join(all_keys)
        header = f"{prefix}[{count}]{{{fields}}}:"
        lines.append(header)
    
    # Data rows: comma-separated values with 2 spaces indentation
    data_prefix = "  "  # Two spaces for data rows
    for obj in data:
        row_values = []
        for k in all_keys:
            value = obj.get(k, "")
            # Handle nested structures specially
            if isinstance(value, list):
                if value and isinstance(value[0], dict):
                    # Array of objects: use compact inline tabular format
                    # Get all unique keys from nested objects
                    nested_keys_dict = {}
                    for nested_obj in value:
                        for nk in nested_obj.keys():
                            nested_keys_dict[nk] = None
                    nested_keys = list(nested_keys_dict.keys())
                    nested_fields = ",".join(nested_keys)
                    nested_count = len(value)
                    
                    # Build compact data rows separated by semicolons
                    nested_rows = []
                    for nested_obj in value:
                        nested_row_values = []
                        for nk in nested_keys:
                            nv = nested_obj.get(nk, "")
                            nv_str = _value_to_toon(nv, 0, 0)
                            # Escape if contains special chars
                            if "," in nv_str or ";" in nv_str or ":" in nv_str:
                                nv_str = f'"{nv_str}"'
                            nested_row_values.append(nv_str)
                        nested_rows.append(",".join(nested_row_values))
                    
                    value_str = f"[{nested_count}]{{{nested_fields}}}:{';'.join(nested_rows)}"
                else:
                    # Array of primitives: use bracket notation
                    items = [_value_to_toon(item, 0, 0) for item in value]
                    value_str = f"[{','.join(items)}]"
            elif isinstance(value, dict):
                # Nested object: use compact key:value format
                nested_items = []
                for nk, nv in value.items():
                    nv_str = _value_to_toon(nv, 0, 0)
                    # Escape if contains special chars
                    if "," in nv_str or ":" in nv_str:
                        nv_str = f'"{nv_str}"'
                    nested_items.append(f"{nk}:{nv_str}")
                value_str = f"{{{','.join(nested_items)}}}"
            else:
                value_str = _value_to_toon(value, 0, 0)
                # Handle values with commas, newlines, colons, or semicolons
                # Only quote if not already quoted and contains special chars
                if not (value_str.startswith('"') and value_str.endswith('"')):
                    if "," in value_str or "\n" in value_str or ":" in value_str or ";" in value_str:
                        # Escape quotes if present
                        if '"' in value_str:
                            value_str = value_str.replace('"', '\\"')
                        value_str = f'"{value_str}"'
            row_values.append(value_str)
        row = ",".join(row_values)
        lines.append(f"{data_prefix}{row}")
    
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
        # Don't escape here - let the caller decide if quoting is needed
        # Only escape actual control characters
        if '\n' in value or '\t' in value or '\r' in value:
            escaped = value.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n').replace('\r', '\\r').replace('\t', '\\t')
            return f'"{escaped}"'
        return value
    
    if isinstance(value, dict):
        return "\n" + _dict_to_toon(value, indent, level)
    
    if isinstance(value, list):
        return "\n" + _list_to_toon(value, indent, level)
    
    return str(value)

