use quote::quote;

pub fn deriveNewType(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => deriveForStruct(&ast, data),
        _ => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}

fn deriveForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    let syn::Fields::Unnamed(fields) = fields else {
        panic!("Newtypes must be a single-field tuple struct")
    };

    if fields.unnamed.len() != 1 {
        panic!("Newtypes must be a single-field tuple struct")
    };

    let field = fields.unnamed.first().unwrap();

    return deriveForSingleFieldUnnamedStruct(ast, field);
}

fn deriveForSingleFieldUnnamedStruct(ast: &syn::DeriveInput, field: &syn::Field) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();

    let fieldType = &field.ty;

    return quote! {
        impl #structImplGenerics std::ops::Deref for #structIdent #structTypeGenerics #structWhereClause {
            type Target = #fieldType;

            fn deref(&self) -> &Self::Target {
                return &self.0;
            }
        }

        impl #structImplGenerics std::ops::DerefMut for #structIdent #structTypeGenerics #structWhereClause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                return &mut self.0;
            }
        }
    }
}
