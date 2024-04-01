use quote::{format_ident, quote};
use syn::DeriveInput;
use syn::spanned::Spanned;
use crate::TokenStream1;
use crate::util::StructContext;

pub(crate) fn gen_ast(input: &DeriveInput) -> TokenStream1 {
    let struct_name = format_ident!("{}Builder",&input.ident);
    let source_struct_name = &input.ident;

    let context = StructContext::new(&input);
    let vis = context.vis;
    let generics = context.generics;
    let where_case = context.where_case;
    let impl_sign = context.impl_sign();

    let fields = context.fields;

    let field_idents = fields.iter().map(|f| {
        f.ident.as_ref()
    }).collect::<Vec<_>>();
    let field_tys = fields.iter().map(|f| {
        &f.ty
    }).collect::<Vec<_>>();
    let field_expects = fields.iter().map(|f| {
        format!("{} field is not set in {} struct", f.ident.as_ref().unwrap(), source_struct_name)
    }).collect::<Vec<_>>();


    let ast = quote! {
        #vis struct #struct_name #generics #where_case {
            #(
                #field_idents: Option<#field_tys>,
            )*
        }


        impl #generics #struct_name #generics {

            #[inline]
            pub fn new() -> Self {
                Self {
                    #(
                        #field_idents: None,
                    )*
                }
            }

            #[inline]
            pub fn build(self) -> #source_struct_name #generics{
                #source_struct_name {
                    #(
                        #field_idents: self.#field_idents.expect(#field_expects),
                    )*
                }
            }
            #(
                #[inline]
                pub fn #field_idents (mut self,v:#field_tys) -> Self {
                    self.#field_idents = Some(v);
                    self
                }
            )*

        }

       #impl_sign {
            #[inline]
            pub fn builder() -> #struct_name #generics{
                #struct_name::new()
            }
        }
    };
    ast.into()
}
