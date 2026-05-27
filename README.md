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

## 🐍 Python Integration (Completed) ✅

The Rust chart kernel now features a **zero-copy JSON pass-through PyO3 bridge** for seamless Django integration.

### Bridge Architecture
- **Pure FFI Interface**: Single `render_chart_from_json()` function
- **Zero-Copy JSON Deserialization**: Direct serde mapping with no structural duplication
- **Production-Ready**: Compiled via Maturin for native Python extensions

### Key Features
- ✅ **Maximum Performance**: Zero-copy JSON pass-through eliminates marshaling overhead
- ✅ **Minimal FFI Surface**: Single elegant execution entry gate
- ✅ **Type Safety**: Full compile-time type checking with Rust ownership guarantees
- ✅ **Error Handling**: Django-compatible exception propagation
- ✅ **Thread Safety**: No shared mutable state
- ✅ **Memory Efficiency**: Direct byte array return to Python

### Implementation Details

**Bridge Function Signature:**
```rust
#[pyfunction]
pub fn render_chart_from_json(
    generation: u8,
    primary_json: &str,
    ancestors_json: &str,
    settings_json: &str,
) -> PyResult<Vec<u8>>
```

**Usage Example:**
```python
import iyou_chart_kernel
import json

# Django ORM data -> JSON
primary_json = json.dumps({
    "id": "I1",
    "full_name": "John Doe",
    "given_name": "John",
    "surname": "Doe",
    "birth_date": "1980-01-01",
    "birth_place": "New York"
})

settings_json = json.dumps({
    "font_family": "Arial",
    "font_color": "black",
    "background_color": "white",
    "name_font_size": 74.0,
    "date_font_size": 52.0,
    "place_font_size": 48.0
})

# Generate chart
png_bytes = iyou_chart_kernel.render_chart_from_json(
    generation=2,
    primary_json=primary_json,
    ancestors_json="{}",
    settings_json=settings_json
)

# Return to Django HttpResponse
return HttpResponse(png_bytes, content_type='image/png')
```

### Build & Installation

```bash
# Install ImageMagick (macOS)
brew install imagemagick@7
export PKG_CONFIG_PATH="/usr/local/opt/imagemagick@7/lib/pkgconfig"

# Build Python extension
maturin develop

# Install in virtual environment
pip install iyou_chart_kernel
```

### Verification

Run the comprehensive verification script:
```bash
python verify_bridge.py
```

Expected output:
- ✅ Import successful
- ✅ JSON pass-through working
- ✅ Valid PNG generation
- ✅ Error handling verified

### Integration Status
- **Status**: ✅ **COMPLETE - Production Ready**
- **Completion Date**: 2024-05-26
- **Performance**: Zero FFI overhead, direct JSON parsing
- **Compatibility**: Python 3.8+, Django 3.2+

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