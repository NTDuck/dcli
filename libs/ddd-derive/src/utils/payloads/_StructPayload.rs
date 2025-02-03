use crate::utils::*;

pub struct DefaultStructPayload {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub fields: Vec<syn::Member>,
}

impl From<(DefaultPayload, darling::ast::Fields<syn::Field>)> for DefaultStructPayload {
    fn from((payload, fields): (DefaultPayload, darling::ast::Fields<syn::Field>)) -> Self {
        let fields = match Self::is_named_struct(&fields) {
            true => Self::generate_fields_for_named_struct(&fields),
            false => Self::generate_fields_for_unnamed_struct(&fields),
        };

        return Self {
            ident: payload.ident,
            generics: payload.generics,
            fields,
        };
    }
}

impl DefaultStructPayload {
    fn is_named_struct(fields: &darling::ast::Fields<syn::Field>) -> bool {
        return fields
            .iter()
            .all(|field| field.ident.is_some());
    }
    
    fn generate_fields_for_named_struct(fields: &darling::ast::Fields<syn::Field>) -> Vec<syn::Member> {
        return fields
            .iter()
            .filter_map(|field| field.ident.clone())
            .map(|ident| syn::Member::Named(ident))
            .collect();
    }

    fn generate_fields_for_unnamed_struct(fields: &darling::ast::Fields<syn::Field>) -> Vec<syn::Member> {
        return (0..fields.len())
            .map(|index| syn::Index::from(index))
            .map(|index| syn::Member::Unnamed(index))
            .collect();
    }
}
