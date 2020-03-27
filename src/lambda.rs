use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token, Error, Expr, Ident, Pat, Result, Token,
};

pub(super) fn proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lambda = parse_macro_input!(input as Lambda);
    let lambda_tokens = quote! {
        #lambda
    };
    proc_macro::TokenStream::from(lambda_tokens)
}

#[derive(Debug)]
struct Lambda {
    or1_token: Token![|],
    inputs: Punctuated<Pat, Token![,]>,
    or2_token: Token![|],
    body: Box<Expr>,
}

impl Parse for Lambda {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut explicit_args = HashSet::new();
        let tokens = replace(input, &mut explicit_args)?;

        let or1_token = Token![|](Span::call_site());
        let inputs = {
            let count = explicit_args.iter().max().map_or(0, |v| v + 1);
            (0..count)
                .map(|index| -> Pat {
                    if explicit_args.contains(&index) {
                        let ident = Ident::new(&format!("_{}", index), Span::call_site());
                        parse_quote!(#ident)
                    } else {
                        parse_quote!(_)
                    }
                })
                .collect()
        };
        let or2_token = Token![|](Span::call_site());
        let body = parse_quote!(#tokens);
        Ok(Self {
            or1_token,
            inputs,
            or2_token,
            body,
        })
    }
}

impl ToTokens for Lambda {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.or1_token.to_tokens(tokens);
        self.inputs.to_tokens(tokens);
        self.or2_token.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

fn replace(input: ParseStream, explicit_args: &mut HashSet<usize>) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();
    let mut content;
    while !input.is_empty() {
        if input.peek(token::Paren) {
            let paren_token = parenthesized!(content in input);
            let token_tree = replace(&content, explicit_args)?;
            paren_token.surround(&mut tokens, |tokens| {
                token_tree.to_tokens(tokens);
            });
        } else if input.peek(Token![$]) {
            let _dollar_token = input.parse::<Token![$]>()?;
            let span = input.span();
            let token_tree = input.parse::<TokenTree>()?;
            let index = token_tree
                .to_string()
                .parse::<usize>()
                .map_err(|err| Error::new(span, err))?;
            let ident = Ident::new(&format!("_{}", index), Span::call_site());
            tokens.extend(quote!(#ident));
            explicit_args.insert(index);
        } else {
            let token_tree = input.parse::<TokenTree>()?;
            tokens.extend(Some(token_tree));
        }
    }
    Ok(tokens)
}
