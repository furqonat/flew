use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::sync::RwLock;
use syn::{parse_macro_input, DeriveInput, ItemFn};

static STRUCT_NAMES: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(Vec::new()));

#[proc_macro_derive(Entity)]
pub fn generate_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.to_string();
    {
        let mut struct_names = STRUCT_NAMES.write().unwrap();
        if !struct_names.contains(&struct_name) {
            struct_names.push(struct_name);
        }
    }

    TokenStream::new()
}

/// This macro generates a consolidated enum using all tracked struct names
#[proc_macro_attribute]
pub fn flew_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let struct_names = STRUCT_NAMES.read().unwrap();

    let variants = struct_names.iter().map(|name| {
        let ident = format_ident!("{}", name);
        quote! { #ident(#ident) }
    });

    let fn_block = &input.block;
    let fn_attrs = &input.attrs;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;

    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            #[derive(Debug, Clone, Serialize, Deserialize)]
            #[serde(untagged)]
            pub enum Entity {
                #(#variants,)*
            }

            #fn_block
        }
    };

    TokenStream::from(expanded)
}
