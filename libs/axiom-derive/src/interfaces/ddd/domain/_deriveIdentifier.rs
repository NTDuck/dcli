use quote::format_ident;
use quote::quote;

pub fn deriveIdentifier(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => deriveForStruct(&ast, data),
        syn::Data::Enum(data) => deriveForEnum(&ast, &data),
        _ => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}

fn deriveForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();

    match fields {
        syn::Fields::Named(fields) => {
            let fieldIdents = fields.named
                .iter()
                .map(|field| &field.ident)
                .collect::<Vec<_>>();

            return quote! {
                impl #structImplGenerics axiom::interfaces::ddd::domain::Identifier for #structIdent #structTypeGenerics #structWhereClause {}

                impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClause {}

                impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        return formatter
                            .debug_struct(stringify!(#structIdent))
                            #( .field(stringify!(#fieldIdents), &self.#fieldIdents) )*
                            .finish();
                    }
                }

                impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClause {
                    fn clone(&self) -> Self {
                        return Self {
                            #( #fieldIdents: self.#fieldIdents.clone(), )*
                        };
                    }
                }

                impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics #structWhereClause {
                    fn eq(&self, other: &Self) -> bool {
                        return #( self.#fieldIdents == other.#fieldIdents && )* true;
                    }
                }

                impl #structImplGenerics Eq for #structIdent #structTypeGenerics #structWhereClause {}

                impl #structImplGenerics std::hash::Hash for #structIdent #structTypeGenerics #structWhereClause {
                    fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
                        #( self.#fieldIdents.hash(state); )*
                    }
                }
            };
        },
        syn::Fields::Unnamed(fields) => {
            let fieldIndices = (0..fields.unnamed.len())
                .map(syn::Index::from)
                .collect::<Vec<_>>();

            return quote! {
                impl #structImplGenerics axiom::interfaces::ddd::domain::Identifier for #structIdent #structTypeGenerics #structWhereClause {}

                impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClause {}
        
                impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        return formatter
                            .debug_tuple(stringify!(#structIdent))
                            #( .field(&self.#fieldIndices) )*
                            .finish();
                    }
                }
        
                impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClause {
                    fn clone(&self) -> Self {
                        return Self( #( self.#fieldIndices.clone(), )* );
                    }
                }
        
                impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics #structWhereClause {
                    fn eq(&self, other: &Self) -> bool {
                        return #( self.#fieldIndices == other.#fieldIndices && )* true;
                    }
                }
        
                impl #structImplGenerics Eq for #structIdent #structTypeGenerics #structWhereClause {}

                impl #structImplGenerics std::hash::Hash for #structIdent #structTypeGenerics #structWhereClause {
                    fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
                        #( self.#fieldIndices.hash(state); )*
                    }
                }
            };
        },
        syn::Fields::Unit => {
            return quote! {
                impl #structImplGenerics axiom::interfaces::ddd::domain::Identifier for #structIdent #structTypeGenerics #structWhereClause {}

                impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClause {}

                impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        return formatter
                            .debug_struct(stringify!(#structIdent))
                            .finish();
                    }
                }

                impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClause {
                    fn clone(&self) -> Self {
                        return Self;
                    }
                }

                impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics #structWhereClause {
                    fn eq(&self, other: &Self) -> bool {
                        return true;
                    }
                }

                impl #structImplGenerics Eq for #structIdent #structTypeGenerics #structWhereClause {}

                impl #structImplGenerics std::hash::Hash for #structIdent #structTypeGenerics #structWhereClause {
                    fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {}
                }
            };
        },
    }
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, enumWhereClause) = ast.generics.split_for_impl();

    let variantDebugImpls = variants
        .iter()
        .map(|variant| -> proc_macro2::TokenStream {
            let variantIdent = &variant.ident;

            match &variant.fields {
                syn::Fields::Named(fields) => {
                    let fieldIdents = fields.named
                        .iter()
                        .map(|field| &field.ident)
                        .collect::<Vec<_>>();

                    return quote! {
                        Self::#variantIdent { #( #fieldIdents, )* } => formatter
                            .debug_struct(stringify!(#enumIdent))
                            #( .field(stringify!(#fieldIdents), #fieldIdents) )*
                            .finish()
                    };
                },
                syn::Fields::Unnamed(fields) => {
                    let fieldIdents = (0..fields.unnamed.len())
                        .map(|index| format_ident!("arg{index}"))
                        .collect::<Vec<_>>();

                    return quote! {
                        Self::#variantIdent(#( #fieldIdents, )*) => formatter
                            .debug_tuple(stringify!(#enumIdent))
                            #( .field(#fieldIdents) )*
                            .finish()
                    };
                }
                syn::Fields::Unit => {
                    return quote! {
                        Self::#variantIdent => write!(formatter, stringify!(#variantIdent))
                    };
                },
            }
        })
        .collect::<Vec<_>>();

    let variantCloneImpls = variants
        .iter()
        .map(|variant| {
            let variantIdent = &variant.ident;

            match &variant.fields {
                syn::Fields::Named(fields) => {
                    let fieldIdents = fields.named
                        .iter()
                        .map(|field| &field.ident)
                        .collect::<Vec<_>>();

                    return quote! {
                        Self::#variantIdent { #( #fieldIdents, )* } => 
                            Self::#variantIdent { #( #fieldIdents: #fieldIdents.clone(), )* }
                    };
                },
                syn::Fields::Unnamed(fields) => {
                    let fieldIdents = (0..fields.unnamed.len())
                        .map(|index| format_ident!("arg{index}"))
                        .collect::<Vec<_>>();

                    return quote! {
                        Self::#variantIdent(#( #fieldIdents, )*) => 
                            Self::#variantIdent(#( #fieldIdents.clone(), )*)
                    };
                },
                syn::Fields::Unit => {
                    return quote! {
                        Self::#variantIdent => Self::#variantIdent
                    };
                },
            }
        })
        .collect::<Vec<_>>();

    let variantPartialEqImpls = variants
        .iter()
        .map(|variant| {
            let variantIdent = &variant.ident;

            match &variant.fields {
                syn::Fields::Named(fields) => {
                    let fieldIdents = fields.named
                        .iter()
                        .map(|field| &field.ident)
                        .collect::<Vec<_>>();
                    let lhsFieldIdents = fieldIdents
                        .iter()
                        .map(|ident| format_ident!("{}Lhs", ident.as_ref().unwrap()))
                        .collect::<Vec<_>>();
                    let rhsFieldIdents = fieldIdents
                        .iter()
                        .map(|ident| format_ident!("{}Rhs", ident.as_ref().unwrap()))
                        .collect::<Vec<_>>();
                    
                    return quote! {
                        (
                            Self::#variantIdent { #( #fieldIdents: #lhsFieldIdents, )* },
                            Self::#variantIdent { #( #fieldIdents: #rhsFieldIdents, )* },
                        ) => {
                            return #( #lhsFieldIdents == #rhsFieldIdents && )* true;
                        }
                    };
                },
                syn::Fields::Unnamed(fields) => {
                    let fieldIdents = (0..fields.unnamed.len())
                        .map(|index| format_ident!("arg{index}"))
                        .collect::<Vec<_>>();
                    let lhsFieldIdents: Vec<_> = fieldIdents
                        .iter()
                        .map(|ident| quote::format_ident!("{}Lhs", ident))
                        .collect::<Vec<_>>();
                    let rhsFieldIdents: Vec<_> = fieldIdents
                        .iter()
                        .map(|ident| quote::format_ident!("{}Rhs", ident))
                        .collect::<Vec<_>>();
                    
                    return quote! {
                        (
                            Self::#variantIdent(#( #lhsFieldIdents, )*),
                            Self::#variantIdent(#( #rhsFieldIdents, )*)
                        ) => {
                            return #( #lhsFieldIdents == #rhsFieldIdents && )* true;
                        }
                    };
                },
                syn::Fields::Unit => {
                    return quote! {
                        (Self::#variantIdent, Self::#variantIdent) => true
                    };
                },
            }
        })
        .collect::<Vec<_>>();

    return quote! {
        impl #enumImplGenerics axiom::interfaces::ddd::domain::ValueObject for #enumIdent #enumTypeGenerics #enumWhereClause {}

        impl #enumImplGenerics std::fmt::Debug for #enumIdent #enumTypeGenerics #enumWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    #( #variantDebugImpls, )*
                };
            }
        }

        impl #enumImplGenerics Clone for #enumIdent #enumTypeGenerics #enumWhereClause {
            fn clone(&self) -> Self {
                return match self {
                    #( #variantCloneImpls, )*
                };
            }
        }

        impl #enumImplGenerics PartialEq for #enumIdent #enumTypeGenerics #enumWhereClause {
            fn eq(&self, other: &Self) -> bool {
                return match (self, other) {
                    #( #variantPartialEqImpls, )*
                    _ => false,
                };
            }
        }

        impl #enumImplGenerics Eq for #enumIdent #enumTypeGenerics #enumWhereClause {}
    };
}