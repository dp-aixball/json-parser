# Enhanced JSON Parser

A powerful command-line JSON parser with filtering, formatting, and extraction capabilities.

## Features

- **Colorful output** with syntax highlighting
- **Filtering** by key prefix
- **Extraction** of specific values using dot notation
- **Multiple output formats** (Pretty, Simple, CSV, Markdown)
- **Statistics** about JSON structure
- **Depth limiting** for large nested structures
- **Leaf-only mode** to show only terminal values
- **Key/Value only modes** for specific use cases

## Installation

```bash
cargo build --bin json_parser_enhanced
```

## Usage

### Basic Usage
```bash
# Parse default file (data.json)
./target/debug/json_parser_enhanced

# Parse specific file
./target/debug/json_parser_enhanced complex.json
```

### Filtering
```bash
# Show only keys starting with "address"
./target/debug/json_parser_enhanced --filter address

# Show only metadata keys
./target/debug/json_parser_enhanced --filter metadata
```

### Extraction
```bash
# Extract specific value
./target/debug/json_parser_enhanced --extract address.city

# Extract array element
./target/debug/json_parser_enhanced --extract hobbies[1]

# Extract with JSON output
./target/debug/json_parser_enhanced --extract metadata.tags --json-output
```

### Output Formats
```bash
# Pretty colored output (default)
./target/debug/json_parser_enhanced --format pretty

# Simple plain text
./target/debug/json_parser_enhanced --format simple

# CSV format
./target/debug/json_parser_enhanced --format csv

# Markdown table
./target/debug/json_parser_enhanced --format markdown
```

### View Modes
```bash
# Show only leaf values (no nested objects)
./target/debug/json_parser_enhanced --leaves-only

# Show only keys
./target/debug/json_parser_enhanced --keys-only

# Show only values
./target/debug/json_parser_enhanced --values-only

# Limit nesting depth
./target/debug/json_parser_enhanced --max-depth 2
```

### Statistics
```bash
# Show JSON structure statistics
./target/debug/json_parser_enhanced --stats
```

## Examples

### Example 1: Filter and extract
```bash
# Get all user emails from complex.json
./target/debug/json_parser_enhanced complex.json --filter users --leaves-only | grep email
```

### Example 2: Create CSV of user data
```bash
# Create CSV of user profiles
./target/debug/json_parser_enhanced complex.json --filter users --leaves-only --format csv > users.csv
```

### Example 3: Quick value lookup
```bash
# Quickly check a specific value
./target/debug/json_parser_enhanced complex.json --extract users[0].name
# Output: "Alice"
```

### Example 4: JSON structure analysis
```bash
# Analyze JSON structure with statistics
./target/debug/json_parser_enhanced complex.json --stats --max-depth 3
```

## Available Binaries

This project includes three binaries:

1. **json_parser** - Original version (src/main.rs)
2. **simple_json_parser** - Simple version (src/main_simple.rs)
3. **json_parser_enhanced** - Enhanced version with all features (src/main_enhanced.rs)

## Command Line Options

```
USAGE:
    json_parser_enhanced [OPTIONS] [FILE]

ARGS:
    <FILE>    JSON file to parse (default: data.json) [default: data.json]

OPTIONS:
    -f, --format <FORMAT>        Output format [default: pretty] [possible values: pretty, simple, csv, markdown]
        --filter <FILTER>        Filter keys by prefix (e.g., "address.")
    -l, --leaves-only            Show only leaf values (no nested objects/arrays)
    -k, --keys-only              Show only keys (no values)
    -v, --values-only            Show only values (no keys)
    -d, --max-depth <MAX_DEPTH>  Maximum depth for nested structures (0 = unlimited) [default: 0]
    -e, --extract <EXTRACT>      Extract specific key path (e.g., "address.city")
    -s, --stats                  Show statistics about the JSON structure
    -j, --json-output            Output in JSON format (for extracted values)
    -h, --help                   Print help (see a summary with '-h')
    -V, --version                Print version
```

## Sample JSON Files

The project includes sample JSON files:

1. **data.json** - Simple example with nested objects and arrays
2. **complex.json** - Complex example with user data and nested structures

## Testing

Run the test script to see all features in action:
```bash
chmod +x test_enhanced.sh
./test_enhanced.sh
```