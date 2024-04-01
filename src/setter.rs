use quote::{format_ident, quote};
use syn::DeriveInput;
use syn::spanned::Spanned;
use crate::TokenStream1;
use crate::util::StructContext;

pub(crate) fn gen_ast(input: &DeriveInput) -> TokenStream1 {
    let context = StructContext::new(&input);
    let impl_sign = context.impl_sign();
    let fields = context.handle_fields_exclude_and_name_attr();

    let gen_fns = fields.iter().map(|fc| {
        let var_name = format_ident!("{}",&fc.name);
        let ty = fc.ty;
        let ident = fc.ident;
        let mut fn_name = format_ident!("set_{}",&var_name);
        fn_name.set_span(fc.ident.span());
        quote! {
             #[inline]
            pub fn #fn_name (&mut self,#var_name:#ty){
                self.#ident = #var_name;
            }
        }
    }).collect::<Vec<_>>();

    let ast = quote! {
        #impl_sign {
            #(#gen_fns)*
        }
    };
    ast.into()
}