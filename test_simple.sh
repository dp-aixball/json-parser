#!/bin/bash

echo "=== Testing JSON Parser ==="
echo ""

echo "1. Running default json_parser (detailed output):"
cargo run
echo ""

echo "2. Running simple_json_parser (compact output):"
cargo run --bin simple_json_parser
echo ""

echo "3. Running simple_json_parser with test.json:"
cargo run --bin simple_json_parser -- test.json
echo ""

echo "4. Building both binaries:"
cargo build --release
echo ""

echo "5. Running built binaries directly:"
echo "   Detailed version:"
./target/release/json_parser
echo ""
echo "   Simple version:"
./target/release/simple_json_parser
echo ""
echo "   Simple version with test.json:"
./target/release/simple_json_parser test.json