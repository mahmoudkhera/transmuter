use crate::{
    armdecoder::ArmInstruction,
    armprocessor::cpu::Condition,
    translate::{IRBuilder, IRProgram, emitargs::translate_arg},
};

// Translator
pub struct ArmTranslator {
    builder: IRBuilder,
    arm_insts: Vec<ArmInstruction>,
}

impl ArmTranslator {
    pub fn new(arm_insts: Vec<ArmInstruction>) -> Self {
        Self {
            builder: IRBuilder::new(),
            arm_insts: arm_insts,
        }
    }

    pub fn translate_instructions(&mut self) -> Result<(), String> {
        for arm_inst in &self.arm_insts {
            if arm_inst.cond != Condition::AL {
                let then_block = self.builder.create_blcok();
                let merge_block = self.builder.create_blcok();

                let cond_val = self.builder.emit_eval_condition(arm_inst.cond);
                self.builder
                    .emit_branch_cond(cond_val, then_block, merge_block);

                self.builder.switch_to_block(then_block);

                translate_arg(&mut self.builder, &arm_inst.inst)?;
                self.builder.emit_branch(merge_block);

                self.builder.switch_to_block(merge_block);
            } else {
                translate_arg(&mut self.builder, &arm_inst.inst)?;
            }
        }

        Ok(())
    }

    pub fn finalize(self) -> IRProgram {
        self.builder.finalize()
    }
}
