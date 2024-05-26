use crate::util::StructContext;
use crate::TokenStream1;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::DeriveInput;

pub(crate) fn gen_ast(input: &DeriveInput) -> TokenStream1 {
    let context = StructContext::new(input);
    let impl_sign = context.impl_sign();
    let fields = context.handle_fields_exclude_and_name_attr();

    let gen_fns = fields
        .iter()
        .map(|fc| {
            let mut fn_name = format_ident!("with_{}", fc.name);
            fn_name.set_span(fc.ident.span());

            let ty = fc.ty;
            let ident = fc.ident;
            quote! {
                 #[inline]
                pub fn #fn_name<F> (mut self,func: F) -> Self
                    where F: FnOnce(#ty) -> #ty
                {
                    self.#ident = func(self.#ident);
                    self
                }
            }
        })
        .collect::<Vec<_>>();

    let ast = quote! {
        #impl_sign {
            #(#gen_fns)*
        }
    };
    ast.into()
}
