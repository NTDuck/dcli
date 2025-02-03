use crate::utils::*;

pub struct DefaultEnumPayload {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub variants: Vec<syn::Variant>,
}

impl From<DefaultPayload> for DefaultEnumPayload {
    fn from(payload: DefaultPayload) -> Self {
        return Self {
            ident: payload.ident,
            generics: payload.generics,
            variants: payload.data
                .take_enum().unwrap(),
        };
    }
}
