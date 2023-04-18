use logical_solver::{parse_expression, permutate, solve_truth_table};

#[test]
fn test_truth_tables() {
    let vars = "A, B";
    let vars: Vec<String> = vars.split(", ").map(|s:&str| s.to_string().replace("\n", "")).collect();
    let vars = permutate(vars);

    let expr = parse_expression("not A or B <=> not A or B").unwrap();
    assert_eq!(vec![true, true, true, true], solve_truth_table(expr, vars.clone()));

    let expr = parse_expression("A and (B or not A) => A").unwrap();
    assert_eq!(vec![true, true, true, true], solve_truth_table(expr, vars.clone()));

    let expr = parse_expression("(not A or B) and not B <=> not B").unwrap();
    assert_eq!(vec![true, true, false, true], solve_truth_table(expr, vars.clone()));

    let expr = parse_expression("not (A and (B or not B)) => not A").unwrap();
    assert_eq!(vec![true, true, true, true], solve_truth_table(expr, vars.clone()));

    let expr = parse_expression("A or (B and not A) <=> A or B").unwrap();
    assert_eq!(vec![true, true, true, true], solve_truth_table(expr, vars.clone()));

    let expr = parse_expression("not (not A or B) or A => A").unwrap();
    assert_eq!(vec![true, true, true, true], solve_truth_table(expr, vars.clone()));

    let expr = parse_expression("A and (B or A) <=> A").unwrap();
    assert_eq!(vec![true, true, true, true], solve_truth_table(expr, vars.clone()));

    let expr = parse_expression("not (A <=> B) or not B => not B").unwrap();
    assert_eq!(vec![true, false, true, true], solve_truth_table(expr, vars.clone()));

    let expr = parse_expression("not (A and B) or (not A and B) <=> B").unwrap();
    assert_eq!(vec![false, true, false, false], solve_truth_table(expr, vars.clone()));

    let expr = parse_expression("A and B => B").unwrap();
    assert_eq!(vec![true, true, true, true], solve_truth_table(expr, vars.clone()));
}
