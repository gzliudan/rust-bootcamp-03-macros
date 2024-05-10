use proc_macro::TokenStream;
use quote::quote;

/*
input = DeriveInput {
    attrs: [],
    vis: Visibility::Inherited,
    ident: Ident {
        ident: "Direction",
        span: #0 bytes(55..64),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    data: Data::Enum {
        enum_token: Enum,
        brace_token: Brace,
        variants: [
            Variant {
                attrs: [],
                ident: Ident {
                    ident: "Up",
                    span: #0 bytes(71..73),
                },
                fields: Fields::Unnamed {
                    paren_token: Paren,
                    unnamed: [
                        Field {
                            attrs: [],
                            vis: Visibility::Inherited,
                            mutability: FieldMutability::None,
                            ident: None,
                            colon_token: None,
                            ty: Type::Path {
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "DirectionUp",
                                                span: #0 bytes(74..85),
                                            },
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                            },
                        },
                    ],
                },
                discriminant: None,
            },
            Comma,
            Variant {
                attrs: [],
                ident: Ident {
                    ident: "Down",
                    span: #0 bytes(92..96),
                },
                fields: Fields::Unit,
                discriminant: None,
            },
            Comma,
        ],
    },
}
*/

// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("input = {:#?}", input);
    // get the ident
    let ident = input.ident;
    // println!("ident = {:#?}", ident);
    // get generics
    let generics = input.generics;
    // println!("generics = {:#?}", generics);
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    // for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let filed_type = &field.ty;
                    quote! {
                        impl #generics From<#filed_type> for #ident #generics {
                            fn from(v: #filed_type) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }

            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_fields) => quote! {},
        }
    });

    // quote return proce-macro2 TokenTtream so we need to convert it to TokenStream
    quote! {
        #(#from_impls)*
    }
    .into()
}
