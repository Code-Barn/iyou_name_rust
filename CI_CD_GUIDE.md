# CI/CD Guide for iyou_chart_kernel Rust Extension

## Overview

This document provides comprehensive guidance for setting up Continuous Integration and Continuous Deployment (CI/CD) for the `iyou_chart_kernel` Rust extension that integrates with the Django-based namechart application.

## CI/CD Strategy

### Key Principles

1. **Dual Repository Coordination**: The Rust extension (`iyou_name_rust`) must be built and tested in sync with the main Django application (`iyou_name`)
2. **Cross-Language Integration**: CI must handle both Rust compilation and Python integration testing
3. **Environment Consistency**: Ensure build environments match production deployment targets

## Recommended CI/CD Pipeline

### 1. GitHub Actions Setup

Create `.github/workflows/ci_cd.yml` in the Rust repository:

```yaml
name: Rust Extension CI/CD

on:
  push:
    branches: [ main, dev ]
  pull_request:
    branches: [ main, dev ]

env:
  CARGO_TERM_COLOR: always
  PYTHON_VERSION: "3.13"

jobs:
  build-and-test:
    name: Build and Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Install Rust toolchain
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        components: rustfmt, clippy

    - name: Install Python
      uses: actions/setup-python@v4
      with:
        python-version: ${{ env.PYTHON_VERSION }}

    - name: Install system dependencies (Linux)
      if: matrix.os == 'ubuntu-latest'
      run: |
        sudo apt-get update
        sudo apt-get install -y pkg-config libmagickwand-dev build-essential

    - name: Install system dependencies (macOS)
      if: matrix.os == 'macos-latest'
      run: |
        brew update
        brew install imagemagick pkg-config

    - name: Install system dependencies (Windows)
      if: matrix.os == 'windows-latest'
      run: |
        choco install imagemagick

    - name: Cache cargo dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

    - name: Build Rust extension
      uses: actions-rs/cargo@v1
      with:
        command: build
        args: --release

    - name: Run Rust tests
      uses: actions-rs/cargo@v1
      with:
        command: test

    - name: Run clippy
      uses: actions-rs/cargo@v1
      with:
        command: clippy
        args: -- -D warnings

    - name: Run rustfmt
      uses: actions-rs/cargo@v1
      with:
        command: fmt
        args: -- --check

  integration-test:
    name: Python Integration Test
    needs: build-and-test
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout Rust code
      uses: actions/checkout@v4
      with:
        path: iyou_name_rust

    - name: Checkout Django app
      uses: actions/checkout@v4
      with:
        repository: your-org/iyou_name
        path: iyou_name
        token: ${{ secrets.DJANGO_REPO_TOKEN }}

    - name: Install Rust toolchain
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Install Python
      uses: actions/setup-python@v4
      with:
        python-version: ${{ env.PYTHON_VERSION }}

    - name: Install system dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y pkg-config libmagickwand-dev

    - name: Build Rust extension
      working-directory: iyou_name_rust
      run: cargo build --release

    - name: Install Rust extension in Django app
      working-directory: iyou_name_rust
      run: |
        cp target/release/libiyou_chart_kernel.so ../iyou_name/apps/generator/rust_extension/

    - name: Set up Django environment
      working-directory: iyou_name
      run: |
        python -m pip install -r requirements.txt
        python manage.py migrate

    - name: Run Django tests with Rust extension
      working-directory: iyou_name
      run: python manage.py test apps.generator.tests.test_rust_integration

  deploy:
    name: Deploy to Production
    needs: integration-test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Install Rust toolchain
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Build release
      run: cargo build --release

    - name: Create artifact
      run: |
        mkdir -p artifact
        cp target/release/libiyou_chart_kernel.so artifact/
        cp README.md artifact/
        cp LICENSE artifact/

    - name: Upload artifact
      uses: actions/upload-artifact@v3
      with:
        name: rust-extension-${{ github.sha }}
        path: artifact/

    - name: Deploy to package registry
      run: |
        # Add your deployment logic here
        # Could be uploading to PyPI, private package registry, etc.
        echo "Deployment logic would go here"

    - name: Notify Django app to update
      run: |
        # This could be a webhook call or other notification
        # to trigger the Django app to pull the new extension
        echo "Notification logic would go here"

## Docker-Based CI/CD Alternative

For more complex environments, consider using Docker:

```yaml
name: Docker CI/CD

