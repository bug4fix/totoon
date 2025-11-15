"""
Format converters for various input formats to TOON
"""

import json
from typing import Any, Union
from pathlib import Path

try:
    import yaml
except ImportError:
    yaml = None

from totoon.core import to_toon


def json_to_toon(data: Union[str, dict, Path]) -> str:
    """
    Convert JSON data to TOON format.
    
    Args:
        data: JSON string, dict, or path to JSON file
    
    Returns:
        TOON formatted string
    
    Examples:
        >>> json_to_toon('{"name": "Alice", "age": 30}')
        'name: Alice\\nage: 30'
        
        >>> json_to_toon({"name": "Alice", "age": 30})
        'name: Alice\\nage: 30'
    """
    if isinstance(data, Path):
        with open(data, 'r', encoding='utf-8') as f:
            data = f.read()
    
    if isinstance(data, str):
        data = json.loads(data)
    
    return to_toon(data)


def yaml_to_toon(data: Union[str, dict, Path]) -> str:
    """
    Convert YAML data to TOON format.
    
    Args:
        data: YAML string, dict, or path to YAML file
    
    Returns:
        TOON formatted string
    
    Raises:
        ImportError: If PyYAML is not installed
    """
    if yaml is None:
        raise ImportError(
            "PyYAML is required for YAML conversion. "
            "Install it with: pip install pyyaml"
        )
    
    if isinstance(data, Path):
        with open(data, 'r', encoding='utf-8') as f:
            data = f.read()
    
    if isinstance(data, str):
        data = yaml.safe_load(data)
    
    return to_toon(data)


def xml_to_toon(data: Union[str, Path]) -> str:
    """
    Convert XML data to TOON format.
    
    Args:
        data: XML string or path to XML file
    
    Returns:
        TOON formatted string
    
    Note:
        This is a basic implementation. For complex XML structures,
        consider using xmltodict or similar libraries.
    """
    try:
        import xml.etree.ElementTree as ET
    except ImportError:
        raise ImportError("XML support requires Python's built-in xml module")
    
    if isinstance(data, Path):
        with open(data, 'r', encoding='utf-8') as f:
            data = f.read()
    
    # Convert XML to dict
    root = ET.fromstring(data)
    data_dict = _xml_element_to_dict(root)
    
    return to_toon(data_dict)


def _xml_element_to_dict(element) -> dict:
    """Convert XML element to dictionary."""
    result = {}
    
    # Add attributes
    if element.attrib:
        result.update(element.attrib)
    
    # Add children
    children = {}
    for child in element:
        child_dict = _xml_element_to_dict(child)
        child_tag = child.tag
        
        # Handle multiple children with same tag
        if child_tag in children:
            if not isinstance(children[child_tag], list):
                children[child_tag] = [children[child_tag]]
            children[child_tag].append(child_dict)
        else:
            children[child_tag] = child_dict
    
    result.update(children)
    
    # Add text content if present and no children
    if element.text and element.text.strip() and not children:
        if result:
            result['_text'] = element.text.strip()
        else:
            return element.text.strip()
    
    return result if result else None

