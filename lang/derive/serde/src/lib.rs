extern crate proc_macro;

#[cfg(feature = "lazy-account")]
mod lazy;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::FoundCrate;
use quote::quote;
use syn::Ident;

fn gen_borsh_serialize(input: TokenStream) -> TokenStream2 {
    let input = TokenStream2::from(input);
    let attrs = helper_attrs("BorshSerialize");
    quote! {
        #attrs
        #input
    }
}

#[proc_macro_derive(AnchorSerialize, attributes(borsh, borsh_skip))]
pub fn anchor_serialize(input: TokenStream) -> TokenStream {
    #[cfg(not(feature = "idl-build"))]
    let ret = gen_borsh_serialize(input);
    #[cfg(feature = "idl-build")]
    let ret = gen_borsh_serialize(input.clone());

    #[cfg(feature = "idl-build")]
    {
        use anchor_syn::idl::*;
        use quote::quote;
        use syn::Item;

        let idl_build_impl = match syn::parse(input).unwrap() {
            Item::Struct(item) => impl_idl_build_struct(&item),
            Item::Enum(item) => impl_idl_build_enum(&item),
            Item::Union(item) => impl_idl_build_union(&item),
            // Derive macros can only be defined on structs, enums, and unions.
            _ => unreachable!(),
        };

        return TokenStream::from(quote! {
            #ret
            #idl_build_impl
        });
    };

    #[allow(unreachable_code)]
    TokenStream::from(ret)
}

fn gen_borsh_deserialize(input: TokenStream) -> TokenStream2 {
    let input = TokenStream2::from(input);
    let attrs = helper_attrs("BorshDeserialize");
    quote! {
        #attrs
        #input
    }
}

#[proc_macro_derive(AnchorDeserialize, attributes(borsh, borsh_skip, borsh_init))]
pub fn borsh_deserialize(input: TokenStream) -> TokenStream {
    #[cfg(feature = "lazy-account")]
    {
        let deser = gen_borsh_deserialize(input.clone());
        let lazy = lazy::gen_lazy(input).unwrap_or_else(|e| e.to_compile_error());
        quote::quote! {
            #deser
            #lazy
        }
        .into()
    }
    #[cfg(not(feature = "lazy-account"))]
    gen_borsh_deserialize(input).into()
}

#[cfg(feature = "lazy-account")]
#[proc_macro_derive(Lazy)]
pub fn lazy(input: TokenStream) -> TokenStream {
    lazy::gen_lazy(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn helper_attrs(mac: &str) -> TokenStream2 {
    // Borsh derives cannot be emitted directly from another derive macro, so emit
    // a duplicate item with the inner derive and erase that duplicate afterwards.
    let mac_path = Ident::new(mac, Span::call_site());
    let anchor_path = anchor_path();
    let borsh_path = quote! { #anchor_path::prelude::borsh };
    let borsh_path_str = borsh_path.to_string();

    quote! {
        #[derive(#borsh_path::#mac_path)]
        #[borsh(crate = #borsh_path_str)]
        #[#anchor_path::__erase]
    }
}

fn anchor_path() -> TokenStream2 {
    match proc_macro_crate::crate_name("anchor-lang")
        .expect("`anchor-derive-serde` must be used through `anchor-lang`")
    {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { #ident }
        }
    }
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn __erase(_: TokenStream, _: TokenStream) -> TokenStream {
    TokenStream::new()
}
