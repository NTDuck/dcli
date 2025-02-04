#[derive(darling::FromDeriveInput)]
#[darling(supports(any))]
pub struct Payload<Variant, Field>
where 
    Variant: darling::FromVariant,
    Field: darling::FromField,
{
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: darling::ast::Data<Variant, Field>,
}
