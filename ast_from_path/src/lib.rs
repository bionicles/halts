// halts/ast_from_path/src/lib.rs
// use quote::quote;
use std::env;
use syn::{File, ItemFn, Path};

// #[proc_macro]
// pub fn ast_from_path(input: TokenStream) -> TokenStream {
pub fn ast_from_path(input_str: &str) -> Result<ItemFn, String> {
    // Parse the input token stream into a string.
    // let input_str = input.to_string();

    // Remove the double quotes from the beginning and end of the file path string.
    let input_str = input_str.trim_matches('"');

    // Split the input string into the file path and the function name.
    let (file_path, function_path) = match input_str.split_once("::") {
        Some((file_path, function_name)) => (file_path, function_name),
        None => {
            return Err("Whatever!".to_string());
            // return syn::Error::new(
            //     span.into(),
            //     "Expected input in the form 'file_path::function_name'",
            // )
            // .into_compile_error()
            // .into();
        }
    };

    // Get the current working directory.
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => return Err(e.to_string()),
    };

    // Construct an absolute file path from the relative file path.
    let absolute_file_path = current_dir.join(file_path);

    // println!("absolute_file_path: \n {:?}", absolute_file_path);

    // Read the source file.
    let file_content = match std::fs::read_to_string(&absolute_file_path) {
        Ok(content) => content,
        Err(e) => return Err(e.to_string()),
    };

    // println!("file_content:\n {}", file_content);

    // Parse the file_content into a token stream.
    let file_ast: syn::File = syn::parse_file(&file_content).unwrap();
    // println!("{:?}", file_ast);

    // Traverse the AST to locate the subtree at `function_path`
    // Define a recursive function to traverse the AST and find the subtree at the specified path.
    fn get_function(function_path: &str, ast: &File) -> Option<ItemFn> {
        let parsed_function_path = syn::parse_str::<Path>(function_path).unwrap();
        let path_segments = parsed_function_path
            .segments
            .iter()
            .map(|s| &s.ident)
            .collect::<Vec<_>>();

        fn find_function_helper<'a>(
            items: &'a [syn::Item],
            path_segments: &'a [&syn::Ident],
        ) -> Option<&'a ItemFn> {
            for item in items {
                let current_segment = path_segments[0].to_string();
                match item {
                    syn::Item::Fn(func) => {
                        // println!("func={:?}", func);
                        let function_identifier = func.sig.ident.to_string();
                        // println!(
                        //     "function_identifier={}, current_segment={}",
                        //     function_identifier, current_segment
                        // );
                        if function_identifier == current_segment {
                            if path_segments.len() == 1 {
                                return Some(func);
                            }
                        }
                    }
                    syn::Item::Mod(module) => {
                        let module_identifier = module.ident.to_string();
                        if module_identifier == current_segment {
                            if let Some(content) = &module.content {
                                return find_function_helper(&content.1, &path_segments[1..]);
                            }
                        }
                    }
                    _ => {}
                }
            }
            None
        }

        find_function_helper(&ast.items, &path_segments).cloned()
    }
    let subtree = get_function(function_path, &file_ast).unwrap();
    // println!("subtree={:?}", subtree);

    // Generate the code for creating the AST from the function.
    // let target_function_token_stream = quote! {
    //     #subtree
    // };

    // println!(
    //     "target_function_token_stream={}",
    //     target_function_token_stream
    // );
    // let output = target_function_token_stream.into();
    // println!("output={}", output);
    // output
    Ok(subtree)
}

// return syn::Error::new(
//     span.into(),
//     &format!("Failed to get current directory: {}", e),
// )
// .into_compile_error()
// .into();
// }

// Parse the function path into a sequence of identifiers.
// let parsed_function_path = syn::parse_str::<syn::Path>(function_path).unwrap();

// let token_stream = syn::parse_str::<TokenStream2>(&file_content).unwrap();
// println!("token_stream:\n{:?}", token_stream);

// fn extract_function(path: &str, tokens: TokenStream2) -> syn::Result<TokenStream2> {
//     let path_segments: Vec<&str> = path.split("::").collect();
//     let mut tokens_iter = tokens.into_iter();
//     let mut segment_n = 0;

