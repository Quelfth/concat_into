use std::env;

use proc_macro::{Literal, Punct, Spacing, TokenStream, TokenTree};

fn parse_string_literal(lit: TokenTree) -> String {
    let s = lit.to_string();

    let first = s.find('"').expect("expected string literal");
    let last = s.rfind('"').expect("expected string literal");

    let prefix = &s[..first];
    let content = &s[first + 1..last];
    let suffix = &s[last + 1..];

    if !prefix.chars().all(|c| c == 'r' || c == '#') {
        panic!("only normal and raw string literals are supported");
    }

    if !suffix.chars().all(|c| c == '#') {
        panic!("string suffixes are not supported");
    }

    content.to_string()
}
#[proc_macro]
pub fn concat_into(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();

    let mut result = String::new();

    loop {
        let token = iter.next().expect("expected string literal or =>");

        match token {
            TokenTree::Punct(p) if p.as_char() == '=' && p.spacing() == Spacing::Joint => {
                match iter.next() {
                    Some(TokenTree::Punct(p2)) if p2.as_char() == '>' => break,
                    _ => panic!("expected =>"),
                }
            }
            TokenTree::Ident(i) => result.push_str(&env::var(i.to_string()).unwrap()),
            token => {
                result.push_str(&parse_string_literal(token));
            }
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
