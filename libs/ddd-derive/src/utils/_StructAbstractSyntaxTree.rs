use crate::utils::AbstractSyntaxTree;
use crate::utils::Field;

#[allow(dead_code)]
pub struct StructAbstractSyntaxTree<Field>
where
    Field: darling::FromField,
{
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub fields: darling::ast::Fields<Field>,
}

impl<Variant, Field> From<AbstractSyntaxTree<Variant, Field>> for StructAbstractSyntaxTree<Field>
where 
    Variant: darling::FromVariant,
    Field: darling::FromField,
{
    fn from(ast: AbstractSyntaxTree<Variant, Field>) -> Self {
        assert!(ast.data.is_struct());
        
        return Self {
            attrs: ast.attrs,
            vis: ast.vis,
            ident: ast.ident,
            generics: ast.generics,
            fields: ast.data
                .take_struct().unwrap(),
        }
    }
}

impl<Attribute> StructAbstractSyntaxTree<Field<Attribute>>
where
    Attribute: darling::FromMeta,
{
    pub fn get_field_idents(&self) -> Vec<syn::Member> {
        return match self.is_named_struct() {
            true => self.generate_fields_for_named_struct(),
            false => self.generate_fields_for_unnamed_struct(),
        };
    }

    fn is_named_struct(&self) -> bool {
        return self.fields
            .iter()
            .all(|field| field.ident.is_some());
    }
    
    fn generate_fields_for_named_struct(&self) -> Vec<syn::Member> {
        return self.fields
            .iter()
            .filter_map(|field| field.ident.clone())
            .map(|ident| syn::Member::Named(ident))
            .collect();
    }
    
    fn generate_fields_for_unnamed_struct(&self) -> Vec<syn::Member> {
        return (0..self.fields.len())
            .map(|index| syn::Index::from(index))
            .map(|index| syn::Member::Unnamed(index))
            .collect();
    }
}

impl StructAbstractSyntaxTree<syn::Field> {
    pub fn get_field_idents(&self) -> Vec<syn::Member> {
        return match self.is_named_struct() {
            true => self.generate_fields_for_named_struct(),
            false => self.generate_fields_for_unnamed_struct(),
        };
    }

    fn is_named_struct(&self) -> bool {
        return self.fields
            .iter()
            .all(|field| field.ident.is_some());
    }
    
    fn generate_fields_for_named_struct(&self) -> Vec<syn::Member> {
        return self.fields
            .iter()
            .filter_map(|field| field.ident.clone())
            .map(|ident| syn::Member::Named(ident))
            .collect();
    }
    
    fn generate_fields_for_unnamed_struct(&self) -> Vec<syn::Member> {
        return (0..self.fields.len())
            .map(|index| syn::Index::from(index))
            .map(|index| syn::Member::Unnamed(index))
            .collect();
    }
}
