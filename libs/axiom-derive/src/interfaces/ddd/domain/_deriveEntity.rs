use quote::quote;

pub fn deriveEntity(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => deriveForStruct(&ast, data),
        _ => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}


fn deriveForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => deriveForNamedStruct(ast, fields),
        _ => panic!(),
    };
}

fn deriveForNamedStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();

    let fieldIdents = fields.named
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<_>>();

    let identifierField = getIdentifierFieldForNamedStruct(fields)
        .expect("Missing field Identifier");
    let identifierFieldIdent = &identifierField.ident;
    let identifierFieldType = &identifierField.ty;

    return quote! {
        impl #structImplGenerics axiom::interfaces::ddd::domain::Entity for #structIdent #structTypeGenerics #structWhereClause {
            type Id = #identifierFieldType;

            #[inline(always)]
            fn getId(&self) -> &Self::Id {
                return &self.#identifierFieldIdent;
            }
        }

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
                use axiom::interfaces::ddd::domain::Entity;
                
                return self.getId() == other.getId();
            }
        }

        impl #structImplGenerics Eq for #structIdent #structTypeGenerics #structWhereClause {}
    };
}

fn getIdentifierFieldForNamedStruct(fields: &syn::FieldsNamed) -> Option<syn::Field> {
    const AcceptedFieldIdents: [&str; 2] = ["id", "identifier"];
    
    return fields.named
        .iter()
        .find(|field| field.ident.as_ref().map_or(false, |ident| AcceptedFieldIdents.contains(&ident.to_string().as_str())))
        .cloned();
}
