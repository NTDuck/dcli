macro_rules! derive {
    ($class:ident, $func:ident) => {
        use proc_macro::TokenStream;
        
        #[allow(non_snake_case)]
        #[proc_macro_derive($class)]
        pub fn $class(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let ast = syn::parse(input).unwrap();
            return $func(&ast);
        }
    };
}

derive!(Identifier, impl_identifier);

use quote::quote;
use syn::DeriveInput;

fn impl_identifier(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let result = quote! {
        impl ddd::Identifier for #struct_name {}
    };

    return result.into();
}

