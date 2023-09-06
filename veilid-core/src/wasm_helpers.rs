cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub use tsify::*;
        pub use wasm_bindgen::prelude::*;

        macro_rules! from_impl_to_jsvalue {
            ($name: ident) => {
                impl From<$name> for JsValue {
                    fn from(value: $name) -> Self {
                        serde_wasm_bindgen::to_value(&value).unwrap()
                    }
                }
            }
        }
    } else {
        macro_rules! from_impl_to_jsvalue {
            ($name: ident) => {}
        }
    }
}
pub(crate) use from_impl_to_jsvalue;
