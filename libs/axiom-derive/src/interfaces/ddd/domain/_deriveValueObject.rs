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
        .map(|variant| {
            let variantIdent = &variant.ident;

            match &variant.fields {
                syn::Fields::Unit => {
                    quote! {
                        #structIdent::#variantIdent => write!(formatter, stringify!(#variantIdent)),
                    }
                }
                _ => todo!(),
            }
        })
        .collect();

    let variantCloneImpls: Vec<_> = variants
        .iter()
        .map(|variant| {
            let variantIdent = &variant.ident;

            match &variant.fields {
                syn::Fields::Unit => {
                    quote! {
                        #structIdent::#variantIdent => #structIdent::#variantIdent,
                    }
                }
                _ => todo!(),
            }
        })
        .collect();

    let variantPartialEqImpls: Vec<_> = variants
        .iter()
        .map(|variant| {
            let variantIdent = &variant.ident;

            match &variant.fields {
                syn::Fields::Unit => {
                    quote! {
                        // (#structIdent::#variantIdent, #structIdent::#variantIdent) => core::mem::discriminant(self) == core::mem::discriminant(other),
                        (#structIdent::#variantIdent, #structIdent::#variantIdent) => true,
                    }
                }
                _ => todo!(),
            }
        })
        .collect();

    return quote! {
        impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClause {}

        impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    #(#variantDebugImpls)*
                    _ => unreachable!(),
                };
            }
        }

        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClause {
            fn clone(&self) -> Self {
                return match self {
                    #(#variantCloneImpls)*
                    _ => unreachable!(),
                };
            }
        }

        impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics #structWhereClause {
            fn eq(&self, other: &Self) -> bool {
                return match (self, other) {
                    #(#variantPartialEqImpls)*
                    _ => false,
                };
            }
        }

        impl #structImplGenerics Eq for #structIdent #structTypeGenerics #structWhereClause {}
    };
}