//! General collection for program statements

use super::*;

/// List of statements forming a code block
#[derive(Clone)]
pub struct Block(Vec<HirElement>);

impl Block {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn assign<U, T>(&mut self, var: &U, expr: T)
    where
        U: Into<Variable> + Clone,
        T: Into<Expr>,
    {
        self.step(Assign::var(var, expr));
    }

    pub fn branch(&mut self) -> &mut Branch {
        self.step(Branch::new());

        match self.last_mut().unwrap() {
            HirElement::Branch(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn extend(&mut self, block: Block) {
        self.0.extend(block.0);
    }

    pub fn global(&mut self, ident: &Variable) {
        self.0.push(HirElement::ScopeGlobal { ident: ident.clone() });
    }

    pub fn import<T>(&mut self, name: T)
    where
        T: Into<Expr>,
    {
        self.0.push(HirElement::Import { name: name.into(), namespaced: true});
    }

    pub fn import_global<T>(&mut self, name: T)
    where
        T: Into<Expr>,
    {
        self.0.push(HirElement::Import { name: name.into(), namespaced: false});
    }

    pub fn last_mut(&mut self) -> Option<&mut HirElement> {
        self.0.last_mut()
    }

    pub fn local(&mut self, ident: &Variable) {
        self.0.push(HirElement::ScopeLocal { ident: ident.clone() });
    }

    pub fn repeat(&mut self) -> &mut Repeat {
        self.step(Repeat::endless());

        match self.last_mut().unwrap() {
            HirElement::Repeat(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn repeat_until(&mut self, condition: Expr) -> &mut Repeat {
        self.step(Repeat::until(condition));

        match self.last_mut().unwrap() {
            HirElement::Repeat(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn repeat_iterating<U, T>(&mut self, collection: U, item: T) -> &mut Repeat
    where
        U: Into<Expr>,
        T: Into<Variable>,
    {
        self.step(Repeat::iterating(collection, item));

        match self.last_mut().unwrap() {
            HirElement::Repeat(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn return_nil(&mut self) {
        self.0.push(HirElement::Return { expr: Value::Nil.into() });
    }

    pub fn return_value<T: Into<Expr>>(&mut self, expr: T) {
        self.0.push(HirElement::Return { expr: expr.into() });
    }

    pub fn step<T>(&mut self, hir: T)
    where
        T: Into<HirElement>,
    {
        self.0.push(hir.into());
    }

    pub fn trigger(&mut self, n: u16)
    {
        self.0.push(HirElement::Interrupt { n });
    }
}

impl Default for Block {
    fn default() -> Self {
        Block::new()
    }
}

impl std::iter::IntoIterator for Block {
    type Item = HirElement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl HirLowering for Block {
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        for element in self.0.iter() {
            // every call has to leave a return value on stack.
            // if that value isn't needed - as in a statement position - we
            // need to get rid of it.
            let requires_drop = matches!(element, HirElement::Call(_));

            element.lower(runtime);

            if requires_drop {
                runtime.emit(LirElement::Drop);
            }
        }
    }
}
