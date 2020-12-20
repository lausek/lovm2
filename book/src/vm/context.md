# Context

The context stores the programs state.

``` rust,no_run
pub struct Context {
    /// the module that will be run first
    pub entry: Option<Rc<dyn CallProtocol>>,
    /// available functions
    scope: HashMap<Variable, CallableRef>,
    /// global variables
    globals: HashMap<Variable, Value>,
    /// call stack with local variables
    lstack: Vec<Frame>,
    /// value stack
    vstack: Vec<Value>,
}
```
