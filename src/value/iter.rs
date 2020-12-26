use std::convert::TryFrom;
use std::rc::Rc;

use crate::vm::{InterruptFn, Vm};

use super::*;

#[derive(Clone, Debug)]
pub struct Iter {
    current: usize,
    limit: usize,
    value: Value,
}

impl Iter {
    pub fn has_next(&self) -> bool {
        self.value.get_by_index(self.current).is_ok()
    }

    pub fn next(&mut self) -> Lovm2Result<Value> {
        let idx = self.current;
        let val = self.value.get_by_index(idx)?;
        self.current += 1;
        Ok(val)
    }

    pub fn reverse(&mut self) {
        todo!()
    }
}

impl TryFrom<Value> for Iter {
    type Error = Lovm2Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let limit = value.len()?;
        Ok(Self {
            current: 0,
            limit,
            value,
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

fn int_iter_create(vm: &mut Vm) -> Lovm2Result<()> {
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

fn int_iter_has_next(vm: &mut Vm) -> Lovm2Result<()> {
    get_iter!(vm, it);
    vm.context_mut().push_value(it.has_next().into());
    Ok(())
}

fn int_iter_next(vm: &mut Vm) -> Lovm2Result<()> {
    get_iter!(vm, it);
    vm.context_mut().push_value(it.next()?.into());
    Ok(())
}

fn int_iter_reverse(_vm: &mut Vm) -> Lovm2Result<()> {
    todo!()
    /*
    let iter = get_iter!(vm)?;
    Ok(())
        */
}

pub(crate) fn register_iter_interrupts(table: &mut [Option<Rc<InterruptFn>>]) {
    use crate::vm::*;

    table[LOVM2_INT_ITER_CREATE as usize] = Some(Rc::new(int_iter_create));
    table[LOVM2_INT_ITER_HAS_NEXT as usize] = Some(Rc::new(int_iter_has_next));
    table[LOVM2_INT_ITER_NEXT as usize] = Some(Rc::new(int_iter_next));
    table[LOVM2_INT_ITER_REVERSE as usize] = Some(Rc::new(int_iter_reverse));
}
