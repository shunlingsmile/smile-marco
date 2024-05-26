use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Field, Fields, Generics, Token, Type, Visibility, WhereClause};

use crate::TokenStream2;

pub struct FieldContext<'a> {
    // field type
    pub ty: &'a Type,
    // field name
    pub ident: Option<&'a Ident>,
    // field new name
    pub name: String,
}

impl<'a> FieldContext<'a> {
    pub fn new(ty: &'a Type, ident: Option<&'a Ident>, name: String) -> Self {
        Self { ty, ident, name }
    }
}

pub(crate) struct StructContext<'a> {
    pub vis: &'a Visibility,
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub where_case: Option<&'a WhereClause>,
    pub fields: &'a Punctuated<Field, Token![,]>,
}

impl<'a> StructContext<'a> {
    pub fn new(input: &'a DeriveInput) -> Self {
        let vis = &input.vis;
        let ident = &input.ident;
        let generics = &input.generics;
        let where_case = generics.where_clause.as_ref();
        let fields = match &input.data {
            syn::Data::Struct(s) => &s.fields,
            _ => panic!("not a struct"),
        };
        let fields = match fields {
            Fields::Named(f) => &f.named,
            _ => panic!("not a named struct"),
        };
        Self {
            vis,
            ident,
            generics,
            where_case,
            fields,
        }
    }
    #[allow(unused_doc_comments)]
    /// Returns the block signature of impl
    pub fn impl_sign(&self) -> TokenStream2 {
        let ident = self.ident;
        let generics = self.generics;
        let where_case = &generics.where_clause;
        quote! {
            impl #generics #ident #generics #where_case
        }
    }
    // This method deals specifically with the processing of exclude and name attributes

    pub fn handle_fields_exclude_and_name_attr(&self) -> Vec<FieldContext> {
        let fields = self
            .fields
            .iter()
            .filter(|f| {
                for attr in &f.attrs {
                    if let syn::Meta::Path(ref path) = attr.meta {
                        if path.is_ident("exclude") {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|f| {
                for attr in &f.attrs {
                    if let syn::Meta::List(ref list) = attr.meta {
                        let option = list.path.segments.first();
                        if let Some(v) = option {
                            if v.ident.to_string().eq("name") {
                                if list.tokens.is_empty() {
                                    panic!("name is empty");
                                } else {
                                    let new_attr_name = list.tokens.to_string();
                                    if !new_attr_name.contains(",") {
                                        return FieldContext::new(
                                            &f.ty,
                                            f.ident.as_ref(),
                                            new_attr_name,
                                        );
                                    } else {
                                        panic!(
                                            "Only one value is required for the name on property"
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
                let attr_name = f
                    .ident
                    .as_ref()
                    .expect("attr cannot be without signatures")
                    .to_string();
                FieldContext::new(&f.ty, f.ident.as_ref(), attr_name)
            })
            .collect::<Vec<_>>();
        fields
    }
}

pub(crate) mod token_tree_utils {
    use crate::TokenStream1;
    use proc_macro2::TokenTree;

    #[inline]
    pub(crate) fn punct_eq<'a>(punct: Option<&'a TokenTree>, eq: &'a str) -> bool {
        if let Some(TokenTree::Punct(ref v)) = punct {
            return v.to_string().eq(eq);
        }
        false
    }

    #[inline]
    pub(crate) fn is_struct(input: &TokenStream1) -> bool {
        for x in input.clone().into_iter() {
            if let proc_macro::TokenTree::Ident(ref v) = x {
                return v.to_string().eq("struct");
            }
        }
        false
    }
}
