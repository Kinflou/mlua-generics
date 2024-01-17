// External Uses
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Generics;


fn make_userdata_type_method(impl_name: &str, generics: Generics) -> TokenStream {
    let return_type = quote ! {
        Self<B>
    };

    let params = quote! {
        param0: B
    };

    let tokens = quote! {
        impl #impl_name {
            fn _lua_generic_ctor(#params) -> LuaResult<#return_type> {
                Self {  }
                todo!()
            }
        }
    };
    tokens
}

fn insert_in_impl(generics: Generics) {
    let mut generic_hashes = vec![];
    for generic in generics.params.iter() {
        let hash = generic.into_token_stream().to_string();

        generic_hashes.push(quote! {
            std::hash::Hash::hash(&std::any::TypeId::of::<#hash>(), &mut hasher);
        })
    }

    let type_method = quote! {
        methods.add_method_mut(parse_table::DEFAULT_TYPE_NAME, |_, _, ()| {
            let mut hasher = DefaultHasher::new();
            #(generic_hashes)
            Ok(hasher.finish())
        });
    };
}
