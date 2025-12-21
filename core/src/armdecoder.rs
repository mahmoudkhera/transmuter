use crate::{
    arm32arch::{Instruction, decode_instruction},
    armmeme::{MemoryInterface, SimpleMemory},
    armprocessor::Condition,
};

#[derive(Clone)]
pub struct ArmInstruction {
    pub cond: Condition,
    pub inst: Instruction,
}

// Main Decode Context
#[derive(Clone)]
pub struct DecodeContext {
    // Basic state
    pub pc: u32,

    pub thumb: bool,
    pub cond: Condition,

    // Load/Store exclusive tracking
    pub is_ldex: bool,
}

impl DecodeContext {
    pub fn new(pc: u32) -> Self {
        Self {
            pc,

            thumb: false,
            cond: Condition::NE,

            is_ldex: false,
        }
    }
    /// Get current condition (handles IT blocks)
    pub fn get_condition(&self, inst: u32) -> Condition {
        Condition::from_u32(inst)
    }
    fn next_pc(&mut self) -> u32 {
        let pc = self.pc;
        self.pc += 1;
        pc
    }

    pub fn get_cond_inst(&mut self, meme: &mut SimpleMemory) -> Option<ArmInstruction> {
        match meme.read_u32(self.next_pc()) {
            Ok(meme) => {
                let cond = self.get_condition(meme);
                let decode_inst = decode_instruction(meme).unwrap();

                Some(ArmInstruction {
                    cond: cond,
                    inst: decode_inst,
                })
            }

            _ => None,
        }
    }
}
