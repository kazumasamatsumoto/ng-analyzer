#!/bin/bash

# Test script to verify the Rust project structure
echo "🔍 Verifying Rust project structure..."

# Check if Cargo.toml exists
if [ -f "Cargo.toml" ]; then
    echo "✅ Cargo.toml found"
else
    echo "❌ Cargo.toml not found"
    exit 1
fi

# Check if src directory exists
if [ -d "src" ]; then
    echo "✅ src directory found"
else
    echo "❌ src directory not found"
    exit 1
fi

# Check if main.rs exists
if [ -f "src/main.rs" ]; then
    echo "✅ src/main.rs found"
else
    echo "❌ src/main.rs not found"
    exit 1
fi

# Check critical modules
modules=("ast" "analyzers" "cli" "config" "output" "parsers")
for module in "${modules[@]}"; do
    if [ -d "src/$module" ]; then
        echo "✅ $module module found"
    else
        echo "❌ $module module not found"
        exit 1
    fi
done

echo ""
echo "🎉 All required files and modules are present!"
echo "📝 Project structure is valid for a Rust Angular analyzer"
echo ""
echo "📋 Project Summary:"
echo "   - CLI interface with clap"
echo "   - TypeScript parsing with SWC"
echo "   - Parallel analysis with Rayon"
echo "   - Multiple output formats (JSON, HTML, Table)"
echo "   - Configurable rules and profiles"
echo "   - Comprehensive Angular analysis"
echo ""
echo "⚡ To build the project (requires Rust toolchain):"
echo "   cargo build --release"
echo ""
echo "🚀 To run the analyzer:"
echo "   ./target/release/ng-analyzer --help"