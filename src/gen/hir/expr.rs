//! Expressions and operations that produce `Values`

use super::*;

use crate::vm::Context;

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

/// Expressions and operations that produce `Values`
#[derive(Clone, Debug)]
pub enum Expr {
    Access(Access),
    Call(Call),
    Conv(Conv),
    DynamicValue(Initialize),
    Iter(Iter),
    Operation1(Operator1, Box<Expr>),
    Operation2(Operator2, Box<Expr>, Box<Expr>),
    Slice(Slice),
    Value { val: Value, boxed: bool },
    Variable(Variable),
}

/// Operator with two operands
#[derive(Clone, Debug, PartialEq)]
pub enum Operator2 {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Rem,
    Shl,
    Shr,
    And,
    Or,
    XOr,

    Equal,
    NotEqual,
    GreaterEqual,
    GreaterThan,
    LessEqual,
    LessThan,
}

/// Operator with one operand
#[derive(Clone, Debug, PartialEq)]
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
            Operator2::Shl => Self::shl(left, right),
            Operator2::Shr => Self::shr(left, right),
            Operator2::And => Self::and(left, right),
            Operator2::Or => Self::or(left, right),
            Operator2::XOr => Self::xor(left, right),

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

    pub fn eval(&self, ctx: &Context) -> Lovm2Result<Value> {
        match self {
            Expr::Access(_) => todo!(),
            Expr::Call(_) => todo!(),
            Expr::Conv(_) => todo!(),
            Expr::DynamicValue(init) => {
                let mut base = init.base.clone();
                for (key, val) in init.slots.iter() {
                    let (key, val) = (key.eval(ctx)?, val.eval(ctx)?);
                    base.set(&key, val)?;
                }
                Ok(base)
            }
            Expr::Iter(_) => todo!(),
            Expr::Operation1(_, _) => todo!(),
            Expr::Operation2(_, _, _) => todo!(),

            Expr::Slice(_) => todo!(),
            Expr::Value { val, .. } => Ok(val.clone()),
            // TODO: wait until `result_cloned` is stabilized
            Expr::Variable(var) => Ok(ctx.value_of(&var)?.clone()),
        }
    }

    pub fn dict() -> Self {
        Expr::Value {
            val: Value::dict(),
            boxed: false,
        }
    }

    pub fn list() -> Self {
        Expr::Value {
            val: Value::list(),
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
    auto_implement!(2, Shl, shl);
    auto_implement!(2, Shr, shr);
    auto_implement!(2, And, and);
    auto_implement!(2, Or, or);
    auto_implement!(2, XOr, xor);

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

impl From<Conv> for Expr {
    fn from(conv: Conv) -> Expr {
        Expr::Conv(conv)
    }
}

impl From<Initialize> for Expr {
    fn from(init: Initialize) -> Expr {
        Expr::DynamicValue(init)
    }
}

impl From<Iter> for Expr {
    fn from(it: Iter) -> Expr {
        Expr::Iter(it)
    }
}

impl From<Slice> for Expr {
    fn from(slice: Slice) -> Expr {
        Expr::Slice(slice)
    }
}

impl<T> From<T> for Expr
where
    T: Into<Value>,
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

impl From<&Variable> for Expr {
    fn from(v: &Variable) -> Expr {
        Expr::Variable(v.clone())
    }
}

impl HirLowering for Expr {
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        match self {
            Expr::Access(ref access) => {
                let variable = &access.target;
                let mut key_it = access.keys.iter().peekable();

                // push (initial) target onto stack
                if runtime.has_local(&variable) {
                    runtime.emit(LirElement::push_dynamic(Scope::Local, variable));
                } else {
                    runtime.emit(LirElement::push_dynamic(Scope::Global, variable));
                }

                // push key onto stack
                if let Some(key) = key_it.next() {
                    key.lower(runtime);

                    while key_it.peek().is_some() {
                        runtime.emit(LirElement::Get);
                        let key = key_it.next().unwrap();
                        key.lower(runtime);
                    }

                    runtime.emit(LirElement::Get);
                }
            }
            Expr::Call(call) => call.lower(runtime),
            Expr::Conv(conv) => conv.lower(runtime),
            Expr::DynamicValue(init) => init.lower(runtime),
            Expr::Iter(it) => it.lower(runtime),
            Expr::Operation1(op, expr) => {
                expr.lower(runtime);
                runtime.emit(LirElement::operation(op));
            }
            Expr::Operation2(op, expr1, expr2) => {
                expr1.lower(runtime);

                // implement short-circuit for `And`/`Or`
                // generates a random label as jump target
                let sc_label = if matches!(op, Operator2::And | Operator2::Or) {
                    let sc_label = runtime.create_new_label();
                    // jump if first expression was already true
                    let cond = *op == Operator2::Or;

                    runtime.emit(LirElement::Duplicate);
                    runtime.emit(LirElement::jump_conditional(cond, sc_label.clone()));

                    Some(sc_label)
                } else {
                    None
                };

                expr2.lower(runtime);
                runtime.emit(LirElement::operation(op));

                // if we have a short-circuit label, lower it after the operation
                if let Some(sc_label) = sc_label {
                    runtime.emit(LirElement::Label(sc_label));
                }
            }
            Expr::Slice(slice) => slice.lower(runtime),
            Expr::Value { ref val, boxed } => {
                runtime.emit(LirElement::push_constant(val));

                if *boxed {
                    runtime.emit(LirElement::Box);
                }
            }
            Expr::Variable(ref var) => {
                if runtime.has_local(var) {
                    runtime.emit(LirElement::push_dynamic(Scope::Local, var));
                } else {
                    runtime.emit(LirElement::push_dynamic(Scope::Global, var));
                }
            }
        }
    }
}
