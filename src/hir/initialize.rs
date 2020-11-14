//! initialize complex objects (`Dict` and `List`), supports `Expr` as arguments

use std::collections::HashMap;

use crate::bytecode::Instruction;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Initialize {
    base: Value,
    slots: Vec<(Expr, Expr)>,
}

impl Initialize {
    pub fn new(base: Expr) -> Self {
        let base = if let Expr::Value { val, .. } = base {
            val
        } else {
            unimplemented!()
        };
        Self {
            base,
            slots: vec![],
        }
    }

    pub fn dict() -> Self {
        Self::new(Value::Dict(HashMap::new()).into())
    }

    pub fn list() -> Self {
        Self::new(Value::List(vec![]).into())
    }

    pub fn add<T>(&mut self, val: T)
    where
        T: Into<Expr>,
    {
        if let Value::List(list) = &mut self.base {
            match val.into() {
                Expr::Value { val, .. } => list.push(val),
                val => {
                    let key = list.len() as i64;
                    list.push(Value::Nil);
                    self.slots.push((key.into(), val));
                }
            }
        } else {
            unimplemented!()
        }
    }

    pub fn add_by_key<T, U>(&mut self, key: T, val: U)
    where
        T: Into<Expr>,
        U: Into<Expr>,
    {
        if let Value::Dict(dict) = &mut self.base {
            match (key.into(), val.into()) {
                (Expr::Value { val: key, .. }, Expr::Value { val, .. }) => {
                    dict.insert(key, val);
                }
                (key, val) => {
                    self.slots.push((key, val));
                }
            }
        } else {
            unimplemented!()
        }
    }
}

impl Lowering for Initialize {
    fn lower(self, runtime: &mut LoweringRuntime) {
        let requires_box = matches!(&self.base, Value::Dict(_) | Value::List(_));

        Expr::from(self.base).lower(runtime);

        if requires_box {
            runtime.emit(Instruction::Box);
        }

        // slots are only allowed on `Dict` and `List`
        for (key, expr) in self.slots.into_iter() {
            runtime.emit(Instruction::Dup);
            key.lower(runtime);
            runtime.emit(Instruction::Getr);
            expr.lower(runtime);
            runtime.emit(Instruction::Set);
        }
    }
}
