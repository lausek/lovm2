use crate::bytecode::Instruction;
use crate::hir::assign::Access;
use crate::hir::call::Call;
use crate::hir::cast::Cast;
use crate::hir::initialize::Initialize;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::hir::slice::Slice;
use crate::value::CoValue;
use crate::var::Variable;

macro_rules! auto_implement {
    (1, $operator:ident, $method:ident) => {
        pub fn $method<T>(expr: T) -> Expr
        where
            T: Into<Expr>,
        {
            Expr::Operation1(Operator1::$operator, Box::new(expr.into()))
        }
    };
    (2, $operator:ident, $method:ident) => {
        pub fn $method<T, U>(left: T, right: U) -> Expr
        where
            T: Into<Expr>,
            U: Into<Expr>,
        {
            Expr::Operation2(
                Operator2::$operator,
                Box::new(left.into()),
                Box::new(right.into()),
            )
        }
    };
}

#[derive(Clone, Debug)]
pub enum Expr {
    Access(Access),
    Call(Call),
    Cast(Cast),
    DynamicValue(Initialize),
    Operation1(Operator1, Box<Expr>),
    Operation2(Operator2, Box<Expr>, Box<Expr>),
    Slice(Slice),
    Value { val: CoValue, boxed: bool },
    Variable(Variable),
}

#[derive(Clone, Debug)]
pub enum Operator2 {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
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

#[derive(Clone, Debug)]
pub enum Operator1 {
    Not,
}

impl Expr {
    pub fn from_opn(op: Operator2, args: Vec<Expr>) -> Self {
        if args.len() < 2 {
            unimplemented!();
        }
        let mut it = args.into_iter();
        let expr = Self::from_op(&op, it.next().unwrap(), it.next().unwrap());
        it.fold(expr, |left, right| Self::from_op(&op, left, right))
    }

    pub fn from_op(op: &Operator2, left: Expr, right: Expr) -> Self {
        match op {
            Operator2::Add => Self::add(left, right),
            Operator2::Sub => Self::sub(left, right),
            Operator2::Mul => Self::mul(left, right),
            Operator2::Div => Self::div(left, right),
            Operator2::Pow => Self::pow(left, right),
            Operator2::Rem => Self::rem(left, right),
            Operator2::And => Self::and(left, right),
            Operator2::Or => Self::or(left, right),

            Operator2::Equal => Self::eq(left, right),
            Operator2::NotEqual => Self::ne(left, right),
            Operator2::GreaterEqual => Self::ge(left, right),
            Operator2::GreaterThan => Self::gt(left, right),
            Operator2::LessEqual => Self::le(left, right),
            Operator2::LessThan => Self::lt(left, right),
        }
    }
}

impl Expr {
    pub fn boxed(mut self) -> Self {
        match &mut self {
            Expr::Value { boxed, .. } => *boxed = true,
            _ => unimplemented!(),
        }
        self
    }

    pub fn is_const(&self) -> bool {
        match self {
            Expr::Operation1(_, item) => item.is_const(),
            Expr::Operation2(_, lhs, rhs) => lhs.is_const() && rhs.is_const(),
            Expr::Value { .. } => true,
            _ => false,
        }
    }

    pub fn eval_const(&self) -> Self {
        // TODO: check if expression is const and evaluate it
        unimplemented!()
    }

    pub fn dict() -> Self {
        use std::collections::HashMap;
        Expr::Value {
            val: CoValue::Dict(HashMap::new()),
            boxed: false,
        }
    }

    pub fn list() -> Self {
        Expr::Value {
            val: CoValue::List(vec![]),
            boxed: false,
        }
    }

