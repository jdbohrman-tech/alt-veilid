use super::*;

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type StringArray;
}

/// Convert a `Vec<String>` into a `js_sys::Array` with the type of `string[]`
pub(crate) fn into_unchecked_string_array(items: Vec<String>) -> StringArray {
    items
        .iter()
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<StringArray>() // TODO: can I do this a better way?
}
