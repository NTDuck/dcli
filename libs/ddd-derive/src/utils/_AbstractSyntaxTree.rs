use crate::utils::TokenStream;

#[derive(darling::FromDeriveInput)]
#[darling(supports(any))]
pub struct AbstractSyntaxTree<Variant, Field>
where 
    Variant: darling::FromVariant,
    Field: darling::FromField,
{
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub vis: syn::Visibility,

    pub data: darling::ast::Data<Variant, Field>,
}

impl<Variant, Field> TryFrom<TokenStream> for AbstractSyntaxTree<Variant, Field>
where 
    Variant: darling::FromVariant,
    Field: darling::FromField,
{
    type Error = darling::Error;

    fn try_from(tokens: TokenStream) -> Result<Self, Self::Error> {
        use darling::FromDeriveInput;

        let derive_input = syn::parse(tokens)?;
        return Self::from_derive_input(&derive_input);
    }
}
