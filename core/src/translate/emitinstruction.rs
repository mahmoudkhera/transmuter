use crate::{
    arm32arch::{Instruction, arg_s_rri_rot, arg_s_rrr_shi},
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
        Instruction::TST_xri { args } => {
            let imm32 = ror32(args.imm, args.rot * 2);
            let result = ir_builder.emit_const(imm32);
            let rn = ir_builder.emit_load_reg(args.rn as u8);
            ir_builder.emit_tst(rn, result);
        }
        Instruction::TEQ_xri { args } => {
            let imm32 = ror32(args.imm, args.rot * 2);
            let result = ir_builder.emit_const(imm32);
            let rn = ir_builder.emit_load_reg(args.rn as u8);
            ir_builder.emit_teq(rn, result);
        }
        Instruction::SUB_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_sub);
        }
        Instruction::ADD_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_add);
        }
        Instruction::ADC_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_adc);
        }
        Instruction::SBC_rri { args } => {
            emit_dp_immediate(ir_builder, args, IRBuilder::emit_sub);
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

        _ => println!("Unkown instruction"),
    }

    Ok(())
}

//# Data-processing (register)

type DpOperand = fn(&mut IRBuilder, u32, u32, bool) -> u32;
/// emit ir for instruction rd=rn (operation) (rm+shift +shfit type
pub fn emit_data_processing(ir_builder: &mut IRBuilder, args: &arg_s_rrr_shi, op: DpOperand) {
    let rm = ir_builder.emit_load_reg(args.rm as u8);
    let res = rrri_shift(ir_builder, rm, args.shim, args.shty);

    let rn = ir_builder.emit_load_reg(args.rn as u8);

    let result = op(ir_builder, rn, res, args.s == 1);
    ir_builder.emit_store_reg(args.rd as u8, result);
}

// # Data-processing (immediate)

type DpImm = fn(&mut IRBuilder, u32, u32, bool) -> u32;

pub fn emit_dp_immediate(ir_builder: &mut IRBuilder, args: &arg_s_rri_rot, op: DpImm) {
    let imm32 = ror32(args.imm, args.rot * 2);

    let result = ir_builder.emit_const(imm32);
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let result = op(ir_builder, rn, result, args.s == 1);
    ir_builder.emit_store_reg(args.rd as u8, result);
}

fn ror32(value: u32, shift: u32) -> u32 {
    let s = shift & 31; // ARM masks shift to 0–31
    if s == 0 {
        value
    } else {
        (value >> s) | (value << (32 - s))
    }
}

pub fn rrri_shift(ir_builder: &mut IRBuilder, rm: u32, shim: u32, shty: u32) -> u32 {
    match shty {
        0b00 => ir_builder.emit_lsl(rm, shty, shim),
        0b01 => ir_builder.emit_lsr(rm, shty, shim),
        0b10 => ir_builder.emit_asr(rm, shty, shim),
        0b11 => ir_builder.emit_ror(rm, shty, shim),
        _ => 0,
    }
}
