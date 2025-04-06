extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(StructFieldsAsStrings)]
pub fn get_struct_fields(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_get_struct_fields(ast)
}



fn impl_get_struct_fields(ast: DeriveInput) -> TokenStream {
    let struct_name = ast.ident;

    // collect the field names of the struct as a vector of strings
    let struct_fields: Vec<String> = match ast.data {
        syn::Data::Struct(data) => data.fields.into_iter()
        .filter_map(|field| {
            match field.ident {
                Some(ident) => Some(ident.to_string()),
                None => None
            }
        }).collect(),
        _ => panic!("Only structs are supported"),
    };

    // generate the impl
    quote::quote! {

        impl StructFieldsAsStrings /*the trait*/ for #struct_name {
            fn get_struct_fields() -> Vec<&'static str> {
                vec![#(#struct_fields),*]
            }
        }
    }.into()
}

