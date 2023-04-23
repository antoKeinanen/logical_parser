# Logical solver
![Crates.io](https://img.shields.io/crates/v/logical_solver)
![docs.rs](https://img.shields.io/docsrs/logical_solver)

Logical solver is a rust library for solving and parsing logical equations.

## Supported operations
| Predicate                         | Usage  |
| --------------------------------- | ------ |
| Conjunction(AND) ∧                | and    |
| Disjunction(OR) ∨                 | or     |
| Negation(NOT) ¬                   | not    |
| Conditional(IF...THEN) ⇒/→        | =>     |
| Biconditional(IF AND ONLY IF) ⇔/↔ | <=>    |
| Variables (for truth tables)\*     | [A-Z]+ |

\*Variables can be one or more capital letters.

## Usage
```rust
let expr = parse_expression("true => not false or (true and false)");
let result = enumerate(expr, HashMap::new());
assert_eq!(result, true);
```

```rust
let vars = vec!(String::from("A"), String::from("B"))
let expr = parse_expression("A => not B");
let states = permutate(vars);
let result = solve_truth_table(expr, vars);
assert_eq!(result, [true, true, true, false]);
```

For full examples check [/examples](https://github.com/antoKeinanen/logical_parser/tree/main/examples) folder. Run them with:
`cargo run --example <example_name>`
