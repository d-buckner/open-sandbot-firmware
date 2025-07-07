#!/bin/bash
set -e

echo "🧪 Running sandbot tests..."

echo "📦 Checking embedded build..."
cargo check

echo "🖥️  Running host-based integration tests..."
rustc --test tests/integration_tests.rs && ./integration_tests

echo "🧹 Cleaning up..."
rm -f integration_tests

echo "✅ All tests passed!"