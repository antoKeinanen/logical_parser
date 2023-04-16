use logical_solver::ast::has_variable;
use logical_solver::{parse_expression, evaluate};
use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    print!("Enter equation> ");
    io::stdout().flush().unwrap();
    let mut user_expr = String::new();
    io::stdin()
        .read_line(&mut user_expr)
        .expect("Failed to read from stdin");

    let expr = parse_expression(user_expr.as_str()).unwrap(); 
    println!("{:?}", &expr);
    println!("{}", has_variable(*expr.clone()));
    
    let mut state = HashMap::new();

    let result = evaluate(expr, state);
    println!("{}", result);
}
