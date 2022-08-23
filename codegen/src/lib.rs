mod main;

mod persist;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    main::r#impl(attr, item)
}

#[proc_macro_derive(Persist)]
pub fn derive_persist(input: TokenStream) -> TokenStream {
    persist::persist_derive::r#impl(input)
}
