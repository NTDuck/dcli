use quote::quote;

pub fn deriveNew(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => deriveForStruct(&ast, data),
        syn::Data::Enum(data) => deriveForEnum(&ast, &data),
        syn::Data::Union(_) => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}

fn deriveForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => {
            let structIdent = &ast.ident;
            let structGenerics = &ast.generics;

            let fieldIdents: Vec<_> = fields.named
                .iter()
                .map(|field| &field.ident)
                .collect();
            let fieldTypes: Vec<_> = fields.named
                .iter()
                .map(|field| &field.ty)
                .collect();

            quote! {
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

            let fieldIndices: Vec<_> = (0..fields.unnamed.len())
                .map(syn::Index::from)
                .collect();
            let fieldTypes: Vec<_> = fields.unnamed
                .iter()
                .map(|field| &field.ty)
                .collect();

            quote! {
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
            
            return quote! {
                impl #structGenerics #structIdent {
                    pub fn new() -> Self {
                        return Self;
                    }
                }
            };
        },
    };
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variantFns = data.variants
        .iter()
        .map(|variant| {
            match &variant.fields {
                syn::Fields::Named(fields) => {
                    
                },
                syn::Fields::Unnamed(fields_unnamed) => todo!(),
                syn::Fields::Unit => todo!(),
            }
        });

    todo!()
}
