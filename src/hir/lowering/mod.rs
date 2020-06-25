pub mod branch;
pub mod repeat;
pub mod runtime;

use crate::bytecode::Instruction;

pub use branch::{LoweringBranch, LoweringCondition};
pub use repeat::LoweringLoop;
pub use runtime::LoweringRuntime;

pub trait Lowering {
    fn lower(self, runtime: &mut LoweringRuntime);
}

pub fn patch_addrs(runtime: &mut LoweringRuntime, positions: &Vec<usize>, addr: usize) {
    for pos in positions.iter() {
        patch_addr(runtime, *pos, addr);
    }
}

pub fn patch_addr(runtime: &mut LoweringRuntime, position: usize, addr: usize) {
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
