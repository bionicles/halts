//! src/main.rs

use halts::halts::string_from_ast;

/// code for the halts module as a const str inside the halts module
const CODE: &str = include_str!("./halts.rs");

fn main() {
    let ast = syn::parse_str::<syn::ItemFn>(CODE).unwrap();
    let ast_str = string_from_ast(&ast);
    println!("{}", ast_str);
}
