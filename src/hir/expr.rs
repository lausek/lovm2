use crate::value::CoValue;
use crate::var::Variable;

macro_rules! auto_implement {
    (1, $operator:ident, $method:ident) => {
        pub fn $method(expr: Expr) -> Expr {
            Expr::Operation1(Operator1::$operator, Box::new(expr))
        }
    };
    (2, $operator:ident, $method:ident) => {
        pub fn $method(left: Expr, right: Expr) -> Expr {
            Expr::Operation2(Operator2::$operator, Box::new(left), Box::new(right))
        }
    };
}

pub enum Operator2 {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,

    Equal,
    NotEqual,
    GreaterEqual,
    GreaterThan,
    LessEqual,
    LessThan,
}

pub enum Operator1 {
    Not,
}

pub enum Expr {
    Operation2(Operator2, Box<Expr>, Box<Expr>),
    Operation1(Operator1, Box<Expr>),
    Call(Variable, Vec<Expr>),
    Value(CoValue),
    Variable(Variable),
}

impl Expr {
    auto_implement!(2, Add, add);
    auto_implement!(2, Sub, sub);
    auto_implement!(2, Mul, mul);
    auto_implement!(2, Div, div);
    auto_implement!(2, Rem, rem);
    auto_implement!(2, And, and);
    auto_implement!(2, Or, or);

    auto_implement!(2, Equal, eq);
    auto_implement!(2, NotEqual, ne);
    auto_implement!(2, GreaterEqual, ge);
    auto_implement!(2, GreaterThan, gt);
    auto_implement!(2, LessEqual, le);
    auto_implement!(2, LessThan, lt);

    auto_implement!(1, Not, not);
}

impl From<CoValue> for Expr {
    fn from(val: CoValue) -> Expr {
        Expr::Value(val)
    }
}

impl From<Variable> for Expr {
    fn from(v: Variable) -> Expr {
        Expr::Variable(v)
    }
}
