extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(StructFieldsAsStrings)]
pub fn get_struct_fields(input: TokenStream) -> TokenStream {
    
    // turn the input into a syntax tree
    let abstract_syntax_tree: DeriveInput = syn::parse(input).unwrap();
    
    // collect the field names of the struct as a vector of strings
    let struct_fields: Vec<String> = match abstract_syntax_tree.data {
        syn::Data::Struct(data) => data.fields.into_iter()
        .filter_map(|field| {
            match field.ident {
                Some(ident) => Some(ident.to_string()),
                None => None
            }
        }).collect(),
        _ => panic!("Only structs are supported"),
    };

    let the_struct = abstract_syntax_tree.ident;

    // generate the impl
    quote! {
        impl StructFieldsAsStrings for #the_struct {
            fn get_struct_fields() -> Vec<&'static str> {
                vec![#(#struct_fields),*]
            }
        }
    }.into()
}

#[proc_macro_derive(AsHashMap)]
pub fn to_hashmap(input: TokenStream) -> TokenStream {

    let ast: DeriveInput = syn::parse(input).unwrap();
    
    let inserts = match ast.data {
        syn::Data::Struct(data) => data.fields.into_iter()
        .filter_map(|field| {
            let struct_item = field.ident.as_ref().unwrap();
            let struct_item_name = struct_item.to_string();
            Some(quote! {
                hashmap.insert(#struct_item_name, self.#struct_item.to_string());
            })
        }),
        _ => panic!("Only structs are supported"),
    };

    let the_struct = ast.ident;

    quote::quote! {
        impl AsHashMap for #the_struct {
            fn to_hashmap(&self) -> HashMap<&'static str, String> {
                let mut hashmap: std::collections::HashMap<&'static str, String> = HashMap::new();

                #(#inserts)*

                hashmap
            }
        }
    }.into()
}

