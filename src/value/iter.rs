use std::convert::TryFrom;

use crate::vm::Vm;

use super::*;

#[derive(Clone, Debug)]
enum IterType {
    Limit(usize),
    Open,
    Over(Value),
}

#[derive(Clone, Debug)]
pub struct Iter {
    current: usize,
    ty: IterType,
}

impl Iter {
    pub fn ranged(from: i64, to: i64) -> Self {
        Self {
            current: from as usize,
            ty: IterType::Limit(to as usize),
        }
    }

    pub fn ranged_from(from: i64) -> Self {
        Self {
            current: from as usize,
            ty: IterType::Open,
        }
    }

    pub fn ranged_to(to: i64) -> Self {
        Self {
            current: 0,
            ty: IterType::Limit(to as usize),
        }
    }

    pub fn has_next(&self) -> bool {
        match &self.ty {
            IterType::Limit(limit) => self.current < *limit,
            IterType::Open => true,
            IterType::Over(val) => val.get_by_index(self.current).is_ok(),
        }
    }

    pub fn next(&mut self) -> Lovm2Result<Value> {
        let idx = self.current;

        let val = match &self.ty {
            IterType::Limit(_) if self.has_next() => (idx as i64).into(),
            IterType::Open => (idx as i64).into(),
            IterType::Over(val) => val.get_by_index(idx)?,
            _ => Err(Lovm2Error::from("iterator exhausted"))?,
        };

        self.current += 1;
        Ok(val)
    }

    pub fn reverse(&mut self) {
        todo!()
    }

    pub fn collect(mut self) -> Vec<Value> {
        let mut result = vec![];

        while self.has_next() {
            result.push(self.next().unwrap());
        }

        result
    }
}

impl TryFrom<Value> for Iter {
    type Error = Lovm2Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let _ = value.len()?;
        Ok(Self {
            current: 0,
            ty: IterType::Over(value),
        })
    }
}

macro_rules! get_iter {
    ($vm:expr, $name:ident) => {
        let $name = $vm.context_mut().pop_value()?.as_any_inner()?;
        let mut $name = $name.borrow_mut();
        let $name = $name
            .0
            .downcast_mut::<Iter>()
            .ok_or_else(|| Lovm2Error::from("not an iterator"))?;
    };
}

pub(crate) fn vm_iter_create(vm: &mut Vm) -> Lovm2Result<()> {
    let from = vm.context_mut().pop_value()?;

    if from
        .as_any_inner()
        .map(|any| any.borrow().0.is::<Iter>())
        .unwrap_or(false)
    {
        vm.context_mut().push_value(from);
    } else {
        let it = from.iter()?;
        vm.context_mut().push_value(Value::create_any(it));
    }

    Ok(())
}

pub(crate) fn vm_iter_create_ranged(vm: &mut Vm) -> Lovm2Result<()> {
    let from = vm.context_mut().pop_value()?;
    let to = vm.context_mut().pop_value()?;

    let it = match (from, to) {
        (Value::Nil, Value::Nil) => unimplemented!(),
        (Value::Nil, to) => {
            let to = to.as_integer_inner()?;
            Iter::ranged_to(to)
        }
        (from, Value::Nil) => {
            let from = from.as_integer_inner()?;
            Iter::ranged_from(from)
        }
        (from, to) => {
            let (from, to) = (from.as_integer_inner()?, to.as_integer_inner()?);
            Iter::ranged(from, to)
        }
    };

    vm.context_mut().push_value(Value::create_any(it));

    Ok(())
}

pub(crate) fn vm_iter_has_next(vm: &mut Vm) -> Lovm2Result<()> {
    get_iter!(vm, it);
    vm.context_mut().push_value(it.has_next().into());
    Ok(())
}

pub(crate) fn vm_iter_next(vm: &mut Vm) -> Lovm2Result<()> {
    get_iter!(vm, it);
    vm.context_mut().push_value(it.next()?.into());
    Ok(())
}

pub(crate) fn vm_iter_reverse(_vm: &mut Vm) -> Lovm2Result<()> {
    todo!()
}
