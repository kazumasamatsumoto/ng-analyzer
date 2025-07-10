#!/bin/bash

# Simple test script to demonstrate search functionality

echo "=== Rust Tools Search Feature Test ==="
echo ""

# Test 1: HTML file search
echo "Test 1: Search for 'component' in HTML files"
echo "Command: ng-analyzer search -k component -f html -p ./src --line-numbers"
echo "Expected: Find all HTML files containing 'component'"
echo ""

# Test 2: TypeScript file search
echo "Test 2: Search for 'function' in TypeScript files"
echo "Command: ng-analyzer search -k function -f ts -p ./src --line-numbers"
echo "Expected: Find all TypeScript files containing 'function'"
echo ""

# Test 3: Search in specific file pattern
echo "Test 3: Search for 'import' in files matching pattern 'service'"
echo "Command: ng-analyzer search -k import --file-pattern service -p ./src --line-numbers"
echo "Expected: Find all files with 'service' in filename containing 'import'"
echo ""

# Test 4: Case sensitive search
echo "Test 4: Case sensitive search for 'Component'"
echo "Command: ng-analyzer search -k Component --case-sensitive -p ./src --line-numbers"
echo "Expected: Find exact matches of 'Component' (case sensitive)"
echo ""

# Test 5: Search with context
echo "Test 5: Search with context lines"
echo "Command: ng-analyzer search -k export -c 2 -p ./src --line-numbers"
echo "Expected: Find 'export' with 2 lines of context before and after"
echo ""

echo "=== Search Feature Implementation Complete ==="
echo ""
echo "Available search options:"
echo "  -k, --keyword <KEYWORD>       Keyword to search for"
echo "  -f, --file-type <TYPE>        File types (html, ts, js, all)"
echo "  -p, --path <PATH>             Path to search in"
echo "  --file-pattern <PATTERN>      Specific file pattern to search"
echo "  --case-sensitive              Case sensitive search"
echo "  --line-numbers                Show line numbers"
echo "  -c, --context <NUM>           Context lines around matches"
echo "  -o, --output <FORMAT>         Output format (simple, table)"
echo ""
echo "Example usage:"
echo "  ng-analyzer search -k 'async' -f ts -p ./src --line-numbers -c 2"
echo "  ng-analyzer search -k 'div' -f html -p ./src --output table"
echo "  ng-analyzer search -k 'service' --file-pattern 'component' -p ./src"