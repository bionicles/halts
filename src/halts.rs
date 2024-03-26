//! Given the 3-Decidability of the Ternary Halting Problem is a separate concern:
//! Whereas the original halting problem has a binary categorical output {HALT, LOOP},
//! Ternary Halting / 3-Decidability affords ternary categorical output {HALT, LOOP, PARADOX}.
//! Given the proof of 2-Undecidability relies heavily on the concept of paradox,
//! I believe this approach to be promising to facilitate real-world static analysis.

use std::collections;
use std::error::Error;
use std::fmt;

use quote::quote;
use syn;

/// Convert a function into an AST
/// # Returns
/// A result containing a `syn::ItemFn` root AST node
/// # Errors
/// If the conversion of the function into a token stream fails
/// If the conversion of the code string into a syn AST fails
pub fn ast_from_function<F>(_function: F) -> syn::Result<syn::ItemFn> {
    let token_stream = quote! { function };
    let code = token_stream.to_string();
    ast_from_str(&code)
}

/// Convert function code string to AST
fn ast_from_str(code: &str) -> syn::Result<syn::ItemFn> {
    let ast: syn::ItemFn = syn::parse_str(code)?;
    Ok(ast)
}

/// Convert function AST to string for visualization and understanding
pub fn string_from_ast(ast: &syn::ItemFn) -> String {
    let code = quote! { #ast };
    code.to_string()
}

/// An enumeration of possible errors that can occur when determining if a function halts.
#[derive(Debug)]
pub enum ParadoxError {
    /// A compile error occurred.
    CompileError,
    /// An inversion paradox was detected.
    InversionParadox,
}

impl Error for ParadoxError {}

impl fmt::Display for ParadoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParadoxError::CompileError => write!(f, "Compile error"),
            ParadoxError::InversionParadox => write!(f, "Inversion paradox detected"),
        }
    }
}

/// Determine if a function halts or loops.
///
/// # Arguments
///
/// * `maybe_paradox_function` - A function that may or may not halt.
///
/// # Returns
///
/// A `Result` containing a boolean indicating if the function halts or an error if an inversion paradox is detected.
///
/// # Errors
///
/// Returns an error if an inversion paradox is detected.
pub fn halts<F>(maybe_paradox_function: F) -> Result<bool, ParadoxError>
where
    F: Fn(),
{
    // Parse the AST of the function
    let ast = ast_from_function(maybe_paradox_function).unwrap();
    // If the AST contains a call to halts, then it's an inversion paradox
    // and we return a compile error
    if ast_contains_halts(&ast) {
        return Err(ParadoxError::InversionParadox);
    }

    // having categorized the ParadoxError cases, those which remain are
    // non-paradoxical (thus hopefully decidable) instances of the binary halting problem
    let loopy = loops(&ast);

    Ok(!loopy)
}

/// Check if the AST of a function contains a call to `halts`.
///
/// # Arguments
///
/// * `_` - A function to check.
///
/// # Returns
///
/// A boolean indicating if the AST of the function contains a call to `halts`.
fn ast_contains_halts<F>(_: F) -> bool {
    // TODO: Implement AST parsing and check for calls to halts
    false
}

/// Determine if a function contains any loops.
///
/// # Arguments
///
/// * `function` - A function to check for loops.
///
/// # Returns
///
/// A boolean indicating if the function contains any loops.
fn loops(ast: &syn::ItemFn) -> bool {
    let mut has_recursion = false;
    let mut has_unreachable_base_case = false;
    let mut has_endless_iteration = false;

    // Check for recursion
    for recursion in iter_recursions(ast) {
        has_recursion = true;
        if !has_base_case(recursion) {
            return true;
        }
        if is_base_case_unreachable(recursion) {
            has_unreachable_base_case = true;
        }
    }

    // Check for iteration
    for iteration in iter_iterations(ast) {
        if is_endless_loop(iteration) {
            has_endless_iteration = true;
        }
    }

    // If there is recursion with an unreachable base case or endless iteration,
    // then the function is loopy
    has_recursion && has_unreachable_base_case || has_endless_iteration
}

/// Get all recursive functions within a function.
///
/// # Arguments
///
/// * `ast` - The AST of the function from which to iterate recursions.
///
/// # Returns
///
/// A vector of recursive functions within the given function.
fn iter_recursions(ast: &syn::ItemFn) -> Vec<&syn::Expr> {
    let mut visited = collections::HashSet::new();
    let mut recursions = Vec::<&syn::Expr>::new();

    let sig = &ast.sig;
    let ident = &sig.ident;

    // Check if the function calls itself directly
    if let Some(syn::Stmt::Expr(syn::Expr::Call(call_expr), _)) = ast.block.stmts.last() {
        if let syn::Expr::Path(path_expr) = &*call_expr.func {
            if path_expr.path.segments.len() == 1 && path_expr.path.segments[0].ident == *ident {
                for pair in call_expr.args.pairs() {
                    match pair {
                        syn::punctuated::Pair::Punctuated(t, _) => recursions.push(t),
                        syn::punctuated::Pair::End(_) => continue,
                    }
                }
            }
        }
    }

    // Recursively traverse the AST to find indirect recursion
    for stmt in &ast.block.stmts {
        if let syn::Stmt::Expr(syn::Expr::Call(call_expr), _) = stmt {
            if let syn::Expr::Path(path_expr) = &*call_expr.func {
                let path = &path_expr.path;
                if let Some(last_segment) = path.segments.last() {
                    let ident = &last_segment.ident;
                    if !visited.contains(ident) {
                        visited.insert(ident.clone());
                        if let Some(called_fn) = find_fn_by_path(&ast.block, path) {
                            recursions.extend(iter_recursions(called_fn));
                        }
                    }
                }
            }
        }
    }
    recursions
}

