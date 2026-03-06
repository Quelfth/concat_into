#![feature(proc_macro_value)]

use proc_macro::{Literal, Punct, Spacing, TokenStream, TokenTree};

#[proc_macro]
pub fn concat_into(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();

    let mut result = String::new();

    loop {
        let token = iter.next().expect("expected string literal or =>");

        match token {
            TokenTree::Literal(lit) => {
                result.push_str(&lit.str_value().unwrap());
            }
            TokenTree::Punct(p) if p.as_char() == '=' && p.spacing() == Spacing::Joint => {
                match iter.next() {
                    Some(TokenTree::Punct(p2)) if p2.as_char() == '>' => break,
                    _ => panic!("expected =>"),
                }
            }
            _ => panic!("expected string literal"),
        }
    }

    let Some(TokenTree::Ident(macro_name)) = iter.next() else {
        panic!("expected macro name after =>")
    };

    let literal = Literal::string(&result);

    [
        TokenTree::Ident(macro_name),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(proc_macro::Group::new(
            proc_macro::Delimiter::Parenthesis,
            TokenStream::from(TokenTree::Literal(literal)),
        )),
    ]
    .into_iter()
    .collect()
}
