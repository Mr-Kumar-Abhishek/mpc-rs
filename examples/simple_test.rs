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
}