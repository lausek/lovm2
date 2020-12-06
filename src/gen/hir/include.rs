//! loads a module by name into the vm

use super::*;

/// loads a module by name into the vm
#[derive(Clone)]
pub enum Include {
    Dynamic { name: Expr },
    Static { name: String },
}

impl Include {
    pub fn load<T>(name: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::Dynamic { name: name.into() }
    }

    pub fn load_static<T>(name: T) -> Self
    where
        T: Into<Expr>,
    {
        if let Expr::Value {
            val: Value::Str(name),
            ..
        } = name.into()
        {
            Self::Static { name }
        } else {
            unimplemented!()
        }
    }
}

impl HirLowering for Include {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        match self {
            Include::Dynamic { name } => {
                name.lower(runtime);
                runtime.emit(LirElement::Load);
            }
            Include::Static { name: _ } => todo!(),
        }
    }
}
