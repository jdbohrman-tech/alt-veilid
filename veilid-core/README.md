<!-- DO NOT EDIT BELOW - content within cargo-sync-readme blocks is generated -->
<!-- cargo-sync-readme start -->

# The Veilid Framework

This is the core library used to create a Veilid node and operate it as part of an application.

`veilid-core` contains all of the core logic for Veilid and can be used in mobile applications as well as desktop
and in-browser WebAssembly apps.

## Getting started

- [Developer Book](https://veilid.gitlab.io/developer-book/)
- [Examples](https://gitlab.com/veilid/veilid/-/tree/main/veilid-core/examples/)
- [API Documentation](https://docs.rs/veilid-core)

The public API is accessed by getting a [VeilidAPI] object via a call to [api_startup], [api_startup_json], or
[api_startup_config].

From there, a [RoutingContext] object can get you access to public and private routed operations.

## Cargo features

The default `veilid-core` configurations are:

* `default` - Uses `tokio` as the async runtime.

If you use `--no-default-features`, you can switch to other runtimes:

* `default-async-std` - Uses `async-std` as the async runtime.
* `default-wasm` - When building for the `wasm32` architecture, use this to enable `wasm-bindgen-futures` as the async runtime.


<!-- cargo-sync-readme end -->
