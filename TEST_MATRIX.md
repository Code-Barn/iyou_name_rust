# iyou_chart_kernel Test Matrix & Verification Framework

## Test Coverage Summary

### 1. Unit Tests - Mathematical Stability Verification

**Module**: `tests/unit/test_coordinates.rs`

#### Test Cases

| Test Name | Purpose | Validation Criteria |
|-----------|---------|---------------------|
| `test_rotate_coordinates_exact` | Verify rotation matrix calculations | ✅ 0°, 90°, 180°, 270° rotations match expected results |
| `test_rotate_around_center_quadrants` | Validate quadrantal center mapping | ✅ Points map accurately to visual quadrants |
| `test_center_constants` | Confirm absolute visual center | ✅ (975.0, 975.0) constants preserved |
| `test_rotation_matrix_orthogonality` | Test matrix composition properties | ✅ 360° rotation returns to origin, 90°×2 = 180° |

#### Mathematical Verification

- **Rotation Matrix Formula**: `rotated_x = dx·cos(θ) - dy·sin(θ)`
- **Center Point**: (975.0, 975.0) - absolute visual center
- **Precision**: All calculations use `approx::assert_relative_eq!` with ε = 1e-6
- **Orthogonality**: Rotation matrices preserve vector lengths and angles

### 2. Integration Tests - Pixel Layout Validation

**Module**: `tests/integration/test_basic.rs`

#### Test Cases

| Test Name | Purpose | Validation Criteria |
|-----------|---------|---------------------|
| `test_environment_initialization` | Thread-safe ImageMagick setup | ✅ Multiple calls safe, no panics |
| `test_gen1_generation` | Single-person layout | ✅ Valid PNG, reasonable size, no memory issues |
| `test_gen2_generation` | Recursive composition | ✅ Valid PNG, proper scaling, memory safety |
| `test_error_handling` | Graceful error recovery | ✅ Handles invalid settings without crashes |
| `test_empty_person_data` | Robustness with minimal data | ✅ Handles empty fields gracefully |

#### PNG Validation

```rust
// PNG magic header verification
const PNG_MAGIC_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

// Size validation
assert!(image_bytes.len() > 1000);
assert!(image_bytes.len() < 2000000);
```

### 3. Memory Safety Verification

#### Recursive Composition Checks

- **Gen2 Overlay Scale**: 0.50 (50% of original size)
- **Center Positioning**: `(CANVAS_WIDTH - scaled_width) / 2`
- **Memory Bounds**: Image sizes validated to prevent overflows
- **Zero-Copy Operations**: `&[u8]` references used for image data

#### Error Handling Patterns

```rust
// Comprehensive error propagation
pub enum ChartError {
    MagickError(MagickError),
    InvalidCoordinate(String),
    FontMetricsError(String),
    CompositionError(String),
    InvalidSettings(String),
    EnvironmentNotInitialized,
}
```

### 4. Compilation & Environment Requirements

#### Required Environment Variables

```bash
export MAGICKCORE_HDRI_ENABLE=1
export MAGICKCORE_QUANTUM_DEPTH=16
```

#### Build Commands

```bash
# Build the library
cargo build

# Run all tests
cargo test

# Run specific test modules
cargo test --test test_basic
cargo test --test test_coordinates

# Run with verbose output
cargo test -- --nocapture
```

### 5. Test Execution Results

#### Expected Outcomes

| Test Category | Expected Result | Success Criteria |
|---------------|-----------------|------------------|
| Unit Tests | ✅ All pass | Mathematical calculations match expected values |
| Integration Tests | ✅ All pass | PNG generation successful, memory safe |
| PNG Validation | ✅ Valid headers | First 8 bytes match PNG magic header |
| Memory Safety | ✅ No violations | Image sizes within bounds, no panics |
| Error Handling | ✅ Graceful recovery | Invalid inputs handled without crashes |

#### Failure Modes & Recovery

| Failure Type | Detection Method | Recovery Strategy |
|--------------|------------------|-------------------|
| Invalid PNG | Header mismatch | Return `ChartError::CompositionError` |
| Memory overflow | Size bounds check | Return `ChartError::CompositionError` |
| Invalid settings | Parameter validation | Return `ChartError::InvalidSettings` |
| Empty data | Field validation | Generate empty canvas with valid PNG |

### 6. Verification Framework

#### Mathematical Stability

1. **Rotation Matrix Verification**: Confirm that `rotate_coordinates()` produces identical results to historical Python implementation
2. **Quadrant Mapping**: Verify that points map to correct visual quadrants (0°, 90°, 180°, 270°)
3. **Matrix Orthogonality**: Ensure rotation matrices preserve vector properties

#### Pixel Layout Validation

1. **PNG Header Check**: First 8 bytes must match `[137, 80, 78, 71, 13, 10, 26, 10]`
2. **Image Size Validation**: Generated images must be within reasonable size bounds
3. **Recursive Composition**: Overlay scaling and positioning must be mathematically correct

#### Memory Safety

1. **Bounds Checking**: Prevent buffer overflows with size validation
2. **Thread Safety**: `std::sync::Once` protects ImageMagick initialization
3. **Zero-Copy**: Use references (`&[u8]`) for efficient data passing

### 7. Continuous Integration Recommendations

#### GitHub Actions Example

```yaml
name: iyou_chart_kernel CI

on: [push, pull_request]

env:
  MAGICKCORE_HDRI_ENABLE: 1
  MAGICKCORE_QUANTUM_DEPTH: 16

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Install dependencies
      run: sudo apt-get update && sudo apt-get install -y libmagickwand-dev
    - name: Run tests
      run: cargo test --all-features
```

#### Test Coverage Targets

- **Unit Test Coverage**: ≥ 90% of core mathematical functions
- **Integration Test Coverage**: ≥ 80% of generator code paths
- **Error Path Coverage**: 100% of `ChartError` variants

### 8. Performance Benchmarks (Future)

#### Target Metrics

| Operation | Target Time | Memory Usage |
|-----------|-------------|---------------|
| Gen1 Generation | < 50ms | < 10MB |
| Gen2 Generation | < 100ms | < 20MB |
| Font Metrics | < 5ms | < 1MB |
| Coordinate Rotation | < 1ms | < 1KB |

#### Benchmarking Commands

```bash
# Run benchmarks
cargo bench

# Profile with perf
perf record --call-graph dwarf cargo test
perf report
```

## Summary

The test matrix provides comprehensive verification of:

✅ **Mathematical Stability**: Rotation matrices and coordinate transformations
✅ **Pixel Layout Accuracy**: PNG generation and visual positioning  
✅ **Memory Safety**: Bounds checking and thread safety
✅ **Error Handling**: Graceful recovery from invalid inputs
✅ **Recursive Composition**: Proper scaling and centering of nested overlays

All tests are designed to run with the required environment variables:
```bash
export MAGICKCORE_HDRI_ENABLE=1
export MAGICKCORE_QUANTUM_DEPTH=16
```

The framework ensures that the Rust implementation maintains pixel-perfect compatibility with the historical Python prototypes while providing enhanced safety and performance.
