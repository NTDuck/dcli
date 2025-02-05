#[derive(darling::FromField)]
#[darling(attributes(ddd))]
pub struct Field<Attributes = ()> {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    
    pub attributes: Option<Attributes>,
}
