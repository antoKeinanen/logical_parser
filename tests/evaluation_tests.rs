use logical_solver::{parse_expression, evaluate};
use std::collections::HashMap; 

#[test]
fn test_evaluation() {
    let state = HashMap::new();

    let expr = parse_expression("true and false => false").unwrap();
    assert_eq!(true, evaluate(expr, state.clone()));
   
    let expr = parse_expression("not true or false <=> false").unwrap();
    assert_eq!(true, evaluate(expr, state.clone()));

    let expr = parse_expression("true and (false or not true) => false").unwrap();
    assert_eq!(true, evaluate(expr, state.clone()));

    let expr = parse_expression("(not true or false) and not false <=> true").unwrap();
    assert_eq!(false, evaluate(expr, state.clone()));

    let expr = parse_expression("not (true and (false or not false)) => true").unwrap();
    assert_eq!(true, evaluate(expr, state.clone()));

    let expr = parse_expression("true or (false and not true) <=> true").unwrap();
    assert_eq!(true, evaluate(expr, state.clone()));

    let expr = parse_expression("not (not true or false) or true => true").unwrap();
    assert_eq!(true, evaluate(expr, state.clone()));

    let expr = parse_expression("true and (false or true) <=> true").unwrap();
    assert_eq!(true, evaluate(expr, state.clone()));

    let expr = parse_expression("not (true <=> false) or not false => true").unwrap();
    assert_eq!(true, evaluate(expr, state.clone()));

    let expr = parse_expression("not (true and false) or (not true and false) <=> true").unwrap();
    assert_eq!(true, evaluate(expr, state.clone()));

}
