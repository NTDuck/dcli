use quote::quote;

use crate::utils::ast::*;

pub fn deriveSerializeForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => deriveSerializeForOrdinaryStruct(ast, fields),
        syn::Fields::Unnamed(fields) => deriveSerializeForTupleStruct(ast, fields),
        syn::Fields::Unit => deriveSerializeForUnitStruct(ast),
    };
}

fn deriveSerializeForOrdinaryStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithSerializeBounds = generateWhereClauseWithSerializeBoundsFromDeriveInput(ast);

    let fieldIdents = getFieldIdentsFromNamedFields(fields);
    let fieldCount = fields.named.len();

    return quote! {
        impl #structImplGenerics serde::Serialize for #structIdent #structTypeGenerics #structWhereClauseWithSerializeBounds {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;

                let mut state = serializer.serialize_struct(stringify!(#structIdent), #fieldCount)?;
                #( state.serialize_field(stringify!(#fieldIdents), &self.#fieldIdents)?; )*
                return state.end();
            }
        }
    };
}

fn deriveSerializeForTupleStruct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithSerializeBounds = generateWhereClauseWithSerializeBoundsFromDeriveInput(ast);

    let fieldIndices = getFieldIndicesFromUnnamedFields(fields);
    let fieldCount = fieldIndices.len();

    if fieldCount == 1 {
        return quote! {
            impl #structImplGenerics serde::Serialize for #structIdent #structTypeGenerics #structWhereClauseWithSerializeBounds {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeTupleStruct;
    
                    return serializer.serialize_newtype_struct(stringify!(#structIdent), &self.0);
                }
            }
        };
    } else {
        return quote! {
            impl #structImplGenerics serde::Serialize for #structIdent #structTypeGenerics #structWhereClauseWithSerializeBounds {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeTupleStruct;
    
                    let mut state = serializer.serialize_tuple_struct(stringify!(#structIdent), #fieldCount)?;
                    #( state.serialize_field(&self.#fieldIndices)?; )*
                    return state.end();
                }
            }
        };
    }
}

fn deriveSerializeForUnitStruct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithSerializeBounds = generateWhereClauseWithSerializeBoundsFromDeriveInput(ast);

    return quote! {
        impl #structImplGenerics serde::Serialize for #structIdent #structTypeGenerics #structWhereClauseWithSerializeBounds {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                return serializer.serialize_unit_struct(stringify!(#structIdent));
            }
        }
    };
}

pub fn deriveSerializeForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();
    let enumWhereClauseWithSerializeBounds = generateWhereClauseWithSerializeBoundsFromDeriveInput(ast);

    let variantSerializeImpls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => deriveSerializeForStructVariant(ast, variant, fields),
            syn::Fields::Unnamed(fields) => deriveSerializeForTupleVariant(ast, variant, fields),
            syn::Fields::Unit => deriveSerializeForUnitVariant(ast, variant),
        })
        .collect::<Vec<_>>();

    if variants.is_empty() {
        return quote! {
            impl #enumImplGenerics serde::Serialize for #enumIdent #enumTypeGenerics #enumWhereClauseWithSerializeBounds {
                fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    match *self {}
                }
            }
        };
    } else {
        return quote! {
            impl #enumImplGenerics serde::Serialize for #enumIdent #enumTypeGenerics #enumWhereClauseWithSerializeBounds {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    match self {
                        #( #variantSerializeImpls, )*
                    }
                }
            }
        };
    }
}

fn deriveSerializeForStructVariant(ast: &syn::DeriveInput, variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;

    let variantIdent = &variant.ident;
    let variantIndex = getVariantIndex(ast, variant).unwrap();

    let fieldIdents = getFieldIdentsFromNamedFields(fields);
    let fieldCount = fields.named.len();

    return quote! {
        Self::#variantIdent { #( #fieldIdents, )* } => {
            use serde::ser::SerializeStructVariant;
            
            let mut state = serializer.serialize_struct_variant(
                stringify!(#enumIdent),
                #variantIndex,
                stringify!(#variantIdent),
                #fieldCount,
            )?;
            #( state.serialize_field(stringify!(#fieldIdents), #fieldIdents)?; )*
            return state.end();
        }
    };
}

fn deriveSerializeForTupleVariant(ast: &syn::DeriveInput, variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;

    let variantIdent = &variant.ident;
    let variantIndex = getVariantIndex(ast, variant).unwrap();

    let fieldIdents = getFormattedFieldsIdentsFromUnnamedFields(fields);
    let fieldCount = fields.unnamed.len();

    if fieldCount == 1 {
        return quote! {
            Self::#variantIdent(arg) => {
                use serde::ser::SerializeTupleVariant;
                
                return serializer.serialize_newtype_variant(
                    stringify!(#enumIdent),
                    #variantIndex,
                    stringify!(#variantIdent),
                    &arg,
                );
            }
        };
    } else {
        return quote! {
            Self::#variantIdent(#( #fieldIdents, )*) => {
                use serde::ser::SerializeTupleVariant;
                
                let mut state = serializer.serialize_tuple_variant(
                    stringify!(#enumIdent),
                    #variantIndex,
                    stringify!(#variantIdent),
                    #fieldCount,
                )?;
                #( state.serialize_field(#fieldIdents)?; )*
                return state.end();
            }
        };
    }
}

fn deriveSerializeForUnitVariant(ast: &syn::DeriveInput, variant: &syn::Variant) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    
    let variantIdent = &variant.ident;
    let variantIndex = getVariantIndex(ast, variant).unwrap();

    return quote! {
        Self::#variantIdent => serializer.serialize_unit_variant(
            stringify!(#enumIdent),
            #variantIndex,
            stringify!(#variantIdent),
        )
    };
}

fn generateWhereClauseWithSerializeBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: serde::Serialize
        },
        ast,
    );
}

fn getVariantIndex(ast: &syn::DeriveInput, variant: &syn::Variant) -> Option<u32> {
    if let syn::Data::Enum(data) = &ast.data {
        return data.variants
            .iter()
            .position(|v| v.ident == variant.ident)
            .map(|index| index as u32);
    } else {
        return None;
    }
}
