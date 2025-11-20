#!/usr/bin/env python3
import json
import os
import re
import sys
from typing import Any, Dict, List, Union

def sanitize_value(key: str, value: Any) -> Any:
    """Sanitize a single value based on key name or value pattern."""
    if not isinstance(value, str):
        return value

    # Sanitize API keys based on key name
    if "API_KEY" in key.upper() or "TOKEN" in key.upper() or "SECRET" in key.upper():
        return f"YOUR_{key.upper()}_HERE"
    
    # Sanitize specific keys from original script
    if key == "tavilyApiKey":
        return "YOUR_TAVILY_API_KEY_HERE"

    # Sanitize local paths
    if key in ["cwd", "MEMORY_FILE_PATH"] and (value.startswith("/Volumes/") or value.startswith("/Users/")):
        if key == "cwd":
            return "/path/to/your/mcp/servers"
        if key == "MEMORY_FILE_PATH":
            return "/path/to/your/memory/memories.jsonl"
            
    # Sanitize specific patterns in values
    # Brave API Key pattern
    if re.search(r"BSA[a-zA-Z0-9]{27}", value):
        return "YOUR_BRAVE_API_KEY_HERE"
    
    # Tavily API Key pattern
    if re.search(r"tvly-[a-zA-Z0-9-]{30,}", value):
        return "YOUR_TAVILY_API_KEY_HERE"

    return value

def sanitize_structure(data: Any, key_context: str = "") -> Any:
    """Recursively sanitize a JSON structure."""
    if isinstance(data, dict):
        return {k: sanitize_structure(v, k) for k, v in data.items()}
    elif isinstance(data, list):
        return [sanitize_structure(item, key_context) for item in data]
    else:
        return sanitize_value(key_context, data)

def process_file(file_path: str) -> bool:
    """Process a single JSON file."""
    try:
        with open(file_path, 'r') as f:
            data = json.load(f)
        
        sanitized_data = sanitize_structure(data)
        
        # Check if changed
        if data == sanitized_data:
            return False
            
        with open(file_path, 'w') as f:
            json.dump(sanitized_data, f, indent=2)
            # Add newline at end of file
            f.write('\n')
            
        return True
    except Exception as e:
        print(f"Error processing {file_path}: {e}", file=sys.stderr)
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: sanitize.py <file1> [file2 ...]")
        sys.exit(1)

    files = sys.argv[1:]
    changed_files = []

    for file_path in files:
        if process_file(file_path):
            changed_files.append(file_path)
            print(f"Sanitized: {file_path}")

    if changed_files:
        print(f"Modified {len(changed_files)} files.")
    else:
        print("No files needed sanitization.")

if __name__ == "__main__":
    main()
