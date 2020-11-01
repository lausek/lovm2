//! highlevel intermediate representation
//!
//! ## Example
//!
//! ``` rust
//! use lovm2::prelude::*;
//!
//! fn main() {
//!     let mut main_hir = HIR::new();
//!
//!     // set the local variable `n` to 10
//!     main_hir.push(Assign::local(var!(n), 10));
//!
//!     // `print` is a builtin function. the `var!` macro
//!     // ensures that the given identifier is not confused
//!     // with a string.
//!     main_hir.push(Call::new("print").arg(var!(n)).arg("Hello World").arg("\n"));
//!     // ... this is equivalent to the developer friendly version:
//!     main_hir.push(call!(print, n, "Hello World", "\n"));
//!
//!     // create a branch
//!     let mut branch = Branch::new();
//!
//!     // if `n` is 5, print something
//!     let good_case = branch.add_condition(Expr::eq(var!(n), 5));
//!     good_case.push(call!(print, "n is 5", "\n"));
//!
//!     // ... print something if `n` is not 5
//!     let bad_case = branch.default_condition();
//!     bad_case.push(call!(print, "n is not 5", "\n"));
//!
//!     main_hir.push(branch);
//!
//!     // create a loop and repeat until `n` is 0
//!     let mut repeat = Repeat::until(Expr::eq(var!(n), 0));
//!     // decrement `n` by one
//!     repeat.push(Assign::local(var!(n), Expr::sub(var!(n), 1)));
//!     repeat.push(call!(print, "n is not 0 yet", "\n"));
//!
//!     main_hir.push(repeat);
//!
//!     let mut module = ModuleBuilder::new();
//!
//!     // a module needs a code object called `main`
//!     // if you want to make it runnable
//!     module.add("main").hir(main_hir);
//!
//!     // consumes the `ModuleBuilder` and transforms
//!     // it into a `Module`
//!     let module = module.build().unwrap();
//! }
//! ```

pub mod assign;
pub mod block;
pub mod branch;
pub mod call;
pub mod cast;
pub mod element;
pub mod expr;
pub mod include;
pub mod initialize;
pub mod interrupt;
pub mod lowering;
pub mod repeat;
pub mod r#return;
pub mod slice;

pub mod prelude;

use lovm2_error::*;

use crate::code::CodeObject;
use crate::hir::block::Block;
use crate::hir::element::HIRElement;
use crate::hir::lowering::LoweringRuntime;
use crate::hir::r#return::Return;
use crate::value::Value;
use crate::var::Variable;

#[derive(Clone)]
pub struct HIR {
    pub args: Vec<Variable>,
    pub consts: Vec<Value>,
    pub locals: Vec<Variable>,
    pub globals: Vec<Variable>,

    pub code: Block,
}

impl HIR {
    pub fn new() -> Self {
        Self {
            args: vec![],
            consts: vec![],
            locals: vec![],
            globals: vec![],

            code: Block::new(),
        }
    }

    pub fn with_args(args: Vec<Variable>) -> Self {
        let mut hir = Self::new();
        hir.args = args;
        hir
    }

    pub fn build(mut self) -> Lovm2CompileResult<CodeObject> {
        // automatically add a `return nil` if not present already
        match self.code.last_mut() {
            Some(HIRElement::Return(_)) => {}
            _ => self.code.push(Return::nil()),
        }
        // TODO: optimise codeobject here; eg. `Not, Jf` is equal to `Jt`
        LoweringRuntime::complete(self)
    }

    pub fn push<T>(&mut self, element: T)
    where
        T: Into<HIRElement>,
    {
        self.code.push(element.into());
    }
}
