use crate::{
    armdecoder::ArmInstruction,
    translate::{
        IRBuilder, trans_mul_and_saturation::translate_mul_and_saturation,
        trasn_data_processing::translate_data_processingarg,
    },
};

pub fn translate_args(
    ir_builder: &mut IRBuilder,
    arm_instruction: &ArmInstruction,
) -> Result<(), String> {
    println!("ars {:?}", arm_instruction.inst);
    translate_data_processingarg(ir_builder, &arm_instruction.inst);
    translate_mul_and_saturation(ir_builder, &arm_instruction.inst)?;

    Ok(())
}
