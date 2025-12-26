use crate::{
    arm32arch::{Instruction, arg_s_rri_rot, arg_s_rrr_shi, arg_s_rrr_shr},
    translate::IRBuilder,
};

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//

pub fn translate_arg(ir_builder: &mut IRBuilder, inst: &Instruction) -> Result<(), String> {
    println!("ars {:?}", inst);

    match inst {
        // ---------- Data-processing (immediate)----------//
        Instruction::AND_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_and);
        }
        Instruction::EOR_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_eor);
        }
        Instruction::ORR_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_orr);
        }
        Instruction::BIC_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_bic);
        }

        Instruction::ADD_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_add);
        }
        Instruction::ADC_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_adc);
        }

        Instruction::SUB_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_sub);
        }
        Instruction::SBC_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_sbc);
        }
        Instruction::RSC_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_rsc);
        }
        Instruction::RSB_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_rsb);
        }
        Instruction::MOV_rxi { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_mov);
        }
        Instruction::MVN_rxi { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_mvn);
        }
        Instruction::TST_xri { args } => {
            update_flag_immediate(ir_builder, args, IRBuilder::emit_tst);
        }
        Instruction::TEQ_xri { args } => {
            update_flag_immediate(ir_builder, args, IRBuilder::emit_teq);
        }

        Instruction::CMP_xri { args } => {
            update_flag_immediate(ir_builder, args, IRBuilder::emit_cmp);
        }
        Instruction::CMN_xri { args } => {
            update_flag_immediate(ir_builder, args, IRBuilder::emit_cmn);
        }

        // ---------- Data-processing ----------//
        Instruction::AND_rrri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_and);
        }
        Instruction::EOR_rrri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_eor);
        }
        Instruction::ORR_rrri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_orr);
        }
        Instruction::BIC_rrri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_bic);
        }

        Instruction::SUB_rrri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_sub);
        }
        Instruction::SBC_rrri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_sbc);
        }
        Instruction::ADD_rrri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_add);
        }
        Instruction::ADC_rrri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_adc);
        }
        Instruction::RSB_rrri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_rsb);
        }
        Instruction::RSC_rrri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_rsc);
        }
        Instruction::MOV_rxri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_mov);
        }
        Instruction::MVN_rxri { args } => {
            emit_data_processing(ir_builder, args, IRBuilder::emit_mvn);
        }
        Instruction::TST_xrri { args } => {
            update_flag_dp(ir_builder, args, IRBuilder::emit_tst);
        }
        Instruction::TEQ_xrri { args } => {
            update_flag_dp(ir_builder, args, IRBuilder::emit_teq);
        }
        Instruction::CMP_xrri { args } => {
            update_flag_dp(ir_builder, args, IRBuilder::emit_cmp);
        }
        Instruction::CMN_xrri { args } => {
            update_flag_dp(ir_builder, args, IRBuilder::emit_cmn);
        }

        //------------- Data-processing (register-shifted register)-------------//
        Instruction::AND_rrrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_and);
        }
        Instruction::EOR_rrrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_eor);
        }
        Instruction::ORR_rrrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_orr);
        }
        Instruction::BIC_rrrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_bic);
        }
        Instruction::SUB_rrrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_sub);
        }
        Instruction::SBC_rrrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_sbc);
        }
        Instruction::ADD_rrrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_add);
        }
        Instruction::ADC_rrrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_adc);
        }
        Instruction::RSB_rrrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_rsb);
        }
        Instruction::RSC_rrrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_rsc);
        }
        Instruction::MOV_rxrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_mov);
        }
        Instruction::MVN_rxrr { args } => {
            emit_data_shiftedreg_processing(ir_builder, args, IRBuilder::emit_mvn);
        }
        Instruction::TST_xrrr { args } => {
            update_flag_dp_shifted_reg(ir_builder, args, IRBuilder::emit_tst);
        }
        Instruction::TEQ_xrrr { args } => {
            update_flag_dp_shifted_reg(ir_builder, args, IRBuilder::emit_teq);
        }
        Instruction::CMP_xrrr { args } => {
            update_flag_dp_shifted_reg(ir_builder, args, IRBuilder::emit_cmp);
        }
        Instruction::CMN_xrrr { args } => {
            update_flag_dp_shifted_reg(ir_builder, args, IRBuilder::emit_cmn);
        }

        _ => println!("Unkown instruction"),
    }

    Ok(())
}

