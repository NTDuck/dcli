use quote::quote;

use crate::utils::ast::*;

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
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();

    let structWhereClauseWithEntityBounds = generateWhereClauseWithEntityBoundsFromDeriveInput(ast);
    let structWhereClauseWithValueObjectBounds = generateWhereClauseWithValueObjectBoundsFromDeriveInput(ast);
    let structWhereClauseWithDebugBounds = generateWhereClauseWithDebugBoundsFromDeriveInput(ast);
    let structWhereClauseWithCloneBounds = generateWhereClauseWithCloneBoundsFromDeriveInput(ast);
    let structWhereClauseWithPartialEqBounds = generateWhereClauseWithPartialEqBoundsFromDeriveInput(ast);
    let structWhereClauseWithEqBounds = generateWhereClauseWithEqBoundsFromDeriveInput(ast);

    let fieldIdents = getFieldIdentsFromNamedFields(fields);

    let identifierField = getIdentifierFieldForNamedStruct(fields)
        .expect(&format!(
            "Struct `{}` must have one field implementing \
            `axiom::interfaces::ddd::domain::Identifier` \
            and annotated {}",
            structIdent.to_string(),
            AcceptedAttributes
                .iter()
                .map(|attr| format!("`#[axiom(attributes({attr}))]`"))
                .collect::<Vec<_>>()
                .join(", "),
        ));

    let identifierFieldIdent = &identifierField.ident;
    let identifierFieldType = &identifierField.ty;

    return quote! {
        impl #structImplGenerics axiom::interfaces::ddd::domain::Entity for #structIdent #structTypeGenerics #structWhereClauseWithEntityBounds {
            type Id = #identifierFieldType;

            #[inline(always)]
            fn getId(&self) -> &Self::Id {
                return &self.#identifierFieldIdent;
            }
        }

        impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClauseWithValueObjectBounds {}

        impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClauseWithDebugBounds {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_struct(stringify!(#structIdent))
                    #( .field(stringify!(#fieldIdents), &self.#fieldIdents) )*
                    .finish();
            }
        }

        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClauseWithCloneBounds {
            fn clone(&self) -> Self {
                return Self {
                    #( #fieldIdents: self.#fieldIdents.clone(), )*
                };
            }
        }

        impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics #structWhereClauseWithPartialEqBounds {
            fn eq(&self, other: &Self) -> bool {
                use axiom::interfaces::ddd::domain::Entity;
                
                return self.getId() == other.getId();
            }
        }

        impl #structImplGenerics Eq for #structIdent #structTypeGenerics #structWhereClauseWithEqBounds {}
    };
}

fn generateWhereClauseWithEntityBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: axiom::interfaces::ddd::domain::Entity
        },
        ast,
    );
}

fn generateWhereClauseWithValueObjectBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: axiom::interfaces::ddd::domain::ValueObject
        },
        ast,
    );
}

fn generateWhereClauseWithDebugBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: std::fmt::Debug
        },
        ast,
    );
}

fn generateWhereClauseWithCloneBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: Clone
        },
        ast,
    );
}

fn generateWhereClauseWithPartialEqBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: PartialEq
        },
        ast,
    );
}

fn generateWhereClauseWithEqBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: Eq
        },
        ast,
    );
}

fn getIdentifierFieldForNamedStruct(fields: &syn::FieldsNamed) -> Option<&syn::Field> {
    return fields.named.iter().find(|field| {
        field.attrs
            .iter()
            .filter_map(|attr| extractNestedMetaListsFromAttr(attr, "axiom"))
            .flat_map(|metaList| extractNestedMetaListsFromMetaList(&metaList, "attributes"))
            .flat_map(extractNestedAttrIdentsFromMetaList)
            .any(|attrIdent| AcceptedAttributes.contains(&attrIdent.as_str()))
    });
}

fn extractNestedMetaListsFromAttr(attr: &syn::Attribute, expectedAttrIdent: &str) -> Option<syn::MetaList> {
    if !attr.path().is_ident(expectedAttrIdent) {
        return None;
    }
        
    return attr.meta.require_list().ok().cloned()
}

fn extractNestedMetaListsFromMetaList(metaList: &syn::MetaList, expectedAttrIdent: &str) -> Vec<syn::MetaList> {
    return metaList
        .parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .map(|punctuated| punctuated
            .into_iter()
            .filter_map(|meta| meta
                .path()
                .is_ident(expectedAttrIdent)
                .then(|| meta.require_list().ok())
                .flatten()
                .cloned())
            .collect())
        .unwrap_or_default();
}

fn extractNestedAttrIdentsFromMetaList(metaList: syn::MetaList) -> Vec<String> {
    return metaList
        .parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)
        .ok()
        .map(|punctuated| punctuated
            .into_iter()
            .map(|path| path.segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join(SegmentSeparator))
            .collect())
        .unwrap_or_default();
}

const AcceptedAttributes: [&str; 3] = [
    "Identifier",
    "ddd::Identifier",
    "ddd::domain::Identifier",
];

const SegmentSeparator: &str = "::";
