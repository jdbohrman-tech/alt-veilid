<!-- DO NOT EDIT BELOW - content within cargo-sync-readme blocks is generated -->
<!-- cargo-sync-readme start -->

# veilid-tools

A collection of baseline tools for Rust development use by Veilid and Veilid-enabled Rust applications

These are used by `veilid-core`, `veilid-server`, `veilid-cli` and may be used by any other applications
that link in `veilid-core` if a common baseline of functionality is desired. Extending this crate with new
utility functions is encouraged rather than adding 'common' functionality to `veilid-core`, allowing it to
remain free of boilerplate and utility classes that could be reused elsewhere.

Everything added to this crate must be extensively unit-tested.

## Cargo features

The default `veilid-tools` configurations are:

- `default` - Uses `tokio` as the async runtime

If you use `--no-default-features`, you can switch to other runtimes:

- `rt-async-std` - Uses `async-std` as the async runtime
- `rt-wasm-bindgen` - When building for the `wasm32` architecture, use this to enable `wasm-bindgen-futures` as the async runtime

<!-- cargo-sync-readme end -->
