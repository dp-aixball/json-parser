#!/bin/bash

echo "=== Testing Enhanced JSON Parser ==="
echo ""

echo "1. Default output (colored):"
./target/debug/json_parser_enhanced --filter address
echo ""

echo "2. Filter by prefix 'metadata':"
./target/debug/json_parser_enhanced --filter metadata
echo ""

echo "3. Leaves only (no nested objects):"
./target/debug/json_parser_enhanced --leaves-only --filter metadata
echo ""

echo "4. Extract specific value:"
./target/debug/json_parser_enhanced --extract address.city
echo ""

echo "5. Extract array element:"
./target/debug/json_parser_enhanced --extract hobbies[1]
echo ""

echo "6. JSON output for extracted value:"
./target/debug/json_parser_enhanced --extract metadata.tags --json-output
echo ""

echo "7. Show statistics:"
./target/debug/json_parser_enhanced --stats --filter address
echo ""

echo "8. Keys only:"
./target/debug/json_parser_enhanced --keys-only --filter address
echo ""

echo "9. Values only:"
./target/debug/json_parser_enhanced --values-only --filter address
echo ""

echo "10. Max depth 1:"
./target/debug/json_parser_enhanced --max-depth 1
echo ""

echo "11. Simple format:"
./target/debug/json_parser_enhanced --format simple --filter address
echo ""

echo "=== All tests completed ==="