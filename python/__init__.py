"""
totoon - Convert any data format to TOON (Token-Oriented Object Notation)
"""

from totoon.core import to_toon, from_toon
from totoon.converters import json_to_toon, yaml_to_toon, xml_to_toon

__version__ = "0.1.1"
__all__ = [
    "to_toon",
    "from_toon",
    "json_to_toon",
    "yaml_to_toon",
    "xml_to_toon",
]

