# veilid-wasm

## Notes

- [`wasm_bindgen`](https://rustwasm.github.io/wasm-bindgen/) is used to generate interop code between JavaScript and Rust, as well as basic TypeScript types.
- [`tsify`](https://github.com/madonoharu/tsify) is used to export TypeScript types along-side [`wasm_bindgen`](https://rustwasm.github.io/wasm-bindgen/) and [`serde_wasm_bindgen`](https://github.com/cloudflare/serde-wasm-bindgen), and enables serialization/deserialization.
