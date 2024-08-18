use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Fields, Lit};

#[proc_macro_derive(TryFromU8)]
pub fn derive_try_from_u8(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match input.data {
        Data::Enum(data_enum) => data_enum.variants,
        _ => panic!("TryFromU8 can only be derived for enums"),
    };

    let variant_arms = variants.iter().filter_map(|variant| {
        let variant_name = &variant.ident;
        if let Some((_, expr)) = &variant.discriminant {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) = expr
            {
                let value = lit_int.base10_parse::<u8>().expect("Invalid u8 value");
                Some(quote! {
                    #value => Ok(Self::#variant_name),
                })
            } else {
                None
            }
        } else {
            None
        }
    });

    let expanded = quote! {
        impl TryFrom<u8> for #name {
            type Error = NrbfError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    #(#variant_arms)*
                    _ => Err(NrbfError::UnexpectedRecordType),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
