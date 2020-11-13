use super::{GenericModule, Module, ModuleProtocol, SharedObjectModule};
use crate::code::NewCodeObject;

#[derive(Debug)]
pub enum LoadableModule {
    Lovm2(NewCodeObject),
    Shared(SharedObjectModule),
}

impl LoadableModule {
    pub fn into_generic(self) -> GenericModule {
        use std::rc::Rc;
        match self {
            Self::Lovm2(m) => Rc::new(m) as GenericModule,
            Self::Shared(m) => Rc::new(m) as GenericModule,
        }
    }
}

impl std::ops::Deref for LoadableModule {
    type Target = dyn ModuleProtocol;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Lovm2(ref m) => m as &Self::Target,
            Self::Shared(ref m) => m as &Self::Target,
        }
    }
}

impl From<NewCodeObject> for LoadableModule {
    fn from(m: NewCodeObject) -> Self {
        Self::Lovm2(m)
    }
}

impl From<SharedObjectModule> for LoadableModule {
    fn from(m: SharedObjectModule) -> Self {
        Self::Shared(m)
    }
}
