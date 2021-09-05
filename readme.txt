# 
add this to cargo.toml to disable wasm-opt and fix the error when using simd128 feature.
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
