use std::convert::TryFrom;

use crate::vm::Vm;

use super::*;

#[derive(Clone, Debug, PartialEq)]
enum IterType {
    Limit(i64),
    Open,
    Over(Value),
}

#[derive(Clone, Debug)]
pub struct Iter {
    current: i64,
    reversed: bool,
    ty: IterType,
}

impl Iter {
    pub fn ranged(from: i64, to: i64) -> Self {
        if to < from {
            Iter::ranged(to, from).reverse()
        } else {
            Self {
                current: from,
                ty: IterType::Limit(to),
                ..Self::default()
            }
        }
    }

    pub fn ranged_from(from: i64) -> Self {
        Self {
            current: from,
            ..Self::default()
        }
    }

    pub fn ranged_to(to: i64) -> Self {
        Self {
            ty: IterType::Limit(to),
            ..Self::default()
        }
    }

    pub fn has_next(&self) -> bool {
        match &self.ty {
            // exclusive range. see `reverse()`
            IterType::Limit(limit) if !self.reversed => self.current < *limit,
            // inclusive range. see `reverse()`
            IterType::Limit(limit) => self.current >= *limit,
            IterType::Open => true,
            IterType::Over(val) => val.get_by_index(self.current as usize).is_ok(),
        }
    }

    pub fn next(&mut self) -> Lovm2Result<Value> {
        let idx = self.current;

        let val = match &self.ty {
            IterType::Limit(_) if self.has_next() => idx.into(),
            IterType::Open => idx.into(),
            IterType::Over(val) => val.get_by_index(idx as usize)?,
            _ => err_iterator_exhausted()?,
        };

        if !self.reversed {
            self.current += 1;
        } else {
            self.current -= 1;
        }

        Ok(val)
    }

    // by default `!reversed (default) => exclusive range`.
    //     example: 0..5 -> [0, 1, 2, 3, 4]
    //     reversing requirements: 0 < limit
    //     mapping rules (new <- old): current <- limit - 1, limit <- current
    //
    // otherwise `reversed => inclusive range`. example:
    //     4..=0 -> [4, 3, 2, 1, 0]
    //     mapping rules (new <- old): current <- limit, limit <- current + 1
    pub fn reverse(self) -> Self {
        let reversed = !self.reversed;

        match &self.ty {
            IterType::Limit(limit) if !self.reversed => Self {
                current: *limit - 1,
                ty: IterType::Limit(self.current),
                reversed,
            },
            IterType::Limit(limit) => Self {
                current: *limit,
                ty: IterType::Limit(self.current + 1),
                reversed,
            },
            IterType::Open => Self {
                current: self.current,
                ty: self.ty,
                reversed,
            },
            IterType::Over(val) => {
                let len = val.len().unwrap() as i64;
                let current = if self.current == 0 {
                    len - 1
                } else if self.current == len - 1 {
                    0
                } else {
                    panic!("iterator was moved")
                };
                Self {
                    current,
                    ty: self.ty,
                    reversed,
                }
            }
        }
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
        if let Value::Any(any) = &value {
            if let Some(value) = any.borrow().0.downcast_ref::<Iter>() {
                return Ok(value.clone());
            }
        }

        // values supporting len tend to support iteration as well
        let _ = value.len()?;
        Ok(Self {
            ty: IterType::Over(value),
            ..Self::default()
        })
    }
}

impl std::default::Default for Iter {
    fn default() -> Self {
        Self {
            current: 0,
            reversed: false,
            ty: IterType::Open,
        }
    }
}

macro_rules! get_iter {
    ($vm:expr, $name:ident) => {
        let $name = $vm.context_mut().pop_value()?.as_any_inner()?;
        let mut $name = $name.borrow_mut();
        let $name = $name
            .0
            .downcast_mut::<Iter>()
            .ok_or_else(|| err_ty_unexpected("iterator", "any"))?;
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
    let to = vm.context_mut().pop_value()?;
    let from = vm.context_mut().pop_value()?;

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
    vm.context_mut().push_value(it.next()?);
    Ok(())
}

pub(crate) fn vm_iter_reverse(vm: &mut Vm) -> Lovm2Result<()> {
    get_iter!(vm, it);
    let reversed = it.clone().reverse();
    vm.context_mut().push_value(Value::create_any(reversed));
    Ok(())
}
