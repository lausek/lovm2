//! transform HIR into actual bytecode

pub mod branch;
pub mod repeat;
pub mod runtime;

use crate::bytecode::Instruction;

pub use branch::{HirLoweringBranch, HirLoweringCondition};
pub use repeat::HirLoweringLoop;
pub use runtime::HirLoweringRuntime;

pub trait HirLowering {
    fn lower(self, runtime: &mut HirLoweringRuntime);
}

pub fn patch_addrs(runtime: &mut HirLoweringRuntime, positions: &[usize], addr: usize) {
    for pos in positions.iter() {
        patch_addr(runtime, *pos, addr);
    }
}

pub fn patch_addr(runtime: &mut HirLoweringRuntime, position: usize, addr: usize) {
    let addr = addr as u16;
    if let Some(inx) = runtime.code.get_mut(position) {
        let unaddr = match inx {
            Instruction::Jmp(ref mut unaddr) => unaddr,
            Instruction::Jt(ref mut unaddr) => unaddr,
            Instruction::Jf(ref mut unaddr) => unaddr,
            _ => unimplemented!(),
        };
        if *unaddr != std::u16::MAX {
            panic!("address is already initialized");
        }
        *unaddr = addr;
    } else {
        unreachable!();
    }
}
