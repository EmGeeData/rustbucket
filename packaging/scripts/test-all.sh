#!/bin/bash
# Master test script for all packaging formats

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🚀 RustBucket Packaging Test Suite"
echo "=================================="
echo ""

# Track test results
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0
FAILED_TESTS=()

run_test() {
    local test_name="$1"
    local test_script="$2"
    
    echo "🧪 Running $test_name test..."
    echo "----------------------------------------"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if [ -f "$SCRIPT_DIR/$test_script" ]; then
        if bash "$SCRIPT_DIR/$test_script"; then
            echo "✅ $test_name test PASSED"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            echo "❌ $test_name test FAILED"
            TESTS_FAILED=$((TESTS_FAILED + 1))
            FAILED_TESTS+=("$test_name")
        fi
    else
        echo "❌ $test_name test script not found: $test_script"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        FAILED_TESTS+=("$test_name (script missing)")
    fi
    
    echo ""
}

# Run all packaging tests
echo "Starting comprehensive packaging tests..."
echo ""

run_test "Arch Linux" "test-arch.sh"
run_test "Homebrew" "test-homebrew.sh"

# Summary
echo "🏁 Test Suite Summary"
echo "====================="
echo "Tests run: $TESTS_RUN"
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $TESTS_FAILED"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo "🎉 All tests passed! Packaging is ready for distribution."
    echo ""
    echo "Next steps:"
    echo "1. Update checksums/hashes for real releases"
    echo "2. Submit to package repositories"
    echo "3. Test on target platforms"
else
    echo "⚠️  Some tests failed:"
    for failed_test in "${FAILED_TESTS[@]}"; do
        echo "  - $failed_test"
    done
    echo ""
    echo "Please fix the failed tests before proceeding with distribution."
    exit 1
fi