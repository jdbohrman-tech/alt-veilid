@echo off
cargo test -- --nocapture
cargo test --no-default-features --features=default-async-std -- --nocapture

