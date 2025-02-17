pub fn deriveNew(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => deriveForStruct(&ast, data),
        syn::Data::Enum(data) => deriveForEnum(&ast, &data),
        syn::Data::Union(_) => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}


macro_rules! tokenize {
    ($($tokens:tt)*) => {
        (quote::quote_spanned!(proc_macro2::Span::call_site() => $($tokens)*))
    }
}

fn deriveForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let syn::Data::Struct(data) = &ast.data else {
        panic!()
    };
    let fields = &data.fields;

    match fields {
        syn::Fields::Named(fields) => {
            let structIdent = &ast.ident;
            let structGenerics = &ast.generics;

            let fieldIdents = fields.named
                .iter()
                .map(|field| &field.ident);
            let fieldTypes = fields.named
                .iter()
                .map(|field| &field.ty);

            return tokenize! {
                impl #structGenerics #structIdent {
                    pub fn new(#(#fieldIdents: #fieldTypes),*) -> Self {
                        return Self {
                            #(#fieldIdents),*
                        };
                    }
                }
            }
        },
        syn::Fields::Unnamed(fields) => {
            let structIdent = &ast.ident;
            let structGenerics = &ast.generics;

            let fieldIndices = (0..fields.unnamed.len())
                .map(syn::Index::from);
            let fieldTypes = fields.unnamed
                .iter()
                .map(|field| &field.ty);

            return tokenize! {
                impl #structGenerics #structIdent {
                    pub fn new(#(#fieldIndices: #fieldTypes),*) -> Self {
                        return Self {
                            #(#fieldIndices),*
                        };
                    }
                }
            }
        },
        syn::Fields::Unit => {
            let structIdent = &ast.ident;
            let structGenerics = &ast.generics;
            
            return tokenize! {
                impl #structGenerics #structIdent {
                    pub fn new() -> Self {
                        return Self;
                    }
                }
            }
        },
    }

    todo!()
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    todo!()
}