/// helper function to locate a function given its syn::Path
fn find_fn_by_path<'a>(block: &'a syn::Block, path: &syn::Path) -> Option<&'a syn::ItemFn> {
    for stmt in &block.stmts {
        if let syn::Stmt::Item(syn::Item::Fn(fn_item)) = stmt {
            if fn_item.sig.ident == path.segments.last()?.ident {
                return Some(fn_item);
            }
        }
    }
    None
}

/// Get all iterative functions within a function.
///
/// # Arguments
///
/// * `ast` - The AST of the function from which to iterate iterations.
///
/// # Returns
///
/// A vector of iterative functions within the given function.
fn iter_iterations(ast: &syn::ItemFn) -> Vec<&syn::Expr> {
    // TODO: Implement iteration detection
    vec![]
}

/// Determine if a function has a base case.
/// Recursion with no base case is a LOOP (modulo stack overflow)
/// (modulo stack overflow without tail loop optimization)
///
/// # Arguments
///
/// * `recursion` - An instance of recursion to check for a nonexistent base case.
///
/// # Returns
///
/// A boolean indicating if the function has a base case.
fn has_base_case(recursion: &syn::Expr) -> bool {
    // TODO: Implement base case detection
    false
}

/// Determine if a base case is unreachable within a function.
/// Recursion with an unreachable base case is a LOOP
/// (modulo stack overflow without tail loop optimization)
///
/// # Arguments
///
/// * `recursion` - An instance of recursion to check for an unreachable base case.
///
/// # Returns
///
/// A boolean indicating if the base case is unreachable within the function.
fn is_base_case_unreachable(recursion: &syn::Expr) -> bool {
    // TODO: Implement unreachable base case detection
    false
}

/// Determine if an iteration contains an endless loop.
/// Iteration with an infinite loop is, unsurprisingly, a LOOP.
///
/// # Arguments
///
/// * `iteration` - An instance of iteration to check for an infinite loop.
///
/// # Returns
///
/// A boolean indicating if the function contains an endless loop.
fn is_endless_loop(iteration: &syn::Expr) -> bool {
    // TODO: Implement endless loop detection
    false
}

/// tests for the ternary halting problem
mod test {
    use super::*;

    // ------ Begin Cases ------
    #[allow(dead_code)]
    /// A function that calls itself recursively.
    pub fn g() {
        match halts(g) {
            Err(_) | Ok(true) => loop_forever(),
            Ok(false) => unit(),
        }
    }

    #[allow(dead_code)]
    /// A function that loops forever.
    pub fn loop_forever() {
        loop {
            println!("Looping forever!")
        }
    }

    #[allow(dead_code)]
    /// A function that does nothing.
    pub fn unit() {}

    #[allow(unconditional_recursion)]
    #[allow(dead_code)]
    /// A function which recurses unconditionally
    fn recurse_unconditionally() {
        recurse_unconditionally()
    }

    // Recursive Chain, which does stop
    #[allow(dead_code)]
    /// The first step in a recursive chain
    fn recursive_chain_start() {
        recursive_chain_middle();
    }

    /// The middle step in a recursive chain
    #[allow(dead_code)]
    fn recursive_chain_middle() {
        recursive_chain_end();
    }

    /// The penultimate step in a recursive chain
    #[allow(dead_code)]
    fn recursive_chain_end() {
        unit();
    }

    // Recursive Cycle, which does not stop

    #[allow(dead_code)]
    /// The first step in a recursive cycle
    fn recursive_cycle_a() {
        recursive_cycle_b();
    }

    #[allow(dead_code)]
    /// The second step in a recursive cycle
    fn recursive_cycle_b() {
        recursive_cycle_c();
    }

    #[allow(dead_code)]
    /// The final step in a recursive cycle, invokes the first
    fn recursive_cycle_c() {
        recursive_cycle_a();
    }

    // ------ End Cases, Begin Tests -------

    #[test]
    /// Test the `iter_recursions` function correctly traverses the AST to identify instances of recursion
    fn test_iter_recursions() {
        let ast = ast_from_function(|| {}).unwrap();
        let recursions = iter_recursions(&ast);
        assert_eq!(recursions.len(), 0);

        let ast = ast_from_function(recurse_unconditionally).unwrap();
        let recursions = iter_recursions(&ast);
        assert_eq!(recursions.len(), 1);

        let ast = ast_from_function(g).unwrap();
        let recursions = iter_recursions(&ast);
        assert_eq!(recursions.len(), 1);

        let ast = ast_from_function(loop_forever).unwrap();
        let recursions = iter_recursions(&ast);
        assert_eq!(recursions.len(), 0);

        let ast = ast_from_function(unit).unwrap();
        let recursions = iter_recursions(&ast);
        assert_eq!(recursions.len(), 0);

        let ast = ast_from_function(recursive_chain_start).unwrap();
        let recursions = iter_recursions(&ast);
        assert_eq!(recursions.len(), 3);

        let ast = ast_from_function(recursive_chain_end).unwrap();
        let recursions = iter_recursions(&ast);
        assert_eq!(recursions.len(), 0);
    }

    /// Test that the `halts` function correctly identifies a function that halts.
    #[test]
    fn test_halts_unit() {
        assert!(halts(unit).unwrap());
    }

    /// Test that the `halts` function correctly identifies a function that loops forever.
    #[test]
    fn test_no_halts_if_loop_forever() {
        assert!(!halts(loop_forever).unwrap());
    }

    // Using dtolnay's trybuild crate for compile-fail test
    // cargo install trybuild
    // trybuild ui/test_g_halts_compile_err.rs

    /// Test that the `halts` function correctly identifies a function that contains an inversion paradox.
    #[test]
    fn test_halts_g() {
        assert!(halts(g).is_err());
    }
}
