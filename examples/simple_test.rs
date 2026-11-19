use mpc::*;

fn main() {
    // Test char parser
    let parser = mpc_char('a');
    match mpc_parse("test", "a", &parser) {
        MpcResult::Ok(val) => {
            println!("Success: {:?}", val.downcast_ref::<String>());
        }
        MpcResult::Err(e) => {
            e.print();
        }
    }

    // Test string parser
    let parser2 = mpc_string("hello");
    match mpc_parse("test", "hello", &parser2) {
        MpcResult::Ok(val) => {
            println!("Success: {:?}", val.downcast_ref::<String>());
        }
        MpcResult::Err(e) => {
            e.print();
        }
    }

    // Test failing case
    let parser3 = mpc_char('a');
    match mpc_parse("test", "b", &parser3) {
        MpcResult::Ok(val) => {
            println!("Success: {:?}", val.downcast_ref::<String>());
        }
        MpcResult::Err(e) => {
            e.print();
        }
    }

    // Test or combinator
    let parser4 = mpc_or(vec![mpc_char('a'), mpc_char('b')]);
    match mpc_parse("test", "b", &parser4) {
        MpcResult::Ok(val) => {
            println!("Or Success: {:?}", val.downcast_ref::<String>());
        }
        MpcResult::Err(e) => {
            e.print();
        }
    }

    // Test AST construction with mpca_root, mpca_tag, and mpcf_fold_ast
    let number = mpca_tag(mpc_digits(), "number");
    let op = mpca_tag(mpc_oneof("+-"), "op");
    let expr = mpca_root(mpca_tag(
        mpc_and(vec![number.clone(), op, number], mpcf_fold_ast),
        "expr",
    ));
    match mpc_parse("expr", "12+34", &expr) {
        MpcResult::Ok(val) => {
            if let Some(ast) = val.downcast_ref::<MpcAst>() {
                ast.print();
            }
        }
        MpcResult::Err(e) => {
            e.print();
        }
    }
}