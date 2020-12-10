# Context

The context stores the programs state.

``` rust,no_run
pub struct Context {
    /// loaded modules
    pub modules: HashMap<String, Rc<Module>>,
    /// the module that will be run first
    pub entry: Option<Rc<dyn CallProtocol>>,
    /// available functions
    pub scope: HashMap<Variable, CallableRef>,
    /// global variables
    pub globals: HashMap<Variable, Value>,
    /// call stack with local variables
    pub lstack: Vec<Frame>,
    /// value stack
    pub vstack: Vec<Value>,
}
```
