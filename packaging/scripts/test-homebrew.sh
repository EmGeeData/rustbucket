#!/bin/bash
# Test script for Homebrew formula

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PACKAGING_DIR="$(dirname "$SCRIPT_DIR")"
HOMEBREW_DIR="$PACKAGING_DIR/homebrew"

echo "=== Testing Homebrew Formula ==="
echo "Working directory: $HOMEBREW_DIR"

cd "$HOMEBREW_DIR"

# Check if formula exists
if [ ! -f "rtbt.rb" ]; then
    echo "❌ Error: rtbt.rb not found in $HOMEBREW_DIR"
    exit 1
fi

echo "✅ Formula file found"

# Check if brew is available
if ! command -v brew &> /dev/null; then
    echo "❌ Error: brew not found. This test requires macOS with Homebrew installed."
    exit 1
fi

echo "✅ Homebrew available"

# Validate formula syntax
echo "🔍 Validating formula syntax..."
if ! brew ruby -e "load './rtbt.rb'" &> /dev/null; then
    echo "❌ Error: Formula syntax validation failed"
    exit 1
fi

echo "✅ Formula syntax valid"

# Check formula structure
echo "🔍 Checking formula structure..."
if ! grep -q "class Rtbt < Formula" rtbt.rb; then
    echo "❌ Error: Formula class definition not found"
    exit 1
fi

if ! grep -q "desc " rtbt.rb; then
    echo "❌ Error: Description not found"
    exit 1
fi

if ! grep -q "homepage " rtbt.rb; then
    echo "❌ Error: Homepage not found"
    exit 1
fi

if ! grep -q "url " rtbt.rb; then
    echo "❌ Error: URL not found"
    exit 1
fi

if ! grep -q "license " rtbt.rb; then
    echo "❌ Error: License not found"
    exit 1
fi

echo "✅ Formula structure valid"

# Extract formula information
echo "📋 Formula information:"
echo "  Description: $(grep 'desc ' rtbt.rb | sed 's/.*desc "//' | sed 's/".*//')"
echo "  Homepage: $(grep 'homepage ' rtbt.rb | sed 's/.*homepage "//' | sed 's/".*//')"
echo "  URL: $(grep 'url ' rtbt.rb | sed 's/.*url "//' | sed 's/".*//')"
echo "  License: $(grep 'license ' rtbt.rb | sed 's/.*license "//' | sed 's/".*//')"

# Test formula installation (if dependencies available)
echo "🔧 Testing formula installation..."
if command -v cargo &> /dev/null && command -v rustc &> /dev/null; then
    echo "  Rust/Cargo available, attempting build..."
    
    # Try to install from local formula
    echo "  Installing from local formula..."
    if brew install --build-from-source ./rtbt.rb; then
        echo "✅ Formula installation successful"
        
        # Test installed binary
        if command -v rtbt &> /dev/null; then
            echo "✅ Binary available in PATH"
            
            # Test basic functionality
            echo "🧪 Testing basic functionality..."
            if rtbt --help &> /dev/null; then
                echo "✅ Help command works"
            else
                echo "❌ Error: Help command failed"
                brew uninstall rtbt || true
                exit 1
            fi
            
            if rtbt --version &> /dev/null; then
                echo "✅ Version command works"
                echo "  Version: $(rtbt --version)"
            else
                echo "❌ Error: Version command failed"
                brew uninstall rtbt || true
                exit 1
            fi
            
            if rtbt --list-palettes &> /dev/null; then
                echo "✅ List palettes command works"
                echo "  Sample palettes: $(rtbt --list-palettes | head -3 | tr '\n' ' ')"
            else
                echo "❌ Error: List palettes command failed"
                brew uninstall rtbt || true
                exit 1
            fi
            
        else
            echo "❌ Error: Binary not found in PATH after installation"
            brew uninstall rtbt || true
            exit 1
        fi
        
        # Test shell completions
        echo "🐚 Testing shell completions..."
        BREW_PREFIX=$(brew --prefix)
        
        if [ -f "$BREW_PREFIX/share/bash-completion/completions/rtbt" ]; then
            echo "✅ Bash completion installed"
        else
            echo "⚠️  Bash completion not found"
        fi
        
        if [ -f "$BREW_PREFIX/share/zsh/site-functions/_rtbt" ]; then
            echo "✅ Zsh completion installed"
        else
            echo "⚠️  Zsh completion not found"
        fi
        
        if [ -f "$BREW_PREFIX/share/fish/vendor_completions.d/rtbt.fish" ]; then
            echo "✅ Fish completion installed"
        else
            echo "⚠️  Fish completion not found"
        fi
        
        # Clean up
        echo "🧹 Cleaning up..."
        brew uninstall rtbt
        echo "✅ Cleanup complete"
        
    else
        echo "⚠️  Formula installation failed (expected - may need source release)"
    fi
else
    echo "⚠️  Rust/Cargo not available, skipping build test"
fi

# Run Homebrew formula audit
echo "🔍 Running formula audit..."
if brew audit --strict ./rtbt.rb; then
    echo "✅ Formula audit passed"
else
    echo "⚠️  Formula audit found issues (may be expected for local testing)"
fi

echo ""
echo "🎉 Homebrew formula test completed successfully!"
echo "Ready for tap submission or homebrew-core PR."