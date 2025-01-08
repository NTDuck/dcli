use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_identifier(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let result = quote! {
        impl ddd::Identifier for #struct_name {}
    };

    return result.into();
}
