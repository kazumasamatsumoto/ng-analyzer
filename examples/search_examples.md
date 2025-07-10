# Search Feature Examples

This document demonstrates how to use the new search functionality in the ng-analyzer tool.

## Basic Search Commands

### 1. Search for keywords in HTML files
```bash
ng-analyzer search -k "component" -f html -p ./src --line-numbers
```
This searches for the word "component" in all HTML files within the ./src directory and shows line numbers.

### 2. Search for keywords in TypeScript files
```bash
ng-analyzer search -k "function" -f ts -p ./src --line-numbers
```
This searches for the word "function" in all TypeScript files.

### 3. Search for keywords in JavaScript files
```bash
ng-analyzer search -k "async" -f js -p ./src --line-numbers
```
This searches for the word "async" in all JavaScript files.

### 4. Search in all supported file types
```bash
ng-analyzer search -k "import" -f all -p ./src --line-numbers
```
This searches for "import" in all supported file types (HTML, TS, JS, CSS, SCSS, JSON, MD, TXT, RS).

## Advanced Search Options

### 5. Case-sensitive search
```bash
ng-analyzer search -k "Component" --case-sensitive -p ./src --line-numbers
```
This performs a case-sensitive search for the exact word "Component".

### 6. Search with context lines
```bash
ng-analyzer search -k "export" -c 2 -p ./src --line-numbers
```
This searches for "export" and shows 2 lines of context before and after each match.

### 7. Search in specific file patterns
```bash
ng-analyzer search -k "service" --file-pattern "component" -p ./src --line-numbers
```
This searches for "service" only in files that have "component" in their filename.

### 8. Table output format
```bash
ng-analyzer search -k "interface" -f ts -p ./src --output table
```
This searches for "interface" in TypeScript files and displays results in a table format.

## Folder-specific Search

### 9. Search in specific folder
```bash
ng-analyzer search -k "ngOnInit" -p ./src/app/components --line-numbers
```
This searches for "ngOnInit" only in the components folder.

### 10. Search in nested folders
```bash
ng-analyzer search -k "Observable" -p ./src/app/services -c 1 --line-numbers
```
This searches for "Observable" in the services folder with 1 line of context.

## Output Options

### Simple format (default)
```bash
ng-analyzer search -k "constructor" -f ts -p ./src
```
Shows results in a simple, readable format.

### Table format
```bash
ng-analyzer search -k "constructor" -f ts -p ./src --output table
```
Shows results in a structured table format.

## Practical Use Cases

### Find all component declarations
```bash
ng-analyzer search -k "@Component" -f ts -p ./src --line-numbers
```

### Find all service injections
```bash
ng-analyzer search -k "inject" -f ts -p ./src --case-sensitive --line-numbers
```

### Find all HTML template references
```bash
ng-analyzer search -k "templateUrl" -f ts -p ./src --line-numbers
```

### Find all CSS class usages
```bash
ng-analyzer search -k "class=" -f html -p ./src --line-numbers
```

### Find all routing configurations
```bash
ng-analyzer search -k "Routes" -f ts -p ./src --line-numbers
```

## Tips

1. Use `--case-sensitive` for exact matches
2. Use `-c` or `--context` to see surrounding code
3. Use `--file-pattern` to narrow down file selection
4. Use `--output table` for structured output
5. Use different file types (`-f`) to focus your search
6. Use specific paths (`-p`) to search in particular directories

## Command Reference

```
ng-analyzer search [OPTIONS]

Options:
  -k, --keyword <KEYWORD>       Keyword to search for [required]
  -p, --path <PATH>             Path to search in [default: ./src]
  -f, --file-type <TYPE>        File types: html, ts, js, all [default: all]
  --file-pattern <PATTERN>      Specific file pattern to search in
  --case-sensitive              Case sensitive search
  --line-numbers                Show line numbers
  -c, --context <NUM>           Context lines around matches [default: 0]
  -o, --output <FORMAT>         Output format: simple, table [default: simple]
  -v, --verbose                 Verbose output
  -q, --quiet                   Quiet mode
```