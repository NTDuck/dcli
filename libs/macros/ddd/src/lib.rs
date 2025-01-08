use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Identifier)]
pub fn identifier_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    return impl_identifier(&ast);
}

fn impl_identifier(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let result = quote! {
        impl ddd::Identifier for #struct_name {}
    };

    return result.into();
}
