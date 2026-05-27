# iyou_chart_kernel - High-Performance Family Tree Chart Generation

A native Rust implementation of the family tree chart generation engine using magick-rust.

## Compilation Requirements

To build and test this project, you need to set the following environment variables:

```bash
export MAGICKCORE_HDRI_ENABLE=1
export MAGICKCORE_QUANTUM_DEPTH=16
```

## Test Matrix

### Unit Tests (Mathematical Stability Verification)

**Location**: `tests/unit/test_coordinates.rs`

- `test_rotate_coordinates_exact`: Verifies rotation matrix calculations for 0°, 90°, 180°, 270°
- `test_rotate_around_center_quadrants`: Validates quadrantal center mapping
- `test_center_constants`: Confirms absolute visual center (975.0, 975.0)
- `test_rotation_matrix_orthogonality`: Tests matrix composition properties

### Integration Tests (Pixel Layout Validation)

**Location**: `tests/integration/test_basic.rs`

- `test_environment_initialization`: Verifies thread-safe ImageMagick initialization
- `test_gen1_generation`: Tests single-person layout with PNG header validation
- `test_gen2_generation`: Tests recursive composition with memory safety checks
- `test_error_handling`: Validates graceful handling of invalid settings
- `test_empty_person_data`: Ensures robustness with minimal data

## Running Tests

## Example Usage

## 🐍 Python Integration (Upcoming)

The Rust chart kernel is designed for seamless integration with Python/Django via PyO3. See [PYTHON_INTEGRATION_ROADMAP.md](PYTHON_INTEGRATION_ROADMAP.md) for detailed plans.

### Bridge Architecture
- **Native PyO3 Extension Module** compiled via Maturin
- **Automated serde deserialization** for Python data classes
- **Production-ready** pattern matching the `iyou_idp` framework

### Key Features
- ✅ Maximum performance with native Rust extensions
- ✅ Type safety between Rust and Python
- ✅ Easy Python packaging and distribution
- ✅ Thread-safe initialization and operation

### Integration Timeline
- **Estimated Duration**: 7 weeks
- **Target Completion**: Q3 2024
- **Status**: Planning phase complete, ready for implementation

See [PYTHON_INTEGRATION_ROADMAP.md](PYTHON_INTEGRATION_ROADMAP.md) for full details.

See `examples/simple_generation.rs` for a complete working example that:
- Initializes the ImageMagick environment
- Creates chart settings and person data
- Generates Gen1 and Gen2 charts
- Saves output as PNG files

Run the example:
```bash
cargo run --example simple_generation
```

## Running Tests

```bash
# Set required environment variables
export MAGICKCORE_HDRI_ENABLE=1
export MAGICKCORE_QUANTUM_DEPTH=16

# Run all tests
cargo test

# Run specific test module
cargo test --test test_basic
cargo test --test test_coordinates

# Run with verbose output
cargo test -- --nocapture
```

## Test Validation Criteria

### PNG Header Verification
All generated images must contain the valid PNG magic header:
```rust
const PNG_MAGIC_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
```

### Memory Safety Checks
- Image sizes must be within reasonable bounds (1KB < size < 2MB)
- Recursive composition must not cause memory boundary violations
- Overlay scale factors must be applied correctly (0.50 for Gen2)

### Mathematical Stability
- Rotation matrices must preserve orthogonality
- Multiple 90° rotations must compose to equivalent single rotations
- Coordinate transformations must map accurately to quadrantal centers

## Implementation Features

✅ **Thread-Safe Initialization**: Single `magick_wand_genesis()` call protected by `std::sync::Once`
✅ **Active Context Metrics**: Font metrics calculated using active canvas context
✅ **Pixel-Perfect Constants**: Extracted from Python prototypes with exact values
✅ **Outward Quadrant Tracking**: Proper offset calculations using historical constants
✅ **Comprehensive Error Handling**: Custom `ChartError` type with proper propagation
✅ **Zero-Copy Operations**: Efficient `&[u8]` image data passing
✅ **Recursive Composition**: Matryoshka pattern with proper scaling and centering

## Project Structure

```
src/
├── core/              # Core data structures and utilities
├── generators/        # Generation-specific implementations
├── rendering/         # Text and graphics rendering
└── utils/             # Utility functions

tests/
├── integration/       # End-to-end tests
└── unit/              # Component tests
```

## Key Constants

- `IMAGE_CENTER_X = 975.0`
- `IMAGE_CENTER_Y = 975.0`
- `CANVAS_WIDTH = 1950`
- `CANVAS_HEIGHT = 1950`
- `GEN2_OVERLAY_SCALE = 0.50`

## Safety Features

1. **Environment Genesis**: Thread-safe initialization prevents memory instability
2. **Active Context**: Eliminates resolution drift in font metrics
3. **Outward Offsets**: Prevents rendering at (0,0) after transformations
4. **Error Propagation**: Comprehensive error handling throughout
5. **Memory Bounds**: Size validation prevents buffer overflows
