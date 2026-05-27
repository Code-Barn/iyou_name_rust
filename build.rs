/// Build script for iyou_chart_kernel
/// Sets required environment variables for ImageMagick

fn main() {
    // Set environment variables for ImageMagick
    println!("cargo:rustc-env=MAGICKCORE_HDRI_ENABLE=1");
    println!("cargo:rustc-env=MAGICKCORE_QUANTUM_DEPTH=16");

    // For macOS Clang compatibility
    println!("cargo:rustc-env=__ALLOW_UNNAMED_STRUCTURES=1");

    // Rerun if any source files change
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
}
