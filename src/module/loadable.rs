use super::{CallableModule, GenericModule, /* Module, */ ModuleProtocol, SharedObjectModule};
use crate::code::CodeObject;

#[derive(Debug)]
pub enum LoadableModule {
    Generic(CallableModule),
    Lovm2(CodeObject),
    Shared(SharedObjectModule),
}

impl LoadableModule {
    pub fn into_generic(self) -> GenericModule {
        use std::rc::Rc;
        match self {
            Self::Generic(m) => Rc::new(m) as GenericModule,
            Self::Lovm2(m) => Rc::new(m) as GenericModule,
            Self::Shared(m) => Rc::new(m) as GenericModule,
        }
    }
}

impl std::ops::Deref for LoadableModule {
    type Target = dyn ModuleProtocol;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Generic(ref m) => m as &Self::Target,
            Self::Lovm2(ref m) => m as &Self::Target,
            Self::Shared(ref m) => m as &Self::Target,
        }
    }
}

impl From<CallableModule> for LoadableModule {
    fn from(m: CallableModule) -> Self {
        Self::Generic(m)
    }
}

impl From<CodeObject> for LoadableModule {
    fn from(m: CodeObject) -> Self {
        Self::Lovm2(m)
    }
}

impl From<SharedObjectModule> for LoadableModule {
    fn from(m: SharedObjectModule) -> Self {
        Self::Shared(m)
    }
}
