use std::fmt;


/// Represents an expression in Boolean algebra.
///
/// The Expr enum has four possible variants:
///
/// - Boolean(bool): A boolean value.
/// - Op(Box<Expr>, Opcode, Box<Expr>): An operation with two expressions and an operator.
/// - Neg(Opcode, Box<Expr>): A negation operation with an operator and an expression.
/// - Variable(String): A variable with a string identifier.
///
/// This enum implements the fmt::Display trait, allowing for its values to be formatted as strings.
#[derive(Debug, Clone)]
pub enum Expr {
    Boolean(bool),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Neg(Opcode, Box<Expr>),
    Variable(String),
}

/// Opcode represents the possible logical operators in propositional logic.
///
/// # Implements
/// This enum implements the `fmt::Display` trait to allow for string formatting.
#[derive(Debug, Clone)]
pub enum Opcode {
    Conditional,
    Biconditional,
    And,
    Or,
    Not,
}


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Boolean(b) => write!(f, "{}", b),
            Expr::Op(ref l, op, ref r) => write!(f, "({} {} {})", l, op, r),
            Expr::Neg(op, ref v) => write!(f, "{} {}", op, v),
            Expr::Variable(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Opcode::Conditional => write!(f, "⇒"),
            Opcode::Biconditional => write!(f, "⇔"),
            Opcode::And => write!(f, "∧"),
            Opcode::Or => write!(f, "∨"),
            Opcode::Not => write!(f, "¬"),
        }
    }
}

impl Expr {
    /// Checks whether the expression is a leaf node or not.
    ///
    /// A leaf node is an expression that doesn't have any sub-expressions, i.e., it's a
    /// `Boolean` or a `Variable` expression. All other expressions are non-leaf nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_solver::ast::{Expr, Opcode};
    ///
    /// let expr1 = Expr::Boolean(true);
    /// let expr2 = Expr::Variable("x".to_string());
    /// let expr3 = Expr::Op(Box::new(Expr::Boolean(false)), Opcode::And, Box::new(Expr::Variable("Y".to_string())));
    ///
    /// assert!(expr1.is_leaf());
    /// assert!(expr2.is_leaf());
    /// assert!(!expr3.is_leaf());
    /// ```
    pub fn is_leaf(self) -> bool {
        match self {
            Expr::Boolean(_) => true,
            Expr::Variable(_) => true,
            Expr::Op(_, _, _) => false,
            Expr::Neg(_, _) => false,
        }
    }
}

/// Check if an expression contains a variable.
///
/// This function checks if an expression contains a variable by recursively searching through its
/// structure. It returns true if a variable is found, and false otherwise.
///
/// # Arguments
///
/// * `expr` - An expression to check for the presence of a variable.
///
/// # Returns
///
/// A boolean value indicating whether the expression contains a variable or not.
///
/// # Examples
///
/// ```
/// use logical_solver::ast::{has_variable, Expr, Opcode};
///
/// let expr = Expr::Op(Box::new(Expr::Variable("P".to_string())), Opcode::And, Box::new(Expr::Boolean(false)));
/// assert_eq!(has_variable(expr), true);
///
/// let expr = Expr::Neg(Opcode::Not, Box::new(Expr::Boolean(true)));
/// assert_eq!(has_variable(expr), false);
/// ```
pub fn has_variable(expr:Expr) -> bool {
    match expr {
        Expr::Boolean(_) => false,
        Expr::Op(left, _, right) => {
            has_variable(*left) || has_variable(*right)
        },
        Expr::Neg(_, operand) => has_variable(*operand),
        Expr::Variable(_) => true,
    }
}
