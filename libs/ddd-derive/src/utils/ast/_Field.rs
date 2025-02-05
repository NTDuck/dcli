#[derive(darling::FromField)]
#[darling(attributes(ddd))]
#[allow(dead_code)]
pub struct Field<Attributes = darling::util::Ignored>
where
    Attributes: darling::FromMeta,
{
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    
    pub attributes: Option<Attributes>,
}
