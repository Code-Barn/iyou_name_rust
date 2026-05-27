// Build script for iyou_chart_kernel
// Environment variables now managed via .cargo/config.toml

fn main() {
    // Rerun if any source files change
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
}
