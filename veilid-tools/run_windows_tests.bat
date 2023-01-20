@echo off
cargo test --features=rt-tokio,tracing -- --nocapture
cargo test --features=rt-async-std,tracing -- --nocapture
cargo test --features=rt-tokio -- --nocapture
cargo test --features=rt-async-std -- --nocapture
