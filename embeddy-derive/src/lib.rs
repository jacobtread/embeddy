use proc_macro::TokenStream;
use quote::quote;
use std::{fs::canonicalize, path::Path};
use syn::{parse_macro_input, DeriveInput, Expr, ExprLit, Ident, Lit, Meta, MetaNameValue};
use walkdir::WalkDir;

/// Derive macro for the [`embeddy-derive::Embedded`] trait
#[proc_macro_derive(Embedded, attributes(folder))]
pub fn derive_embeddy(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let ident: Ident = input.ident;

    // Find the #[folder = ""] attribute form the derive input
    let folder: String = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("folder"))
        .and_then(|attr| match &attr.meta {
            Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(val), ..
                    }),
                ..
            }) => Some(val.value()),
            _ => None,
        })
        .expect("Missing folder attribute");

    // Path to the folder relative to the Cargo.toml file
    let folder_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(folder);

    if !folder_path.exists() {
        panic!(
            "Unable to embed folder that doesn't exist: {}",
            folder_path.display()
        )
    }

    let mut includes = Vec::new();

    // Walk the folder path collecting all files
    let files = WalkDir::new(&folder_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file());

    for file in files {
        let rel_path = file
            .path()
            // Remove the path prefix from the relative path
            .strip_prefix(&folder_path)
            .unwrap()
            .to_str()
            .expect("Path missing str repr")
            // Normalize the path seporator
            .replace("\\", "/");

        let full_path = canonicalize(file.path()).expect("Failed to get file canonical path");
        let full_path = full_path.to_str().expect("Path missing str repr");

        includes.push(quote! {
            #rel_path => include_bytes!(#full_path),
        });
    }

    quote! {
        impl embeddy::Embedded for #ident {

            fn get(path: &str) -> Option<&'static [u8]> {
                Some(match path {
                    #(#includes)*
                    _ => return None
                })
            }
        }
    }
    .into()
}
