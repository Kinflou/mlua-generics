// Relative Modules
mod concrete;
mod concrete_global;

// Standard Uses
use proc_macro::TokenStream;
use syn::{ItemImpl, parse_macro_input};

// Crate Uses

// External Uses


#[proc_macro_attribute]
pub fn lua_generic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let imp = parse_macro_input!(item as ItemImpl);

    concrete::setup_generic_impl(attr.into(), imp).into()
}

#[proc_macro_attribute]
pub fn lua_global_generic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let imp = parse_macro_input!(item as ItemImpl);

    concrete_global::setup_generic_impl(attr.into(), imp).into()
}
