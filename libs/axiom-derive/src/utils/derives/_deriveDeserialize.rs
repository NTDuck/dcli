use quote::quote;

use crate::utils::ast::*;

pub fn deriveDeserializeForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => deriveDeserializeForNamedStruct(ast, fields),
        syn::Fields::Unnamed(fields) => deriveDeserializeForTupleStruct(ast, fields),
        syn::Fields::Unit => deriveDeserializeForUnitStruct(ast),
    };
}

fn deriveDeserializeForNamedStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithDeserializeBounds = generateWhereClauseWithDeserializeBoundsFromDeriveInput(ast);

    let fieldIdents = getFieldIdentsFromNamedFields(fields);
    let fieldCount = fields.named.len();

    return quote! {
        impl #structImplGenerics serde::Deserialize<'de> for #structIdent #structTypeGenerics #structWhereClauseWithDeserializeBounds {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct StructVisitor;

                impl<'de> serde::de::Visitor<'de> for StructVisitor {
                    type Value = #structIdent;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!(#structIdent)))
                    }

                    fn visit_map<V>(self, mut map: V) -> Result<#structIdent, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        #(
                            let mut #fieldIdents = None;
                        )*

                        while let Some(key) = map.next_key::<String>()? {
                            match key.as_str() {
                                #(
                                    stringify!(#fieldIdents) => {
                                        if #fieldIdents.is_some() {
                                            return Err(serde::de::Error::duplicate_field(stringify!(#fieldIdents)));
                                        }
                                        #fieldIdents = Some(map.next_value()?);
                                    }
                                )*
                                _ => {
                                    return Err(serde::de::Error::unknown_field(&key, FIELDS));
                                }
                            }
                        }

                        #(
                            let #fieldIdents = #fieldIdents.ok_or_else(|| serde::de::Error::missing_field(stringify!(#fieldIdents)))?;
                        )*

                        Ok(#structIdent {
                            #(
                                #fieldIdents,
                            )*
                        })
                    }
                }

                const FIELDS: &[&str] = &[#( stringify!(#fieldIdents), )*];
                deserializer.deserialize_struct(stringify!(#structIdent), FIELDS, StructVisitor)
            }
        }
    };
}

fn deriveDeserializeForTupleStruct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let genericsWithDeserializeLifetime = generateGenericsWithDeserializeLifetimeFromDeriveInput(ast);
    let (structImplGenerics, ..) = genericsWithDeserializeLifetime.split_for_impl();
    let (_, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithDeserializeBounds = generateWhereClauseWithDeserializeBoundsFromDeriveInput(ast);

    let fieldCount = fields.unnamed.len();
    let indices = (0..fieldCount).collect::<Vec<_>>();

    if fieldCount == 1 {
        return quote! {
            impl #structImplGenerics serde::Deserialize<'de> for #structIdent #structTypeGenerics #structWhereClauseWithDeserializeBounds {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    Ok(#structIdent(serde::Deserialize::deserialize(deserializer)?))
                }
            }
        };
    } else {
        return quote! {
            impl #structImplGenerics serde::Deserialize<'de> for #structIdent #structTypeGenerics #structWhereClauseWithDeserializeBounds {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct TupleVisitor;

                    impl<'de> serde::de::Visitor<'de> for TupleVisitor {
                        type Value = #structIdent;

                        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            formatter.write_str(concat!("tuple struct ", stringify!(#structIdent)))
                        }

                        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
                        where
                            V: serde::de::SeqAccess<'de>,
                        {
                            Ok(#structIdent(
                                #(
                                    seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(#indices, &self))?,
                                )*
                            ))
                        }
                    }

                    deserializer.deserialize_tuple_struct(stringify!(#structIdent), #fieldCount, TupleVisitor)
                }
            }
        };
    }
}

fn deriveDeserializeForUnitStruct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let genericsWithDeserializeLifetime = generateGenericsWithDeserializeLifetimeFromDeriveInput(ast);
    let (structImplGenerics, ..) = genericsWithDeserializeLifetime.split_for_impl();
    let (_, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithDeserializeBounds = generateWhereClauseWithDeserializeBoundsFromDeriveInput(ast);

    return quote! {
        impl #structImplGenerics serde::Deserialize<'de> for #structIdent #structTypeGenerics #structWhereClauseWithDeserializeBounds {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct UnitStructVisitor;

                impl<'de> serde::de::Visitor<'de> for UnitStructVisitor {
                    type Value = #structIdent;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        return formatter.write_str(concat!("unit struct ", stringify!(#structIdent)));
                    }

                    fn visit_unit<E>(self) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        return Ok(#structIdent);
                    }
                }

                return deserializer.deserialize_unit_struct(stringify!(#structIdent), UnitStructVisitor);
            }
        }
    };
}

