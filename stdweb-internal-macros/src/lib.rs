#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate base_x;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod macro_js_export;
mod macro_async_test;

#[proc_macro_attribute]
pub fn js_export( attrs: proc_macro::TokenStream, input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    macro_js_export::js_export( attrs, input )
}

#[proc_macro_attribute]
pub fn async_test( attrs: proc_macro::TokenStream, input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    macro_async_test::async_test( attrs, input )
}
