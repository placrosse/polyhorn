#![warn(missing_docs)]

//! This crate wraps the macros implemented in Polyhorn UI in a
//! `proc-macro = true` library.

use proc_macro::TokenStream;

/// Calls `polyhorn_ui::macros::asset::asset_impl` with a token stream that we
/// obtain from the Rust compiler.
#[proc_macro]
pub fn asset(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let result = polyhorn_ui::macros::asset::asset_impl(input);
    let result: proc_macro2::TokenStream = result.into();
    result.into()
}

/// Calls `polyhorn_ui::macros::render::render_impl` with a token stream that we
/// obtain from the Rust compiler.
#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let result = polyhorn_ui::macros::render::render_impl(input);
    let result: proc_macro2::TokenStream = result.into();
    result.into()
}

/// Calls `polyhorn_ui::macros::style::style_impl` with a token stream that we
/// obtain from the Rust compiler.
#[proc_macro]
pub fn style(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let result = polyhorn_ui::macros::style::style_impl(input);
    let result: proc_macro2::TokenStream = result.into();
    result.into()
}

/// Calls `polyhorn_ui::macros::test::test_impl` with a token stream that we
/// obtain from the Rust compiler.
#[proc_macro_attribute]
pub fn test(_: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let result = polyhorn_ui::macros::test::test_impl(input);
    let result: proc_macro2::TokenStream = result.into();
    result.into()
}
