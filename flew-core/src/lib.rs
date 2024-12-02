use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemStruct};

pub mod store;

#[proc_macro_attribute]
pub fn node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    // Tambahkan struct ke enum `Node`
    let enum_variant = format_ident!("{}", struct_name);
    let expanded_enum = quote! {
        #[derive(Debug, Serialize, Deserialize, Clone)]
        #[serde(untagged)]
        pub enum Node {
            #enum_variant(Vec<#struct_name>),
            #enum_variant(#struct_name),
        }
    };

    // Hasilkan kembali struct dan tambahkan enum ke token stream
    let expanded_struct = quote! {
        #input

        #expanded_enum
    };

    TokenStream::from(expanded_struct)
}
