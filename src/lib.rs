#![forbid(missing_docs)]
#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::{parse_macro_input, Ident, LitStr, Token};

struct Args {
    since: Option<String>,
    note: Option<String>,
    remove: String,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = ["since", "note", "remove"]
            .into_iter()
            .map(|s| (String::from(s), None))
            .collect::<HashMap<_, _>>();
        while !input.is_empty() {
            let (ident, _, value) = (
                input.parse::<Ident>()?,
                input.parse::<Token![=]>()?,
                input.parse::<LitStr>()?,
            );
            match args.insert(ident.to_string(), Some(value.value())) {
                None => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unknown meta item '{ident}'"),
                    ))
                }
                Some(Some(_)) => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("duplicate '{ident}' items"),
                    ))
                }
                Some(None) => (),
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        if let Some(Some(remove)) = args.remove("remove") {
            Ok(Args {
                since: args.remove("since").unwrap(),
                note: args.remove("note").unwrap(),
                remove,
            })
        } else {
            Err(syn::Error::new(
                Span::call_site(),
                "mandatory 'remove' item missing",
            ))
        }
    }
}

impl ToTokens for Args {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let removal = format!("removal scheduled for version {}", self.remove);
        let note = match &self.note {
            Some(n) => format!("{n} ({removal})"),
            None => removal,
        };
        let args = if let Some(since) = &self.since {
            quote!(since = #since, note = #note)
        } else {
            quote!(note = #note)
        };
        tokens.extend(quote!(#[deprecated(#args)]));
    }
}

macro_rules! error {
    ($fmt:literal $(,$args:expr)*$(,)?) => {{
        let error = format!($fmt $(,$args)*);
        return quote! { compile_error!(#error); }.into();
    }}
}

/// Similar to Rust `deprecated` attribute, with the addition of a mandatory
/// `remove` attribute argument which contains a semver requirement at which
/// the item must be definitely removed from the source code.
///
/// See the [crate level documentation](self) for a complete description.
#[proc_macro_attribute]
pub fn deprecate_until(args: TokenStream, tokens: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let remove = match semver::VersionReq::parse(&args.remove) {
        Ok(c) => c,
        Err(e) => error!("version '{}' cannot be parsed: {e:?}", args.remove),
    };
    if let Ok(version) = std::env::var("CARGO_PKG_VERSION") {
        let version = match semver::Version::parse(&version) {
            Ok(v) => v,
            Err(e) => error!("unable to parse current version '{version}': {e:?}"),
        };
        if remove.matches(&version) {
            error!("version '{version}' matches '{remove}', item should be removed");
        }
    }
    let tokens: proc_macro2::TokenStream = tokens.into();
    quote!(#args #tokens).into()
}
