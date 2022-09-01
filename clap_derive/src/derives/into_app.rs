// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This work was derived from Structopt (https://github.com/TeXitoi/structopt)
// commit#ea76fa1b1b273e65e3b0b1046643715b49bec51f which is licensed under the
// MIT/Apache 2.0 license.

use std::env;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, Generics, Ident};

use crate::{
    item::{Item, Name, DEFAULT_CASING, DEFAULT_ENV_CASING},
    utils::Sp,
};

pub fn gen_for_struct(
    struct_name: &Ident,
    generics: &Generics,
    attrs: &[Attribute],
) -> TokenStream {
    let app_name = env::var("CARGO_PKG_NAME").ok().unwrap_or_default();

    let item = Item::from_args_struct(
        Span::call_site(),
        attrs,
        Name::Assigned(quote!(#app_name)),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );
    let name = item.cased_name();
    let app_var = Ident::new("__clap_app", Span::call_site());

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let tokens = quote! {
        #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
        )]
        #[deny(clippy::correctness)]
        impl #impl_generics clap::CommandFactory for #struct_name #ty_generics #where_clause {
            fn command<'b>() -> clap::Command {
                let #app_var = clap::Command::new(#name);
                <Self as clap::Args>::augment_args(#app_var)
            }

            fn command_for_update<'b>() -> clap::Command {
                let #app_var = clap::Command::new(#name);
                <Self as clap::Args>::augment_args_for_update(#app_var)
            }
        }
    };

    tokens
}

pub fn gen_for_enum(enum_name: &Ident, generics: &Generics, attrs: &[Attribute]) -> TokenStream {
    let app_name = env::var("CARGO_PKG_NAME").ok().unwrap_or_default();

    let item = Item::from_subcommand_enum(
        Span::call_site(),
        attrs,
        Name::Assigned(quote!(#app_name)),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );
    let name = item.cased_name();
    let app_var = Ident::new("__clap_app", Span::call_site());

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
        )]
        #[deny(clippy::correctness)]
        impl #impl_generics clap::CommandFactory for #enum_name #ty_generics #where_clause {
            fn command<'b>() -> clap::Command {
                let #app_var = clap::Command::new(#name)
                    .subcommand_required(true)
                    .arg_required_else_help(true);
                <Self as clap::Subcommand>::augment_subcommands(#app_var)
            }

            fn command_for_update<'b>() -> clap::Command {
                let #app_var = clap::Command::new(#name);
                <Self as clap::Subcommand>::augment_subcommands_for_update(#app_var)
            }
        }
    }
}
