use layout::class_mod;

class_mod!(pub _TokenStream);
class_mod!(pub _AbstractSyntaxTree);

pub fn is_named_struct(fields: &darling::ast::Fields<syn::Field>) -> bool {
    return fields
        .iter()
        .all(|field| field.ident.is_some());
}
