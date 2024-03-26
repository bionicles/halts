// halts/ast_from_path/tests/ast_from_path_test.rs

use ast_from_path::ast_from_path;

// This is the function you want to test your proc macro on
fn test_function() {
    println!("level 1");
}
mod nested {
    fn nested_test_function() {
        println!("level 2");
    }

    mod deeply {
        fn deeply_nested_test_function() {
            println!("level 3");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ast_from_path() {
        let ast1 = ast_from_path("tests/ast_from_path_test.rs::test_function").unwrap();
        println!("ast1={:?}", ast1);
        assert!(false);
        // let ast2 = ast_from_path!("tests/ast_from_path_test.rs::nested::nested_test_function");
        // println!("{}", ast2.to_string());
        // let ast3 =
        //     ast_from_path!("tests/ast_from_path_test.rs::nested::deeply::deeply_nested_test_function");
        // println!("{}", ast3.to_string());
    }
}
