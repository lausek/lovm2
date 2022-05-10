pub mod transpiler;

use lovm2::prelude::{LV2Module, LV2ModuleMeta, LV2Result};

pub use crate::transpiler::Transpiler;

pub fn create_module(name: &str, src: &str) -> LV2Result<LV2Module> {
    let mut trans = Transpiler::new();
    let meta: LV2ModuleMeta = name.to_string().into();
    let module: LV2Module = trans.build(meta, src).unwrap();
    Ok(module)
}
