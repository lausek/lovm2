//! Expressions and operations that produce `Values`

use super::*;

use crate::value::ValueType;
use crate::vm::Context;

macro_rules! auto_implement {
    (1, $operator:ident, $method:ident) => {
        pub fn $method<T>(expr: T) -> Self
        where
            T: Into<Self>,
        {
            Self::Operation1(LV2Operator1::$operator, Box::new(expr.into()))
        }
    };
    (2, $operator:ident, $method:ident) => {
        pub fn $method<T, U>(left: T, right: U) -> Self
        where
            T: Into<Self>,
            U: Into<Self>,
        {
            Self::Operation2(
                LV2Operator2::$operator,
                Box::new(left.into()),
                Box::new(right.into()),
            )
        }
    };
}

/// Expressions and operations that produce `Values`
#[derive(Clone, Debug)]
pub enum LV2Expr {
    Append {
        base: Box<LV2Expr>,
        value: Box<LV2Expr>,
    },
    Branch(Box<ExprBranch>),
    Call(LV2Call),
    /// Do type conversion on a lowered `Expr` at runtime
    Conv {
        ty: ValueType,
        expr: Box<LV2Expr>,
    },
    /// Consecutive read on a `List` or `Dict`
    Get {
        target: Box<LV2Expr>,
        key: Box<LV2Expr>,
    },
    // TODO: change name to `Set`
    Insert {
        base: Box<LV2Expr>,
        key: Box<LV2Expr>,
        value: Box<LV2Expr>,
    },
    /// Create an iterator from some collection
    IterCreate {
        expr: Box<LV2Expr>,
    },
    /// Create an iterator for a range
    IterCreateRanged {
        from: Box<LV2Expr>,
        to: Box<LV2Expr>,
    },
    IterHasNext {
        expr: Box<LV2Expr>,
    },
    IterNext {
        expr: Box<LV2Expr>,
    },
    IterReverse {
        expr: Box<LV2Expr>,
    },
    Operation1(LV2Operator1, Box<LV2Expr>),
    Operation2(LV2Operator2, Box<LV2Expr>, Box<LV2Expr>),
    Slice {
        target: Box<LV2Expr>,
        start: Box<LV2Expr>,
        end: Box<LV2Expr>,
    },
    Value {
        val: Value,
        boxed: bool,
    },
    Variable(LV2Variable),
}

/// Operator with two operands
#[derive(Clone, Debug, PartialEq)]
pub enum LV2Operator2 {
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
pub enum LV2Operator1 {
    Not,
}

impl LV2Expr {
    pub fn append<T: Into<LV2Expr>>(self, value: T) -> Self {
        LV2Expr::Append {
            base: Box::new(self.boxed()),
            value: Box::new(value.into()),
        }
    }

    pub fn boxed(mut self) -> Self {
        match &mut self {
            LV2Expr::Value { boxed, .. } => *boxed = true,
            _ => {},
        }
        self
    }

    pub fn branch() -> ExprBranchIncomplete {
        ExprBranchIncomplete::new()
    }

    pub fn dict() -> Self {
        LV2Expr::Value {
            val: Value::dict(),
            boxed: false,
        }
    }

    pub fn get<T: Into<LV2Expr>>(self, key: T) -> Self {
        LV2Expr::Get {
            key: Box::new(key.into()),
            target: Box::new(self),
        }
    }

    pub fn eval(&self, ctx: &Context) -> Lovm2Result<Value> {
        match self {
            LV2Expr::Append { base, value } => {
                let mut base = base.eval(ctx)?;
                let (key, value) = (base.len()? as i64, value.eval(ctx)?);
                base.set(&key.into(), value)?;
                Ok(base)
            }
            LV2Expr::Branch(_) => todo!(),
            LV2Expr::Call(_) => todo!(),
            LV2Expr::Conv { .. } => todo!(),
            LV2Expr::Get { .. } => todo!(),
            LV2Expr::Insert { base, key, value } => {
                let mut base = base.eval(ctx)?;
                let (key, value) = (key.eval(ctx)?, value.eval(ctx)?);
                base.set(&key, value)?;
                Ok(base)
            }
            LV2Expr::IterCreate { .. }
            | LV2Expr::IterCreateRanged { .. }
            | LV2Expr::IterHasNext { .. }
            | LV2Expr::IterNext { .. }
            | LV2Expr::IterReverse { .. } => {
                todo!()
            }
            LV2Expr::Operation1(_, _) => todo!(),
            LV2Expr::Operation2(_, _, _) => todo!(),

            LV2Expr::Slice { .. } => todo!(),
            LV2Expr::Value { val, .. } => Ok(val.clone()),
            // TODO: wait until `result_cloned` is stabilized
            LV2Expr::Variable(var) => Ok(ctx.value_of(&var)?.clone()),
        }
    }

