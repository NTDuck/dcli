#[macro_export]
macro_rules! define_procedural_macro {
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
