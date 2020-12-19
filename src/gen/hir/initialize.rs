//! Initialize complex objects (`Dict` and `List`), supports `Expr` as arguments

use super::*;

/// Initialize complex objects (`Dict` and `List`), supports `Expr` as arguments
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
        Self::new(Value::dict().into())
    }

    pub fn list() -> Self {
        Self::new(Value::list().into())
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

impl HirLowering for Initialize {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        let requires_box = matches!(&self.base, Value::Dict(_) | Value::List(_));

        Expr::from(self.base).lower(runtime);

        if requires_box {
            runtime.emit(LirElement::Box);
        }

        // slots are only allowed on `Dict` and `List`
        for (key, expr) in self.slots.into_iter() {
            runtime.emit(LirElement::Duplicate);
            key.lower(runtime);
            runtime.emit(LirElement::RGet);
            expr.lower(runtime);
            runtime.emit(LirElement::Set);
        }
    }
}