    pub fn from_opn(op: LV2Operator2, args: Vec<LV2Expr>) -> Self {
        if args.len() < 2 {
            unimplemented!();
        }

        let mut it = args.into_iter();
        let expr = Self::from_op(&op, it.next().unwrap(), it.next().unwrap());

        it.fold(expr, |left, right| Self::from_op(&op, left, right))
    }

    pub fn from_op(op: &LV2Operator2, left: LV2Expr, right: LV2Expr) -> Self {
        match op {
            LV2Operator2::Add => Self::add(left, right),
            LV2Operator2::Sub => Self::sub(left, right),
            LV2Operator2::Mul => Self::mul(left, right),
            LV2Operator2::Div => Self::div(left, right),
            LV2Operator2::Pow => Self::pow(left, right),
            LV2Operator2::Rem => Self::rem(left, right),
            LV2Operator2::Shl => Self::shl(left, right),
            LV2Operator2::Shr => Self::shr(left, right),
            LV2Operator2::And => Self::and(left, right),
            LV2Operator2::Or => Self::or(left, right),
            LV2Operator2::XOr => Self::xor(left, right),

            LV2Operator2::Equal => Self::eq(left, right),
            LV2Operator2::NotEqual => Self::ne(left, right),
            LV2Operator2::GreaterEqual => Self::ge(left, right),
            LV2Operator2::GreaterThan => Self::gt(left, right),
            LV2Operator2::LessEqual => Self::le(left, right),
            LV2Operator2::LessThan => Self::lt(left, right),
        }
    }

    pub fn has_next(self) -> Self {
        LV2Expr::IterHasNext {
            expr: Box::new(self),
        }
    }

    pub fn is_const(&self) -> bool {
        match self {
            LV2Expr::Operation1(_, item) => item.is_const(),
            LV2Expr::Operation2(_, lhs, rhs) => lhs.is_const() && rhs.is_const(),
            LV2Expr::Value { .. } => true,
            _ => false,
        }
    }

    pub fn insert<T: Into<LV2Expr>, U: Into<LV2Expr>>(mut self, key: T, value: U) -> Self {
        if let LV2Expr::Value { val: Value::Dict(_) | Value::List(_), boxed} = &mut self {
            *boxed = true;
        }

        LV2Expr::Insert {
            base: Box::new(self),
            key: Box::new(key.into()),
            value: Box::new(value.into()),
        }
    }

    pub fn iter_ranged<T: Into<LV2Expr>, U: Into<LV2Expr>>(from: T, to: U) -> Self {
        LV2Expr::IterCreateRanged {
            from: Box::new(from.into()),
            to: Box::new(to.into()),
        }
    }

    pub fn list() -> Self {
        LV2Expr::Value {
            val: Value::list(),
            boxed: false,
        }
    }

    pub fn next(self) -> Self {
        LV2Expr::IterNext {
            expr: Box::new(self),
        }
    }

    pub fn pow<T, U>(left: T, right: U) -> Self
    where
        T: Into<LV2Expr>,
        U: Into<LV2Expr>,
    {
        LV2Expr::Operation2(
            LV2Operator2::Pow,
            Box::new(left.into()),
            Box::new(right.into()),
        )
    }

    pub fn reverse(self) -> Self {
        LV2Expr::IterReverse {
            expr: Box::new(self),
        }
    }

    pub fn slice<T: Into<LV2Expr>, U: Into<LV2Expr>>(self, start: T, end: U) -> Self {
        LV2Expr::Slice {
            target: Box::new(self),
            start: Box::new(start.into()),
            end: Box::new(end.into()),
        }
    }

    pub fn to_bool(self) -> Self {
        LV2Expr::Conv {
            ty: ValueType::Bool,
            expr: Box::new(self),
        }
    }

    pub fn to_float(self) -> Self {
        LV2Expr::Conv {
            ty: ValueType::Float,
            expr: Box::new(self),
        }
    }

    pub fn to_integer(self) -> Self {
        LV2Expr::Conv {
            ty: ValueType::Int,
            expr: Box::new(self),
        }
    }

    pub fn to_iter(self) -> Self {
        LV2Expr::IterCreate {
            expr: Box::new(self),
        }
    }

