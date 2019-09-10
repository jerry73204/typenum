extern crate proc_macro;

mod tyint;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitInt, Result as SynResult, UnOp};

#[proc_macro]
pub fn tyint(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitInt);
    let tokens = tyint::f_tyint(literal).unwrap_or_else(|err| err.to_compile_error().into());
    TokenStream::from(tokens)
}
