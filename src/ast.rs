use std::fmt;


#[derive(Debug, Clone)]
pub enum Expr {
    Boolean(bool),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Neg(Opcode, Box<Expr>),
    Variable(String),
}

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
    pub fn is_leaf(self) -> bool {
        match self {
            Expr::Boolean(_) => true,
            Expr::Variable(_) => true,
            Expr::Op(_, _, _) => false,
            Expr::Neg(_, _) => false,
        }
    }
}

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