    pub fn pow<T, U>(left: T, right: U) -> Self
    where
        T: Into<Expr>,
        U: Into<Expr>,
    {
        Expr::Operation2(
            Operator2::Pow,
            Box::new(left.into()),
            Box::new(right.into()),
        )
    }
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

impl From<Access> for Expr {
    fn from(access: Access) -> Expr {
        Expr::Access(access)
    }
}

impl From<Call> for Expr {
    fn from(call: Call) -> Expr {
        Expr::Call(call)
    }
}

impl From<Cast> for Expr {
    fn from(cast: Cast) -> Expr {
        Expr::Cast(cast)
    }
}

impl From<Initialize> for Expr {
    fn from(init: Initialize) -> Expr {
        Expr::DynamicValue(init)
    }
}

impl From<Slice> for Expr {
    fn from(slice: Slice) -> Expr {
        Expr::Slice(slice)
    }
}

impl<T> From<T> for Expr
where
    T: Into<CoValue>,
{
    fn from(val: T) -> Expr {
        Expr::Value {
            val: val.into(),
            boxed: false,
        }
    }
}

impl From<Variable> for Expr {
    fn from(v: Variable) -> Expr {
        Expr::Variable(v)
    }
}

impl Lowering for Expr {
    // TODO: add short-circuit for and (can be implemented via branching), or
    fn lower(self, runtime: &mut LoweringRuntime) {
        match self {
            Expr::Access(access) => {
                let variable = access.target;
                let mut key_it = access.keys.into_iter().peekable();

                // push (initial) target onto stack
                if runtime.has_local(&variable) {
                    let lidx = runtime.index_local(&variable);
                    runtime.emit(Instruction::Pushl(lidx as u16));
                } else {
                    let gidx = runtime.index_global(&variable);
                    runtime.emit(Instruction::Pushg(gidx as u16));
                }

                // push key onto stack
                let key = key_it.next().unwrap();
                key.lower(runtime);

                while key_it.peek().is_some() {
                    runtime.emit(Instruction::Get);
                    let key = key_it.next().unwrap();
                    key.lower(runtime);
                }

                runtime.emit(Instruction::Get);
            }
            Expr::Call(mut call) => {
                call.keep(true);
                call.lower(runtime);
            }
            Expr::Cast(cast) => cast.lower(runtime),
            Expr::DynamicValue(init) => init.lower(runtime),
            Expr::Operation1(op, expr) => {
                expr.lower(runtime);
                let inx = match op {
                    Operator1::Not => Instruction::Not,
                };
                runtime.emit(inx);
            }
            Expr::Operation2(op, expr1, expr2) => {
                expr1.lower(runtime);
                expr2.lower(runtime);
                let inx = match op {
                    Operator2::Add => Instruction::Add,
                    Operator2::Sub => Instruction::Sub,
                    Operator2::Mul => Instruction::Mul,
                    Operator2::Div => Instruction::Div,
                    Operator2::Pow => Instruction::Pow,
                    Operator2::Rem => Instruction::Rem,
                    Operator2::And => Instruction::And,
                    Operator2::Or => Instruction::Or,
                    Operator2::Equal => Instruction::Eq,
                    Operator2::NotEqual => Instruction::Ne,
                    Operator2::GreaterEqual => Instruction::Ge,
                    Operator2::GreaterThan => Instruction::Gt,
                    Operator2::LessEqual => Instruction::Le,
                    Operator2::LessThan => Instruction::Lt,
                };
                runtime.emit(inx);
            }
            Expr::Slice(slice) => slice.lower(runtime),
            Expr::Value { val, boxed } => {
                let cidx = runtime.index_const(&val);
                runtime.emit(Instruction::Pushc(cidx as u16));

                if boxed {
                    runtime.emit(Instruction::Box);
                }
            }
            Expr::Variable(ref var) => {
                if runtime.locals.contains(var) {
                    let lidx = runtime.index_local(var);
                    runtime.emit(Instruction::Pushl(lidx as u16));
                } else {
                    let gidx = runtime.index_global(var);
                    runtime.emit(Instruction::Pushg(gidx as u16));
                }
            }
        }
    }
}
