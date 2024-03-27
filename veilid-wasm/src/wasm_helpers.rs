use super::*;

cfg_if::cfg_if! {
  if #[cfg(target_arch = "wasm32")] {
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Uint8Array[]")]
    pub type Uint8ArrayArray;
}
/// Convert a `Vec<Uint8Array>` into a `js_sys::Array` with the type of `Uint8Array[]`
pub(crate) fn into_unchecked_uint8array_array(items: Vec<Uint8Array>) -> Uint8ArrayArray {
    items
        .iter()
        .collect::<js_sys::Array>()
        .unchecked_into::<Uint8ArrayArray>() // TODO: can I do this a better way?
}

/// Convert a StringArray (`js_sys::Array` with the type of `string[]`) into `Vec<String>`
pub(crate) fn into_unchecked_string_vec(items: StringArray) -> Vec<String> {
    items
        .unchecked_into::<js_sys::Array>()
        .to_vec()
        .into_iter()
        .map(|i| serde_wasm_bindgen::from_value(i).unwrap())
        .collect::<Vec<String>>()
}
