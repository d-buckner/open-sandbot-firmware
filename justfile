# Test runner for sandbot firmware

# Run host-based integration tests
test-host:
    rustc --test tests/integration_tests.rs && ./integration_tests

# Clean test artifacts
test-clean:
    rm -f integration_tests

# Run embedded build check
test-embedded:
    cargo check

# Run all tests
test-all: test-embedded test-host

# Default recipe
default: test-all