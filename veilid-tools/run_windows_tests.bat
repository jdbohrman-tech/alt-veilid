@echo off
cargo test --features=tracing -- --nocapture
cargo test --no-default-features --features=rt-async-std,tracing -- --nocapture
cargo test -- --nocapture
cargo test --no-default-features --features=rt-async-std -- --nocapture