//# Data-processing (register)

type DpOperand = fn(&mut IRBuilder, u32, u32, bool) -> u32;
/// emit ir for instruction rd=rn (operation) (rm+shift +shfit type
pub fn emit_data_processing(ir_builder: &mut IRBuilder, args: &arg_s_rrr_shi, op: DpOperand) {
    let rm = ir_builder.emit_load_reg(args.rm as u8);
    let imm_value = ir_builder.emit_const(args.shim);

    let res = rrri_shift(ir_builder, rm, imm_value, args.shty);

    let rn = ir_builder.emit_load_reg(args.rn as u8);

    let result = op(ir_builder, rn, res, args.s == 1);
    ir_builder.emit_store_reg(args.rd as u8, result);
}
// emit update flag Data-processing (register)
type DpFlag = fn(&mut IRBuilder, u32, u32);

pub fn update_flag_dp(ir_builder: &mut IRBuilder, args: &arg_s_rrr_shi, op: DpFlag) {
    let rm = ir_builder.emit_load_reg(args.rm as u8);
    let result = rrri_shift(ir_builder, rm, args.shim, args.shty);

    let rn = ir_builder.emit_load_reg(args.rn as u8);
    op(ir_builder, rn, result);
}

// # Data-processing (register-shifted register)

type DpShiftedRegOperand = fn(&mut IRBuilder, u32, u32, bool) -> u32;
/// emit ir for instruction rd=rn (operation) (rm+shift +shfit type
pub fn emit_data_shiftedreg_processing(
    ir_builder: &mut IRBuilder,
    args: &arg_s_rrr_shr,
    op: DpShiftedRegOperand,
) {
    let rm = ir_builder.emit_load_reg(args.rm as u8);
    let rs = ir_builder.emit_load_reg(args.rs as u8);

    let res = rrri_shift(ir_builder, rm, rs, args.shty);

    let rn = ir_builder.emit_load_reg(args.rn as u8);

    let result = op(ir_builder, rn, res, args.s == 1);
    ir_builder.emit_store_reg(args.rd as u8, result);
}
// emit update flag Data-processing (register)
type DpShiftedRegFlag = fn(&mut IRBuilder, u32, u32);

fn update_flag_dp_shifted_reg(
    ir_builder: &mut IRBuilder,
    args: &arg_s_rrr_shr,
    op: DpShiftedRegFlag,
) {
    let rm = ir_builder.emit_load_reg(args.rm as u8);
    let rs = ir_builder.emit_load_reg(args.rs as u8);

    let res = rrri_shift(ir_builder, rm, rs, args.shty);

    let rn = ir_builder.emit_load_reg(args.rn as u8);
    op(ir_builder, rn, res);
}

// # Data-processing (immediate)

type DpImm = fn(&mut IRBuilder, u32, u32, bool) -> u32;

pub fn emit_dp_immediate(ir_builder: &mut IRBuilder, args: &arg_s_rri_rot, op: DpImm) {
    let imm_val = ir_builder.emit_const(args.imm);
    let rot_vlue = ir_builder.emit_const(args.rot);

    let result = ir_builder.emit_ror(imm_val, rot_vlue);

    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let result = op(ir_builder, rn, result, args.s == 1);
    ir_builder.emit_store_reg(args.rd as u8, result);
}
// emit update flag immediate
type DpImmFlag = fn(&mut IRBuilder, u32, u32);

pub fn update_flag_immediate(ir_builder: &mut IRBuilder, args: &arg_s_rri_rot, op: DpImmFlag) {
    let imm32 = ror32(args.imm, args.rot * 2);
    let result = ir_builder.emit_const(imm32);
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    op(ir_builder, rn, result);
}

fn ror32(value: u32, shift: u32) -> u32 {
    let s = shift & 31; // ARM masks shift to 0–31
    if s == 0 {
        value
    } else {
        (value >> s) | (value << (32 - s))
    }
}

pub fn rrri_shift(ir_builder: &mut IRBuilder, rm: u32, shift_value: u32, shty: u32) -> u32 {
    match shty {
        0b00 => ir_builder.emit_lsl(rm, shift_value),
        0b01 => ir_builder.emit_lsr(rm, shift_value),
        0b10 => ir_builder.emit_asr(rm, shift_value),
        0b11 => ir_builder.emit_ror(rm, shift_value),
        _ => 0,
    }
}
