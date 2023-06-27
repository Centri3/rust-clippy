use convert_case::Case;
use convert_case::Casing;
use itertools::Itertools;
use proc_macro2::Ident as Ident2;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree as TokenTree2;
use quote::quote;
use std::str::FromStr;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Error;
use syn::Fields;
use syn::GenericParam;
use syn::LitStr;
use syn::Meta;
use syn::Result;
use syn::Token;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(lint);
    custom_keyword!(version);
    custom_keyword!(desc);
}

#[derive(Clone, Copy, Debug)]
pub enum LintCategory {
    Correctness,
    Suspicious,
    Style,
    Complexity,
    Perf,
    Pedantic,
    Restriction,
    Nursery,
    Cargo,
    Internal,
}

impl FromStr for LintCategory {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "correctness" => Ok(Self::Correctness),
            "suspicious" => Ok(Self::Suspicious),
            "style" => Ok(Self::Style),
            "complexity" => Ok(Self::Complexity),
            "perf" => Ok(Self::Perf),
            "pedantic" => Ok(Self::Pedantic),
            "restriction" => Ok(Self::Restriction),
            "nursery" => Ok(Self::Nursery),
            "cargo" => Ok(Self::Cargo),
            "internal" => Ok(Self::Internal),
            _ => Err(s.to_owned()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LintInfo {
    pub lint_token: kw::lint,
    pub version_token: kw::version,
    pub version: LitStr,
    pub category_token: Ident2,
    pub category: LintCategory,
    pub desc_token: kw::desc,
    pub desc: LitStr,
}

impl Parse for LintInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        let lint_token = input.parse::<kw::lint>()?;

        let content;
        parenthesized!(content in input);

        let version_token = content.parse::<kw::version>()?;
        content.parse::<Token![=]>()?;
        let version = content.parse::<LitStr>()?;
        content.parse::<Token![,]>()?;

        let category_token = content.parse::<Ident2>()?;
        let Ok(category) = LintCategory::from_str(&category_token.to_string()) else {
            return Err(Error::new_spanned(category_token, "unknown lint category"));
        };

        content.parse::<Token![,]>()?;
        let desc_token = content.parse::<kw::desc>()?;
        content.parse::<Token![=]>()?;
        let desc = content.parse::<LitStr>()?;

        Ok(Self {
            lint_token,
            version_token,
            version,
            category_token,
            category,
            desc_token,
            desc,
        })
    }
}

pub fn expand_paperclip(input: DeriveInput) -> Result<TokenStream2> {
    let Data::Struct(DataStruct { fields: Fields::Named(ref fields), .. }) = &input.data else {
        return Err(Error::new_spanned(
            input.ident,
            "`#[derive(Paperclip)]` only supports structs with named fields",
        ));
    };
    if !input
        .generics
        .params
        .iter()
        .all(|param| matches!(param, GenericParam::Lifetime(_)))
    {
        return Err(Error::new_spanned(
            input.ident,
            "`#[derive(Paperclip)]` only supports structs without generics",
        ));
    }

    let (names, lint_info): (Vec<Ident2>, Vec<Result<LintInfo>>) = fields
        .named
        .iter()
        .filter_map(|field| {
            let ident = Ident2::new(
                &field
                    .ident
                    .clone()
                    .expect("must be a struct with named fields")
                    .to_string()
                    .to_case(Case::ScreamingSnake),
                field.ident.clone().expect("must be a struct with named fields").span(),
            );
            let lint_attrs = field
                .attrs
                .iter()
                .filter_map(|attr| {
                    if attr.path().is_ident("paperclip")
                        && let Meta::List(ref list) = &attr.meta
                        && let Some(TokenTree2::Ident(lint)) = list.tokens.clone().into_iter().next()
                        && lint == "lint"
                    {
                        return Some((ident.clone(), attr.parse_args::<LintInfo>()));
                    }

                    None
                })
                .collect_vec();

            if let [lint] = &*lint_attrs {
                Some(lint.clone())
            } else if lint_attrs.is_empty() {
                None
            } else {
                Some((
                    ident.clone(),
                    Err(Error::new_spanned(
                        ident.clone(),
                        "`lint` can only be specified once per field",
                    )),
                ))
            }
        })
        .unzip();
    let lints = names
        .iter()
        .zip(lint_info)
        .map(|(name, lint_info)| -> Result<_> {
            let lint_info = lint_info?;

            let level = Ident2::new(
                match lint_info.category {
                    LintCategory::Correctness => "Deny",
                    LintCategory::Suspicious | LintCategory::Style | LintCategory::Complexity | LintCategory::Perf => {
                        "Warn"
                    },
                    LintCategory::Pedantic
                    | LintCategory::Restriction
                    | LintCategory::Nursery
                    | LintCategory::Cargo
                    | LintCategory::Internal => "Allow",
                },
                lint_info.category_token.span(),
            );
            let desc = lint_info.desc;

            Ok(quote! {
                declare_tool_lint! {
                    pub clippy::#name,
                    #level,
                    #desc,
                    report_in_external_macro: true
                }
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let ident = input.ident;
    let params = input.generics;

    Ok(quote! {
        #(#lints)*

        // FIXME: We must allow some way to include the lifetimes
        impl_lint_pass![#ident #params => [#(#names),*]];
    }
    .into())
}
