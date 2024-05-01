cargo +nightly contract build --manifest-path foresta-contracts/algo-store/Cargo.toml --release
cargo +nightly contract build --manifest-path foresta-contracts/execute/Cargo.toml --release
cargo +nightly contract build --manifest-path foresta-contracts/catalog/Cargo.toml --release
cargo +nightly contract build --manifest-path foresta-contracts/proxy/Cargo.toml --release
# cargo +nightly contract build --manifest-path foresta-contracts/phala-contract/Cargo.toml --release
echo "Build complete."