on:
  push:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Set up Docker Buildx
      uses: docker/setup-buildx-action@v2

    - name: Login to Docker Hub
      uses: docker/login-action@v2
      with:
        username: ${{ secrets.DOCKER_HUB_USERNAME }}
        password: ${{ secrets.DOCKER_HUB_TOKEN }}

    - name: Build and push
      uses: docker/build-push-action@v4
      with:
        context: .
        push: true
        tags: your-org/iyou-chart-kernel:latest,your-org/iyou-chart-kernel:${{ github.sha }}
        build-args: |
          RUST_VERSION=stable
          PYTHON_VERSION=3.13

    - name: Deploy to Kubernetes
      run: |
        # Example kubectl commands to update deployment
        # kubectl set image deployment/iyou-chart-kernel iyou-chart-kernel=${{ github.sha }}
        echo "Kubernetes deployment would go here"
```

## Environment Configuration

### Required Environment Variables

```bash
# For local development
export IMAGE_MAGICK_DIR=/usr/local/Cellar/imagemagick/7.1.2-24

# For CI/CD
export CARGO_TERM_COLOR=always
export PYTHON_VERSION="3.13"
export RUSTFLAGS="-C target-cpu=native"
```

### System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y pkg-config libmagickwand-dev build-essential python3-dev
```

**macOS (Homebrew):**
```bash
brew update
brew install imagemagick pkg-config python@3.13
```

**Windows (Chocolatey):**
```powershell
choco install imagemagick
```

## Build Troubleshooting

### Common Issues and Solutions

1. **ImageMagick version mismatch:**
   - Ensure ImageMagick 7.x is installed
   - Set `IMAGE_MAGICK_DIR` environment variable
   - Use `brew link --force imagemagick` on macOS if needed

2. **Bindgen failures:**
   - Update bindgen: `cargo update -p bindgen`
   - Try cleaning: `cargo clean && rm -rf target/`
   - Use newer Rust toolchain

3. **Linking errors:**
   - Verify `pkg-config` can find ImageMagick
   - Check library paths with `pkg-config --libs MagickWand-7`
   - Set `LIBRARY_PATH` if needed

4. **Python integration issues:**
   - Ensure Python version matches PyO3 requirements
   - Check `PYTHON_SYS_EXECUTABLE` environment variable
   - Use `maturin` for better Python-Rust integration

## Deployment Strategies

### Option 1: Direct File Deployment

1. Build the extension: `cargo build --release`
2. Copy the `.so`/`.dll`/`.dylib` file to the Django app
3. Restart the Django application server

### Option 2: Package Registry

1. Publish to crates.io or private registry
2. Add dependency to Django app's `requirements.txt`
3. Use `maturin` for PyPI publishing

### Option 3: Containerized Deployment

1. Build Docker image with Rust extension
2. Deploy container to Kubernetes/ECS/etc.
3. Mount or copy extension into Django container

## Monitoring and Maintenance

### CI/CD Health Metrics

- Build success rate
- Test coverage trends
- Build duration
- Dependency update frequency

### Maintenance Tasks

1. **Monthly:** Update Rust toolchain
2. **Quarterly:** Audit dependencies with `cargo audit`
3. **As needed:** Update ImageMagick version
4. **Before releases:** Run full integration test suite

## Security Considerations

1. **Dependency scanning:** Use `cargo audit` in CI
2. **Secret management:** Never commit secrets to repository
3. **SBOM generation:** Create Software Bill of Materials
4. **Vulnerability scanning:** Regular scans of container images

## Performance Optimization

1. **Caching:** Cache Cargo dependencies between CI runs
2. **Parallel builds:** Use `cargo build -j$(nproc)`
3. **Release builds:** Always use `--release` for production
4. **Profile-guided optimization:** Consider PGO for critical paths

## Rollback Procedures

1. **Version tagging:** Always tag releases
2. **Artifact retention:** Keep previous build artifacts
3. **Quick rollback:** Maintain script to revert to previous version
4. **Database compatibility:** Ensure Rust extension changes are backward compatible

## Future Enhancements

1. **Automated benchmarking:** Add performance regression tests
2. **Cross-compilation:** Build for multiple platforms in single CI run
3. **Canary deployments:** Gradual rollout to production
4. **Automated documentation:** Generate API docs from Rust doc comments

## Appendix: Manual Build Instructions

### Local Development Build

```bash
# Install dependencies
git clone https://github.com/your-org/iyou_name_rust.git
cd iyou_name_rust

# Build for development
cargo build

# Build for production
cargo build --release

# Run tests
cargo test

# Install in Django app
cp target/release/libiyou_chart_kernel.so /path/to/django/app/rust_extension/
```

### Cross-Compilation Example

```bash
# Install cross-compilation target
rustup target add x86_64-unknown-linux-gnu

# Cross-compile
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc \
cargo build --release --target x86_64-unknown-linux-gnu
```

This comprehensive CI/CD guide should help establish robust build, test, and deployment pipelines for the Rust extension while maintaining compatibility with the Django application.