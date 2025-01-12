#[allow(unused_imports)]
use ddd::Identifier;

#[proc_macro_derive(Identifier)]
pub fn identifier_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

    let struct_name = &ast.ident;
    let tokens = quote::quote! {
        impl ddd::Identifier for #struct_name {}
    };

    return tokens.into();
}
