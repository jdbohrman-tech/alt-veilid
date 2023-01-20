@echo off
cargo test --features=rt-tokio -- --nocapture
cargo test --features=rt-async-std -- --nocapture

