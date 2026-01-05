# no_std Tests

This directory contains tests for the `oci-spec` crate in no_std environments.

## Setup and Build

```bash
cd tests-no-std

# Check compilation (verify it compiles in no_std environment)
cargo check --bin test_minimal

# Build
cargo build --bin test_minimal
```

**Note**: Warnings about `static mut` usage are common in no_std environments.

## Test Stages

Tests are added incrementally to identify at which stage issues occur:

1. `test_empty_json()`: Empty JSON object `{}`
   - Verifies basic deserialization works

2. `test_minimal_version()`: Version field only `{"ociVersion": "1.0.2-dev"}`
   - Verifies explicit value deserialization works

3. `test_version_only()`: Empty JSON (serde(default) applies)
   - Verifies `serde(default)` behavior

4. `test_version_and_process()`: Version and process fields
   - Verifies `Process` struct deserialization

5. `test_with_root()`: Adds root field
   - Verifies `Root` struct deserialization

## Notes

These tests verify that the code compiles in `#![no_std]` environments.
Actual execution requires proper allocator and runtime configuration.