pub fn deriveDeserializeForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();
    let enumWhereClauseWithDeserializeBounds = generateWhereClauseWithDeserializeBoundsFromDeriveInput(ast);

    let variants = &data.variants;
    let variant_idents: Vec<_> = variants.iter().map(|v| &v.ident).collect();
    let variant_names: Vec<_> = variant_idents.iter().map(|i| quote!(stringify!(#i))).collect();

    let variant_arms = variants.iter().enumerate().map(|(idx, variant)| {
        let variant_ident = &variant.ident;
        let variant_name = quote!(stringify!(#variant_ident));

        match &variant.fields {
            syn::Fields::Named(fields) => {
                let field_idents = getFieldIdentsFromNamedFields(fields);
                let field_names: Vec<_> = field_idents.iter().map(|i| quote!(stringify!(#i))).collect();

                quote! {
                    #variant_name => {
                        struct FieldVisitor;

                        impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                            type Value = #enumIdent;

                            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                                formatter.write_str(concat!("struct variant ", #variant_name))
                            }

                            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                            where
                                A: serde::de::MapAccess<'de>,
                            {
                                #(
                                    let mut #field_idents = None;
                                )*

                                while let Some(key) = map.next_key::<String>()? {
                                    match key.as_str() {
                                        #(
                                            #field_names => {
                                                if #field_idents.is_some() {
                                                    return Err(serde::de::Error::duplicate_field(#field_names));
                                                }
                                                #field_idents = Some(map.next_value()?);
                                            },
                                        )*
                                        _ => return Err(serde::de::Error::unknown_field(&key, &[#(#field_names,)*])),
                                    }
                                }

                                #(
                                    let #field_idents = #field_idents.ok_or_else(|| {
                                        serde::de::Error::missing_field(#field_names)
                                    })?;
                                )*

                                Ok(#enumIdent::#variant_ident {
                                    #(#field_idents,)*
                                })
                            }
                        }

                        deserializer.deserialize_map(FieldVisitor)
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let field_count = fields.unnamed.len();
                if field_count == 1 {
                    quote! {
                        #variant_name => {
                            Ok(#enumIdent::#variant_ident(
                                serde::Deserialize::deserialize(deserializer)?
                            ))
                        }
                    }
                } else {
                    let field_indices = (0..field_count).collect::<Vec<_>>();
                    quote! {
                        #variant_name => {
                            struct TupleVisitor;

                            impl<'de> serde::de::Visitor<'de> for TupleVisitor {
                                type Value = #enumIdent;

                                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                                    formatter.write_str(concat!("tuple variant ", #variant_name))
                                }

                                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                                where
                                    A: serde::de::SeqAccess<'de>,
                                {
                                    Ok(#enumIdent::#variant_ident(
                                        #(
                                            seq.next_element()?.ok_or_else(|| {
                                                serde::de::Error::invalid_length(#field_indices, &self)
                                            })?,
                                        )*
                                    ))
                                }
                            }

                            deserializer.deserialize_tuple(#field_count, TupleVisitor)
                        }
                    }
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #variant_name => Ok(#enumIdent::#variant_ident)
                }
            }
        }
    });

    if variants.is_empty() {
        return quote! {
            impl #enumImplGenerics serde::Deserialize<'de> for #enumIdent #enumTypeGenerics #enumWhereClauseWithDeserializeBounds {
                fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    Err(serde::de::Error::custom("cannot deserialize empty enum"))
                }
            }
        };
    }

    return quote! {
        impl #enumImplGenerics serde::Deserialize<'de> for #enumIdent #enumTypeGenerics #enumWhereClauseWithDeserializeBounds {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct EnumVisitor;

                impl<'de> serde::de::Visitor<'de> for EnumVisitor {
                    type Value = #enumIdent;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(concat!("enum ", stringify!(#enumIdent)))
                    }

                    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::EnumAccess<'de>,
                    {
                        let (variant, variant_deserializer) = data.variant()?;
                        match variant.as_str() {
                            #( #variant_arms, )*
                            _ => Err(serde::de::Error::unknown_variant(
                                &variant,
                                &[#(#variant_names,)*]
                            )),
                        }
                    }
                }

                deserializer.deserialize_enum(
                    stringify!(#enumIdent),
                    &[#(#variant_names,)*],
                    EnumVisitor
                )
            }
        }
    };
}

fn generateGenericsWithDeserializeLifetimeFromDeriveInput(ast: &syn::DeriveInput) -> syn::Generics {
    let mut generics = ast.generics.clone();

    let deserializeLifetimeParam: syn::LifetimeParam = syn::parse_quote!('de);
    generics.params.insert(0, syn::GenericParam::Lifetime(deserializeLifetimeParam));

    return generics;
}

fn generateWhereClauseWithDeserializeBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: for<'de> serde::Deserialize<'de>
        },
        ast,
    );
}
