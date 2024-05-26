use crate::util::token_tree_utils;
use crate::{TokenStream1, TokenStream2};
use proc_macro2::TokenTree;
use quote::{format_ident, quote};

pub(crate) fn ast_gen(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    if !token_tree_utils::is_struct(&item) {
        return item;
    }
    let mut link = format_ident!("smile_marco");
    let mut need_derives = vec![
        "Getter".to_string(),
        "Setter".to_string(),
        "Builder".to_string(),
        "Wither".to_string(),
    ];
    let attr = TokenStream2::from(attr);
    let mut attr_iter = attr.into_iter();
    while let Some(TokenTree::Ident(ref ident)) = attr_iter.next() {
        if ident.to_string().eq("link")
            && token_tree_utils::punct_eq(attr_iter.next().as_ref(), "=")
        {
            if let Some(TokenTree::Literal(ref lit)) = attr_iter.next() {
                let v = lit.to_string();
                let v = v.replace("\"", "");
                link = format_ident!("{}", v);
            }
        } else if ident.to_string().eq("exclude")
            && token_tree_utils::punct_eq(attr_iter.next().as_ref(), "=")
        {
            if let Some(TokenTree::Group(ref group)) = attr_iter.next() {
                let exclude = group
                    .stream()
                    .into_iter()
                    .filter(|v| matches!(v, TokenTree::Literal(_) | TokenTree::Ident(_)))
                    .map(|v| {
                        let str = v.to_string();
                        str.replace("\"", "")
                    })
                    .collect::<Vec<_>>();
                let vec = need_derives
                    .iter()
                    .filter(|v| !exclude.contains(v))
                    .map(String::from)
                    .collect::<Vec<_>>();
                need_derives = vec;
            }
        }
    }

    let need_derives = need_derives
        .iter()
        .map(|v| format_ident!("{}", v))
        .collect::<Vec<_>>();

    let source_stream = TokenStream2::from(item);
    let ast = quote! {
        use #link::{
            #(#need_derives ,)*
        };
        #[derive(Getter, Setter, Wither, Builder)]
        #source_stream
    };
    ast.into()
}
