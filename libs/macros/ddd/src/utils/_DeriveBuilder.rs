macro_rules! derive {
    ($class:ident, $func:ident) => {
        use proc_macro::TokenStream;

        #[allow(non_snake_case)]
        #[proc_macro_derive($class)]
        pub fn $class(input: TokenStream) -> TokenStream {
            let ast = syn::parse(input).unwrap();
            return $func(&ast);
        }
    };
}

pub(crate) use derive;