//     while let Some(token) = tokens_iter.next() {
//         let segment = path_segments[segment_n];
//         println!("segment={}, token={:?}", segment, token);
//         match token {
//             proc_macro2::TokenTree::Ident(ident) => {
//                 if ident.to_string() == segment {
//                     println!("HIT!");
//                     if segment_n == path_segments.len() - 1 {
//                         // Found the function! Collect every remaining token for no reason
//                         let mut function_tokens = Vec::<proc_macro2::TokenTree>::new();
//                         if let Some(proc_macro2::TokenTree::Group(next_token)) =
//                             tokens_iter.next()
//                         {
//                             println!("next_token={:?}", next_token.to_string());
//                             function_tokens.extend(next_token.stream());
//                             let mut function_token_stream = TokenStream2::new();
//                             function_token_stream.extend(function_tokens);
//                             return Ok(function_token_stream);
//                         } else {
//                             return Err(syn::Error::new(
//                                 proc_macro::Span::call_site().into(),
//                                 "whatever",
//                             ));
//                         }
//                     } else {
//                         segment_n += 1;
//                         continue;
//                     }
//                 }
//             }
//             _ => {} // Ignore other tokens
//         }
//     }

//     return Err(syn::Error::new(
//         proc_macro::Span::call_site().into(),
//         "whatever",
//     ));
// }

// fn extract_function(path: &str, tokens: proc_macro2::TokenStream) -> syn::Result<syn::ItemFn> {
//     let mut iter = tokens.into_iter().enumerate();
//     let mut function_tokens = Vec::<proc_macro2::TokenTree>::new();
//     let path_segments: Vec<&str> = path.split("::").collect();

//     let mut in_function = false;
//     let mut found_function = false;
//     let mut path_step_index = 0;

//     // /// A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`).
//     // #[derive(Clone)]
//     // pub enum TokenTree {
//     //     /// A token stream surrounded by bracket delimiters.
//     //     Group(Group),
//     //     /// An identifier.
//     //     Ident(Ident),
//     //     /// A single punctuation character (`+`, `,`, `$`, etc.).
//     //     Punct(Punct),
//     //     /// A literal character (`'a'`), string (`"hello"`), number (`2.3`), etc.
//     //     Literal(Literal),
//     // }
//     while let Some((i, token)) = iter.next() {
//         let current_segment = path_segments[path_step_index];
//         println!("{} {:?}", i, token);
//         match token {
//             proc_macro2::TokenTree::Ident(ident) => {
//                 let ident_string = ident.to_string();
//                 println!("ident string {}", ident_string);
//                 if ident_string == current_segment {
//                     path_step_index += 1;
//                 }
//             }
//             proc_macro2::TokenTree::Group(group) => {
//                 let group_string = group.to_string();
//                 println!("group string {}", group_string);
//             }
//             _ => {}
//         }
//     }

//     if found_function {
//         let mut function_token_stream = TokenStream2::new();
//         function_token_stream.extend(function_tokens);
//         syn::parse2::<syn::ItemFn>(function_token_stream.into())
//     } else {
//         Err(syn::Error::new(
//             proc_macro2::Span::call_site(),
//             "Function not found in the given path",
//         ))
//     }
// }

// commented out this parsing since it's done in extract_function or unused
// let file_root_ast_node = syn::parse(token_stream.into()).unwrap();
// println!("PARSED file_root_ast_node");

// // Parse the function path into a sequence of identifiers.
// let parsed_function_path = syn::parse_str::<syn::Path>(function_path).unwrap();
// println!("PARSED function_path:\n{:?}", "function_path");
//     syn::Token![fn] => {
//         if in_function {
//             function_tokens.push(token);
//         } else {
//             in_function = true;
//         }
//     }
//     syn::Ident { ident, .. } => {
//         if in_function {
//             if ident == path_segments.last().unwrap() {
//                 found_function = true;
//             }
//             function_tokens.push(token);
//         }
//     }
//     syn::Token![:] => {
//         if in_function {
//             function_tokens.push(token);
//         }
//     }
//     syn::Group {
//         group_token: syn::token::Paren,
//         stream,
//         ..
//     } => {
//         if in_function {
//             function_tokens.extend(stream);
//         }
//     }
//     syn::Group {
//         group_token: syn::token::Brace,
//         stream,
//         ..
//     } => {
//         if in_function {
//             function_tokens.extend(stream);
//             break;
//         }
//     }
//     _ => {
//         if in_function {
//             function_tokens.push(token);
//         }
//     }
