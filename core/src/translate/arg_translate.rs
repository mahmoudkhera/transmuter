use crate::{
    armdecoder::ArmInstruction,
    translate::{IRBuilder, trasn_data_processing::translate_data_processingarg},
};

pub fn translate_args(
    ir_builder: &mut IRBuilder,
    arm_instruction: &ArmInstruction,
) -> Result<(), String> {
    translate_data_processingarg(ir_builder, &arm_instruction.inst)?;

    Ok(())
}
