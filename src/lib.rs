use std::collections::HashMap;

use ast::Opcode;
use lalrpop_util::{ParseError, lexer};

use crate::ast::Expr;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub logic_parser);
pub mod ast;

pub fn solve_truth_table(expr: Box<Expr>, states: Vec<HashMap<String, bool>>) -> Vec<bool>  {
    let mut solutions: Vec<bool> = Vec::new(); 
    for state in states {
        let res = evaluate(expr.clone(), state);
        solutions.push(res);
    }
    solutions
}

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

pub fn parse_expression(expression: &str) -> Result<Box<Expr>, ParseError<usize, lexer::Token, &str>> {
    logic_parser::ExprParser::new()
        .parse(expression)
}
