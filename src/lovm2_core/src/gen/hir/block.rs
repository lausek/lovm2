//! General collection for program statements

use super::*;

/// List of statements forming a code block
#[derive(Clone)]
pub struct LV2Block(Vec<LV2Statement>);

impl LV2Block {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn assign<T: Into<LV2Variable>, U: Into<LV2Expr>>(&mut self, target: T, source: U) {
        self.step(LV2Statement::AssignVariable {
            target: target.into(),
            source: source.into(),
        });
    }

    pub fn branch(&mut self) -> &mut LV2Branch {
        self.step(LV2Branch::new());

        match self.last_mut().unwrap() {
            LV2Statement::Branch(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn break_repeat(&mut self) {
        self.0.push(LV2Statement::Break);
    }

    pub fn continue_repeat(&mut self) {
        self.0.push(LV2Statement::Continue);
    }

    pub fn decrement(&mut self, target: &LV2Variable) {
        self.0.push(LV2Statement::AssignVariable {
            target: target.clone(),
            source: LV2Expr::from(target).sub(1),
        });
    }

    pub fn extend(&mut self, block: LV2Block) {
        self.0.extend(block.0);
    }

    pub fn global(&mut self, ident: &LV2Variable) {
        self.0.push(LV2Statement::ScopeGlobal {
            ident: ident.clone(),
        });
    }

    pub fn import<T: Into<LV2Expr>>(&mut self, name: T) {
        self.0.push(LV2Statement::Import {
            name: name.into(),
            namespaced: true,
        });
    }

    pub fn import_from<T: Into<LV2Expr>>(&mut self, name: T) {
        self.0.push(LV2Statement::Import {
            name: name.into(),
            namespaced: false,
        });
    }

    pub fn increment(&mut self, target: &LV2Variable) {
        self.0.push(LV2Statement::AssignVariable {
            target: target.clone(),
            source: LV2Expr::from(target).add(1),
        });
    }

    pub fn last_mut(&mut self) -> Option<&mut LV2Statement> {
        self.0.last_mut()
    }

    pub fn local(&mut self, ident: &LV2Variable) {
        self.0.push(LV2Statement::ScopeLocal {
            ident: ident.clone(),
        });
    }

    pub fn repeat(&mut self) -> &mut LV2Repeat {
        self.step(LV2Repeat::endless());

        match self.last_mut().unwrap() {
            LV2Statement::Repeat(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn repeat_until(&mut self, condition: LV2Expr) -> &mut LV2Repeat {
        self.step(LV2Repeat::until(condition));

        match self.last_mut().unwrap() {
            LV2Statement::Repeat(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn repeat_iterating<T: Into<LV2Expr>, U: Into<LV2Variable>>(
        &mut self,
        collection: T,
        item: U,
    ) -> &mut LV2Repeat {
        self.step(LV2Repeat::iterating(collection, item));

        match self.last_mut().unwrap() {
            LV2Statement::Repeat(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn return_nil(&mut self) {
        self.0.push(LV2Statement::Return {
            expr: LV2Value::Nil.into(),
        });
    }

    pub fn return_value<T: Into<LV2Expr>>(&mut self, expr: T) {
        self.0.push(LV2Statement::Return { expr: expr.into() });
    }

    pub fn set<T: Into<LV2Expr>, U: Into<LV2Expr>>(&mut self, target: T, source: U) {
        self.0.push(LV2Statement::AssignReference {
            target: target.into(),
            source: source.into(),
        });
    }

    pub fn step<T: Into<LV2Statement>>(&mut self, hir: T) {
        self.0.push(hir.into());
    }

    pub fn trigger(&mut self, n: u16) {
        self.0.push(LV2Statement::Interrupt { n });
    }
}

impl Default for LV2Block {
    fn default() -> Self {
        LV2Block::new()
    }
}

impl std::iter::IntoIterator for LV2Block {
    type Item = LV2Statement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl LV2HirLowering for LV2Block {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut LV2HirLoweringRuntime<'lir>) {
        for element in self.0.iter() {
            // every call has to leave a return value on stack.
            // if that value isn't needed - as in a statement position - we
            // need to get rid of it.
            let requires_drop = matches!(element, LV2Statement::Call(_));

            element.lower(runtime);

            if requires_drop {
                runtime.emit(LirElement::Drop);
            }
        }
    }
}
