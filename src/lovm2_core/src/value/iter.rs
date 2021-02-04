use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::vm::Vm;

use super::*;

#[derive(Clone, Debug, PartialEq)]
enum IterType {
    Limit(i64),
    Open,
    Over(Value),
}

/// Runtime iterator implementation
#[derive(Clone, Debug)]
pub struct Iter {
    current: i64,
    reversed: bool,
    ty: IterType,
}

impl Iter {
    /// Create a number generator where `from` is inclusive and `to` is exclusive.
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

    /// Creates an endless number generator starting at `from` inclusive.
    pub fn ranged_from(from: i64) -> Self {
        Self {
            current: from,
            ..Self::default()
        }
    }

    /// Creates a number generator starting at 0 and running until `to` exclusive.
    pub fn ranged_to(to: i64) -> Self {
        Self {
            ty: IterType::Limit(to),
            ..Self::default()
        }
    }

    /// Returns true if `next()` can be called safely.
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

    /// Advance the iterator and get the next value. This will return an error
    /// if the iterator has no elements left.
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

    /// Consume the iterator into a vector of values.
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
        match &value {
            Value::Any(any) => {
                if let Some(value) = any.borrow().0.downcast_ref::<Iter>() {
                    return Ok(value.clone());
                }
            }
            Value::Iter(it) => return Ok(it.borrow().clone()),
            _ => {}
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

#[inline]
fn get_iter(vm: &mut Vm) -> Lovm2Result<Rc<RefCell<Iter>>> {
    match vm.context_mut().pop_value()? {
        Value::Iter(it) => Ok(it),
        val => return Err(err_ty_unexpected("iterator", format!("{:?}", val))),
    }
}

pub(crate) fn vm_iter_create(vm: &mut Vm) -> Lovm2Result<()> {
    let from = vm.context_mut().pop_value()?;

    // if the value on stack already is an iterator, leave it
    let it = if matches!(from, Value::Iter(_)) {
        from
    } else {
        Value::from(from.iter()?)
    };

    vm.context_mut().push_value(it);

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

    vm.context_mut().push_value(Value::from(it));

    Ok(())
}

pub(crate) fn vm_iter_has_next(vm: &mut Vm) -> Lovm2Result<()> {
    let it = get_iter(vm)?;
    let it = it.borrow();
    vm.context_mut().push_value(it.has_next().into());
    Ok(())
}

pub(crate) fn vm_iter_next(vm: &mut Vm) -> Lovm2Result<()> {
    let it = get_iter(vm)?;
    let mut it = it.borrow_mut();
    vm.context_mut().push_value(it.next()?);
    Ok(())
}

pub(crate) fn vm_iter_reverse(vm: &mut Vm) -> Lovm2Result<()> {
    let it = get_iter(vm)?;
    let it = it.borrow();
    let reversed = it.clone().reverse();
    vm.context_mut().push_value(Value::from(reversed));
    Ok(())
}

impl std::fmt::Display for Iter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "iterator")
    }
}
