# Virtual Machine

The virtual machine is the heart of `lovm2` projects and thrives computation forward. It maintains the whole program state inside a `LV2Context` and shares said data with every function and module interested in it.

## Context

The context stores the programs state.

``` rust,no_run
pub struct LV2Context {
    /// the module that will be run first
    entry: Option<Rc<dyn LV2CallProtocol>>,
    /// available functions
    scope: HashMap<LV2Variable, LV2CallableRef>,
    /// global variables
    globals: HashMap<LV2Variable, LV2Value>,
    /// call stack with local variables
    lstack: Vec<LV2Frame>,
    /// value stack
    vstack: Vec<LV2Value>,
}
```