    pub fn to_str(self) -> Self {
        LV2Expr::Conv {
            ty: ValueType::Str,
            expr: Box::new(self),
        }
    }
}

impl LV2Expr {
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

impl From<ExprBranch> for LV2Expr {
    fn from(branch: ExprBranch) -> LV2Expr {
        LV2Expr::Branch(Box::new(branch))
    }
}

impl From<LV2Call> for LV2Expr {
    fn from(call: LV2Call) -> LV2Expr {
        LV2Expr::Call(call)
    }
}

impl<T> From<T> for LV2Expr
where
    T: Into<Value>,
{
    fn from(val: T) -> LV2Expr {
        LV2Expr::Value {
            val: val.into(),
            boxed: false,
        }
    }
}

impl From<LV2Variable> for LV2Expr {
    fn from(v: LV2Variable) -> LV2Expr {
        LV2Expr::Variable(v)
    }
}

impl From<&LV2Variable> for LV2Expr {
    fn from(v: &LV2Variable) -> LV2Expr {
        LV2Expr::Variable(v.clone())
    }
}

impl HirLowering for LV2Expr {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    {
        match self {
            LV2Expr::Append { base, value } => {
                base.lower(runtime);
                runtime.emit(LirElement::Duplicate);
                value.lower(runtime);
                runtime.emit(LirElement::Append);
            }
            LV2Expr::Branch(branch) => branch.lower(runtime),
            LV2Expr::Call(call) => call.lower(runtime),
            LV2Expr::Conv { ty, expr } => {
                expr.lower(runtime);
                runtime.emit(LirElement::Conv { ty: ty.clone() });
            }
            LV2Expr::Get { target, key } => {
                target.lower(runtime);
                key.lower(runtime);
                runtime.emit(LirElement::RGet);
            }
            LV2Expr::Insert { base, key, value } => {
                lower_insert(runtime, base, key, value);
            }
            LV2Expr::IterCreate { expr } => {
                expr.lower(runtime);
                runtime.emit(LirElement::IterCreate);
            }
            LV2Expr::IterCreateRanged { from, to } => {
                from.lower(runtime);
                to.lower(runtime);
                runtime.emit(LirElement::IterCreateRanged);
            }
            LV2Expr::IterHasNext { expr } => {
                expr.lower(runtime);
                runtime.emit(LirElement::IterHasNext);
            }
            LV2Expr::IterNext { expr } => {
                expr.lower(runtime);
                runtime.emit(LirElement::IterNext);
            }
            LV2Expr::IterReverse { expr } => {
                expr.lower(runtime);
                runtime.emit(LirElement::IterReverse);
            }
            LV2Expr::Operation1(op, expr) => {
                expr.lower(runtime);
                runtime.emit(LirElement::operation(op));
            }
            LV2Expr::Operation2(op, expr1, expr2) => {
                expr1.lower(runtime);

                // implement short-circuit for `And`/`Or`
                // generates a random label as jump target
                let sc_label = if matches!(op, LV2Operator2::And | LV2Operator2::Or) {
                    let sc_label = runtime.create_new_label();
                    // jump if first expression was already true
                    let cond = *op == LV2Operator2::Or;

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
            LV2Expr::Slice { target, start, end } => lower_slice(runtime, target, start, end),
            LV2Expr::Value { ref val, boxed } => {
                runtime.emit(LirElement::push_constant(val));

                if *boxed || matches!(val, Value::Dict(_) | Value::List(_)) {
                    runtime.emit(LirElement::Box);
                }
            }
            LV2Expr::Variable(ref var) => {
                runtime.emit(LirElement::push_dynamic(var));
            }
        }
    }
}

pub struct ExprBranchIncomplete {
    branches: Vec<(LV2Expr, LV2Expr)>,
}

impl ExprBranchIncomplete {
    pub fn new() -> Self {
        Self { branches: vec![] }
    }

    pub fn add_condition<T, U>(mut self, condition: T, value: U) -> Self
    where
        T: Into<LV2Expr>,
        U: Into<LV2Expr>,
    {
        self.branches.push((condition.into(), value.into()));
        self
    }

    pub fn default_value<T>(self, default: T) -> ExprBranch
    where
        T: Into<LV2Expr>,
    {
        ExprBranch {
            branches: self.branches,
            default: Some(default.into()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExprBranch {
    branches: Vec<(LV2Expr, LV2Expr)>,
    default: Option<LV2Expr>,
}

impl ExprBranch {
    pub fn add_condition<T, U>(mut self, condition: T, value: U) -> Self
    where
        T: Into<LV2Expr>,
        U: Into<LV2Expr>,
    {
        self.branches.push((condition.into(), value.into()));
        self
    }

    pub fn default_value<T>(mut self, default: T) -> Self
    where
        T: Into<LV2Expr>,
    {
        self.default = Some(default.into());
        self
    }
}

impl HirLowering for ExprBranch {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    {
        super::branch::lower_map_structure(runtime, &self.branches, &self.default);
    }
}

fn lower_insert<'lir, 'hir: 'lir>(
    runtime: &mut HirLoweringRuntime<'lir>,
    base: &'hir LV2Expr,
    key: &'hir LV2Expr,
    value: &'hir LV2Expr,
)
{
    base.lower(runtime);

    runtime.emit(LirElement::Duplicate);
    key.lower(runtime);
    runtime.emit(LirElement::RGet);

    value.lower(runtime);
    runtime.emit(LirElement::Set);
}

fn lower_slice<'lir, 'hir: 'lir>(
    runtime: &mut HirLoweringRuntime<'lir>,
    target: &'hir LV2Expr,
    start: &'hir LV2Expr,
    end: &'hir LV2Expr,
)
{
    target.lower(runtime);
    start.lower(runtime);
    end.lower(runtime);
    runtime.emit(LirElement::Slice);
}