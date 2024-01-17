// Standard Uses

// Crate Uses

// External Uses
use quote::{quote, ToTokens};
use syn::{ImplGenerics, ItemImpl};
use proc_macro2::TokenStream;


pub fn setup_generic_impl(attr: TokenStream, imp: ItemImpl) -> TokenStream {
    let name = imp.self_ty.into_token_stream().to_string();
    let (imp_gens, _typ_gens, _whr_cls)
        = imp.generics.split_for_impl();

    let ctor = make_lua_constructor(&*name, imp_gens).into_token_stream();

    quote! {
        // Generated impl
        #ctor
    }
}


fn make_lua_constructor(impl_name: &str, imp_gens: ImplGenerics) -> TokenStream {
    let return_type = quote ! { Self<B> };
    let params = quote! { param0: B };


    let tokens = quote! {
        impl #impl_name {
            fn _lua_generic_ctor(#params) -> #return_type {
                todo!()
            }
        }
    };
    tokens
}
