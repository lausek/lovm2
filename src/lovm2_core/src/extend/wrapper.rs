use std::rc::Rc;

use crate::code::{CallProtocol, CallableRef};
use crate::prelude::*;
use crate::vm::Vm;

struct StaticFunctionWrapper<T>(T);

impl<T> CallProtocol for StaticFunctionWrapper<T>
where
    T: Fn(&mut Vm) -> Lovm2Result<()>,
{
    fn run(&self, vm: &mut Vm) -> Lovm2Result<()> {
        self.0(vm)
    }
}

impl<T> std::fmt::Debug for StaticFunctionWrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<static_function>")
    }
}

/// Wrap a static function inside `Callable`.
pub fn create_callable(from: impl Fn(&mut Vm) -> Lovm2Result<()> + 'static) -> CallableRef {
    Rc::new(StaticFunctionWrapper(from)) as CallableRef
}
