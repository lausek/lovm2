use std::rc::Rc;

use crate::code::{LV2CallProtocol, LV2CallableRef};
use crate::prelude::*;
use crate::vm::Vm;

struct StaticFunctionWrapper<T>(T);

impl<T> LV2CallProtocol for StaticFunctionWrapper<T>
where
    T: Fn(&mut Vm) -> LV2Result<()>,
{
    fn run(&self, vm: &mut Vm) -> LV2Result<()> {
        self.0(vm)
    }
}

impl<T> std::fmt::Debug for StaticFunctionWrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<static_function>")
    }
}

/// Wrap a static function inside `Callable`.
pub fn create_callable(from: impl Fn(&mut Vm) -> LV2Result<()> + 'static) -> LV2CallableRef {
    Rc::new(StaticFunctionWrapper(from)) as LV2CallableRef
}
