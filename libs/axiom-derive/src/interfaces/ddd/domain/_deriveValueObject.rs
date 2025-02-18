use quote::quote;

pub fn deriveValueObject(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    return match fields {
        syn::Fields::Named(fields) => {
            let fieldIdents: Vec<_> = fields.named
                .iter()
                .map(|field| &field.ident)
                .collect();
            let fieldTypes: Vec<_> = fields.named
                .iter()
                .map(|field| &field.ty)
                .collect();

            quote! {
                impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClause {}

                impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        return formatter.debug_struct(stringify!(#structIdent))
                            #(.field(stringify!(#fieldIdents), &self.#fieldIdents))*
                            .finish();
                    }
                }

                impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClause {
                    fn clone(&self) -> Self {
                        return Self {
                            #(#fieldIdents: self.#fieldIdents.clone()),*
                        };
                    }
                }

                impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics #structWhereClause {
                    fn eq(&self, other: &Self) -> bool {
                        return true #( && self.#fieldIdents == other.#fieldIdents)*;
                    }
                }

                impl #structImplGenerics Eq for #structIdent #structTypeGenerics #structWhereClause {}
            }
        },
        syn::Fields::Unnamed(fields) => {
            let fieldIndices: Vec<_> = (0..fields.unnamed.len())
                .map(syn::Index::from)
                .collect();
            let fieldTypes: Vec<_> = fields.unnamed
                .iter()
                .map(|field| &field.ty)
                .collect();

            quote! {
                impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClause {}
        
                impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        return formatter.debug_tuple(stringify!(#structIdent))
                            #(.field(&self.#fieldIndices))*
                            .finish();
                    }
                }
        
                impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClause {
                    fn clone(&self) -> Self {
                        return Self(
                            #(self.#fieldIndices.clone()),*
                        );
                    }
                }
        
                impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics #structWhereClause {
                    fn eq(&self, other: &Self) -> bool {
                        return true #( && self.#fieldIndices == other.#fieldIndices)*;
                    }
                }
        
                impl #structImplGenerics Eq for #structIdent #structTypeGenerics #structWhereClause {}
            }
        },
        syn::Fields::Unit => {
            quote! {
                impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClause {}

                impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        return formatter.debug_struct(stringify!(#structIdent))
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
            }
        },
    };
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();

    let variantDebugImpls: Vec<_> = variants
        .iter()
        .map(|variant| -> proc_macro2::TokenStream {
            let variantIdent = &variant.ident;

            match &variant.fields {
                syn::Fields::Named(fields) => {
                    let fieldIdents: Vec<_> = fields.named
                        .iter()
                        .map(|field| &field.ident)
                        .collect();

                    quote! {
                        #structIdent::#variantIdent {
                            #(#fieldIdents),*
                        } => formatter
                            .debug_struct(stringify!(#structIdent))
                            #(.field(stringify!(#fieldIdents), #fieldIdents))*
                            .finish(),
                    }
                },
                syn::Fields::Unnamed(fields) => {
                    let fieldIdents: Vec<_> = (0..fields.unnamed.len())
                        .map(|index| syn::Ident::new(
                            &format!("arg{index}"),
                            proc_macro2::Span::call_site(),
                        ))
                        .collect();

                    quote! {
                        #structIdent::#variantIdent(#(#fieldIdents),*) => formatter
                            .debug_tuple(stringify!(#structIdent))
                            #(.field(#fieldIdents))*
                            .finish(),
                    }
                }
                syn::Fields::Unit => {
                    quote! {
                        #structIdent::#variantIdent => write!(formatter, stringify!(#variantIdent)),
                    }
                },
            }
        })
        .collect();

    let variantCloneImpls: Vec<_> = variants
        .iter()
        .map(|variant| {
            let variantIdent = &variant.ident;

            match &variant.fields {
                syn::Fields::Named(fields) => {
                    let fieldIdents: Vec<_> = fields.named
                        .iter()
                        .map(|field| &field.ident)
                        .collect();

                    quote! {
                        Self::#variantIdent {
                            #(#fieldIdents),*
                        } => Self::#variantIdent {
                            #(#fieldIdents: #fieldIdents.clone(),)*
                        },
                    }
                },
                syn::Fields::Unnamed(fields) => {
                    let fieldIdents: Vec<_> = (0..fields.unnamed.len())
                        .map(|index| syn::Ident::new(
                            &format!("arg{index}"),
                            proc_macro2::Span::call_site(),
                        ))
                        .collect();

                    quote! {
                        Self::#variantIdent(#(#fieldIdents,)*) => 
                            Self::#variantIdent(#(#fieldIdents.clone(),)*),
                    }
                },
                syn::Fields::Unit => {
                    quote! {
                        #structIdent::#variantIdent => #structIdent::#variantIdent,
                    }
                },
            }
        })
        .collect();

    let variantPartialEqImpls: Vec<_> = variants
        .iter()
        .map(|variant| {
            let variantIdent = &variant.ident;

            match &variant.fields {
                syn::Fields::Named(fields) => {
                    let fieldIdents: Vec<_> = fields.named
                        .iter()
                        .map(|field| &field.ident)
                        .collect();
                    let lhsFieldIdents: Vec<_> = fieldIdents
                        .iter()
                        .map(|ident| quote::format_ident!("{}Lhs", ident.as_ref().unwrap()))
                        .collect();
                    let rhsFieldIdents: Vec<_> = fieldIdents
                        .iter()
                        .map(|ident| quote::format_ident!("{}Rhs", ident.as_ref().unwrap()))
                        .collect();
                    
                    quote! {
                        (Self::#variantIdent { #(#fieldIdents: #lhsFieldIdents,)* }, Self::#variantIdent { #(#fieldIdents: #rhsFieldIdents,)* }) => {
                            true #( && #lhsFieldIdents == #rhsFieldIdents)* 
                        },
                    }
                },
                syn::Fields::Unnamed(fields) => {
                    let fieldIdents: Vec<_> = (0..fields.unnamed.len())
                        .map(|index| syn::Ident::new(
                            &format!("arg{index}"),
                            proc_macro2::Span::call_site(),
                        ))
                        .collect();
                    let lhsFieldIdents: Vec<_> = fieldIdents
                        .iter()
                        .map(|ident| quote::format_ident!("{}Lhs", ident))
                        .collect();
                    let rhsFieldIdents: Vec<_> = fieldIdents
                        .iter()
                        .map(|ident| quote::format_ident!("{}Rhs", ident))
                        .collect();
                    
                    quote! {
                        (Self::#variantIdent(#(#lhsFieldIdents,)*), Self::#variantIdent(#(#rhsFieldIdents,)*)) => true #( && #lhsFieldIdents == #rhsFieldIdents)*,
                    }
                },
                syn::Fields::Unit => {
                    quote! {
                        (Self::#variantIdent, Self::#variantIdent) => true,
                    }
                },
            }
        })
        .collect();

    return quote! {
        impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClause {}

        impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    #(#variantDebugImpls)*
                };
            }
        }

        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClause {
            fn clone(&self) -> Self {
                return match self {
                    #(#variantCloneImpls)*
                };
            }
        }

        impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics #structWhereClause {
            fn eq(&self, other: &Self) -> bool {
                return match (self, other) {
                    #(#variantPartialEqImpls)*
                    _ => core::mem::discriminant(self) == core::mem::discriminant(other),
                };
            }
        }

        impl #structImplGenerics Eq for #structIdent #structTypeGenerics #structWhereClause {}
    };
}