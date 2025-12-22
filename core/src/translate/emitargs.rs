use crate::{
    arm32arch::{Instruction, arg_s_rri_rot, arg_s_rrr_shi},
    translate::{Flag, IRBuilder, IROp},
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
        Instruction::ADD_rrri { args } => {
            emit_add_rrri(ir_builder, args);
        }
        Instruction::MOV_rxi { args } => {
            emit_mov_rxi(ir_builder, args);
        }
        _ => println!("Unkown instruction"),
    }

    Ok(())
}

fn emit_add_rrri(ir_builder: &mut IRBuilder, args: &arg_s_rrr_shi) {
    let rm_val = ir_builder.emit_load_reg(args.rm as u8);

    let res = rrri_shift(ir_builder, rm_val, args.shim, args.shty);

    let result = ir_builder.emit_add(args.rn, res);
    ir_builder.emit_store_reg(args.rd as u8, result);

    if args.s == 1 {
        ir_builder.emit(IROp::SetFlag(Flag::Z), vec![result]);
        ir_builder.emit(IROp::SetFlag(Flag::N), vec![result]);
    }
}

/// rd=ROR(imm, rot × 2)
pub fn emit_mov_rxi(ir_builder: &mut IRBuilder, args: &arg_s_rri_rot) {
    let imm32 = ror32(args.imm, args.rot * 2);

    let result = ir_builder.emit_const(imm32);
    ir_builder.emit_store_reg(args.rd as u8, result);

    if args.s == 1 {
        ir_builder.emit(IROp::SetFlag(Flag::Z), vec![result]);
        ir_builder.emit(IROp::SetFlag(Flag::N), vec![result]);
    }
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
