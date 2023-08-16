// Copied from https://github.com/madonoharu/tsify/issues/24#issuecomment-1670228789
// TODO: I think this had more to do with:
// the trait `From<veilid_core::VeilidUpdate>` is not implemented for `wasm_bindgen::JsValue`
// than it does with tsify itself... Maybe rename to something else?

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(TsifyAsync)]
pub fn tsify_async_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_tsify_async_macro(&ast)
}

fn impl_tsify_async_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl From<#name> for JsValue {
            fn from(value: #name) -> Self {
                serde_wasm_bindgen::to_value(&value).unwrap()
            }
        }
    };
    gen.into()
}
