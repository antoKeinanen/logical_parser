use logical_solver::ast::has_variable;
use logical_solver::{parse_expression, evaluate, permutate, solve_truth_table};
use std::io;
use std::io::Write;

fn main() {
    print!("Enter variables> ");
    io::stdout().flush().unwrap();
    let mut user_vars = String::new();
    io::stdin()
        .read_line(&mut user_vars)
        .expect("Failed to read from stdin");
    
    let user_vars: Vec<String> = user_vars.split(", ").map(|s:&str| s.to_string().replace("\n", "")).collect();

    print!("Enter equation> ");
    io::stdout().flush().unwrap();
    let mut user_expr = String::new();
    io::stdin()
        .read_line(&mut user_expr)
        .expect("Failed to read from stdin");

    let expr = parse_expression(user_expr.as_str()).unwrap(); 
    println!("Generating permutations...");
    let states = permutate(user_vars);
    println!("Solving truth table...");
    let result = solve_truth_table(expr, states);
    
    println!("{:?}", result);
}
