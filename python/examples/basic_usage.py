#!/usr/bin/env python3
"""
Basic usage examples for totoon
"""

import sys
from pathlib import Path

# Add parent directory to path for development
sys.path.insert(0, str(Path(__file__).parent.parent))

from totoon import to_toon, json_to_toon

# Example 1: Simple dictionary
print("=== Example 1: Simple Dictionary ===")
data = {"name": "Alice", "age": 30, "city": "New York"}
print(to_toon(data))
print()

# Example 2: List of objects (tabular format - TOON's strength)
print("=== Example 2: List of Objects (Tabular Format) ===")
users = [
    {"name": "Alice", "age": 30, "active": True},
    {"name": "Bob", "age": 25, "active": False},
    {"name": "Charlie", "age": 35, "active": True}
]
print(to_toon(users))
print()

# Example 3: Nested structure
print("=== Example 3: Nested Structure ===")
complex_data = {
    "users": [
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25}
    ],
    "metadata": {
        "count": 2,
        "timestamp": "2024-01-01"
    }
}
print(to_toon(complex_data))
print()

# Example 4: Convert from JSON string
print("=== Example 4: Convert from JSON String ===")
json_str = '{"products": [{"name": "Widget", "price": 9.99}, {"name": "Gadget", "price": 19.99}]}'
print(json_to_toon(json_str))
print()

# Example 5: LLM-friendly data structure
print("=== Example 5: LLM-Friendly Data Structure ===")
llm_data = {
    "messages": [
        {"role": "user", "content": "Hello"},
        {"role": "assistant", "content": "Hi there!"},
        {"role": "user", "content": "How are you?"}
    ]
}
print(to_toon(llm_data))
print()

