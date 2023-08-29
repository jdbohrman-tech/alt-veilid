@echo off
cargo test -- --nocapture
cargo test --features=rt-async-std -- --nocapture

