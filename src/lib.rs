// proc macro crate

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    print!("{:#?}", input);
    // get the ident
    let ident = input.ident;

    // get the generics
    let generics = input.generics;
    // get variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };
    // for each variant, get the fields and ident
    let from_impls = variants.iter().map(|variant| {
        let varent = &variant.ident;
        let fields = match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(v: #ty) -> Self {
                                #ident::#varent(v)
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_fields) => quote! {},
        };
        fields
    });
    // quote! return proc_macro2::TokenStream so we need to convert it to TokenStream
    quote! {
        #(#from_impls)*
    }
    .into()
}
