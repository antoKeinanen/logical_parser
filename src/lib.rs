use std::collections::HashMap;

use ast::Opcode;
use lalrpop_util::{ParseError, lexer};

use crate::ast::Expr;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub logic_parser);
pub mod ast;

/// The function evaluates the Boolean expression expr for each of the states in the states vector. The evaluation is performed by calling the `evaluate` function with the `state` and `expression` as arguments. The result of the evaluation is stored in a new vector. The `solve_truth_table` function returns this solution vector.
///
/// # Parameters
/// * `expr` - A boxed expression of the `Expr` type, which represents a logical expression to be evaluated.
/// * `states` - A vector of hash maps, where each hash map represents a different state of the Boolean variables used in the expression.
///
/// # Return Value
/// A vector of Boolean values that represent the evaluation result of the expression expr for each state in the states vector.
pub fn solve_truth_table(expr: Box<Expr>, states: Vec<HashMap<String, bool>>) -> Vec<bool>  {
    let mut solutions: Vec<bool> = Vec::new(); 
    for state in states {
        let res = evaluate(expr.clone(), state);
        solutions.push(res);
    }
    solutions
}


/// Generates all possible combinations of boolean values for a set of variables.
///
/// The function takes a vector of variable names `vars` and returns a vector of HashMaps.
/// Each key-value pair in the hash represents a variable and its corresponding boolean value.
///
/// # Arguments
///
/// * `vars` - A vector of variable names as strings.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use logical_solver::permutate;
///
/// let vars = vec!["A".to_string(), "B".to_string(), "C".to_string()];
/// let combs = permutate(vars);
///
/// for comb in combs {
///     println!("{:?}", comb);
/// }
/// ```
///
/// ## Output:
///
/// ```compile_fail
/// {"A": false, "B": false, "C": false}
/// {"A": false, "B": false, "C": true}
/// {"A": false, "B": true, "C": false}
/// {"A": false, "B": true, "C": true}
/// {"A": true, "B": false, "C": false}
/// {"A": true, "B": false, "C": true}
/// {"A": true, "B": true, "C": false}
/// {"A": true, "B": true, "C": true}
/// ```
pub fn permutate(vars: Vec<String>) -> Vec<HashMap<String, bool>>{
    let n = vars.len();
    if n == 0 {
        return vec![HashMap::new()];
    }
    else {
        let subcombs = permutate(vars[..n-1].to_vec()); 
        let mut combs = Vec::new();
        for subcomb in subcombs {
            let mut comb1 = subcomb.clone(); 
            comb1.insert(vars[n-1].clone(), false);
            let mut comb2 = subcomb.clone(); 
            comb2.insert(vars[n-1].clone(), true);
            combs.push(comb1);
            combs.push(comb2);
        }
        return combs;
    }
}
/// Evaluates a boolean expression using a given state of variables.
///
/// # Arguments
///
/// * `expr` - A boxed expression of type `Expr` to be evaluated.
/// * `state` - A hashmap of type `HashMap<String, bool>` that maps variable names to their boolean values.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use logical_solver::{evaluate, ast::{Expr, Opcode}};
///
/// let mut state = HashMap::new();
/// state.insert(String::from("P"), true);
/// state.insert(String::from("Q"), false);
///
/// let expr = Box::new(Expr::Op(Box::new(Expr::Variable(String::from("P"))), Opcode::And, Box::new(Expr::Variable(String::from("Q")))));
///
/// assert_eq!(evaluate(expr, state), false);
/// ```
///
/// # Panics
///
/// * If an `Expr::Not` node is encountered while evaluating the expression. The `Expr::Not` node should never be part of an `Expr::Op` node in a boolean expression.
/// * If variable mentioned in the expression is not found in the state `HashMap`.
///
pub fn evaluate(expr: Box<Expr>, state: HashMap<String, bool>) -> bool{
    match *expr {
        Expr::Boolean(value) => value,
        Expr::Neg(_, operand) => !evaluate(operand, state),
        Expr::Op(left, operator, right) => {
            let left_operand = evaluate(left, state.clone());
            let right_operand = evaluate(right, state.clone());

            match operator {
                Opcode::Conditional => return !left_operand || right_operand,
                Opcode::Biconditional => return left_operand == right_operand,
                Opcode::And => return left_operand && right_operand,
                Opcode::Or => left_operand || right_operand,
                Opcode::Not => panic!("Not should never be in Expr::Op"),
            }
        },
        Expr::Variable(name) => {
            *state.get(&name).unwrap()
        }
    }
}

/// Parses a logical expression represented as a string and returns a boxed expression.
///
/// The `expression` parameter is a string slice representing a logical expression to be parsed.
///
/// # Example
///
/// ```
/// use logical_solver::{ast::Expr, parse_expression};
/// use lalrpop_util::{ParseError, lexer};
///
/// let result = parse_expression("A and B");
///
/// match result {
///     Ok(expr) => {
///         assert_eq!(format!("{}", expr), "(A ∧ B)");
///     },
///     Err(error) => {
///         panic!("Failed to parse expression: {:?}", error);
///     }
/// }
/// ```
///
/// # Supported operations
///
/// | Predicate                         | Usage  |
/// | --------------------------------- | ------ |
/// | Conjunction(AND) ∧                | and    |
/// | Disjunction(OR) ∨                 | or     |
/// | Negation(NOT) ¬                   | not    |
/// | Conditional(IF...THEN) ⇒/→        | =>     |
/// | Biconditional(IF AND ONLY IF) ⇔/↔ | <=>    |
/// | Variables (for truth tables)\*     | [A-Z]+ |
///
/// \*Variables can be one or more capital letters.
///
/// # Returns
///
/// Returns a `Result` object containing a boxed expression if parsing was successful, or a `ParseError` object
/// if there was an error during parsing.
///
/// The `Expr` type is defined in the `logic_parser` crate, and represents a logical expression that can be
/// evaluated.
///
/// The `ParseError` type is also defined in the `lalrpo_util` crate, and represents an error that occurred
/// during parsing. The error contains information about the location of the error in the input string, the token
/// that caused the error, and a message describing the error.
pub fn parse_expression(expression: &str) -> Result<Box<Expr>, ParseError<usize, lexer::Token, &str>> {
    logic_parser::ExprParser::new()
        .parse(expression)
}
