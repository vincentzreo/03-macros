use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
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
}
