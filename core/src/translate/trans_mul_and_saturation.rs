//
//
//
//
//
//
//

use crate::{
    arm32arch::{Instruction, arg_rrrr, arg_s_rrrr},
    translate::IRBuilder,
};

pub fn translate_mul_and_saturation(
    ir_builder: &mut IRBuilder,
    inst: &Instruction,
) -> Result<(), String> {
    match inst {
        Instruction::MUL { args } => {
            translate_mul(ir_builder, args);
        }
        Instruction::MLA { args } => {
            translate_mla(ir_builder, args);
        }

        Instruction::MLS { args } => {
            translate_mls(ir_builder, args);
        }
        Instruction::UMAAL { args } => {
            translate_umaal(ir_builder, args);
        }
        Instruction::UMULL { args } => {
            translate_umull(ir_builder, args);
        }
        Instruction::UMLAL { args } => {
            translate_umlal(ir_builder, args);
        }
        Instruction::SMULL { args } => {
            translate_smull(ir_builder, args);
        }
        Instruction::SMLAL { args } => {
            translate_smlal(ir_builder, args);
        }
        _ => println!("not implemented yet"),
    }

    Ok(())
}

///  Translate  MUL: Rd = Rm * Rn
fn translate_mul(ir_builder: &mut IRBuilder, args: &arg_s_rrrr) {
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let rm = ir_builder.emit_load_reg(args.rm as u8);
    let res = ir_builder.emit_mul(rn, rm, args.s == 1);
    ir_builder.emit_store_reg(args.rd as u8, res);
}

///  Translate   MLA: Rd = (Rm * Rn) + Ra
fn translate_mla(ir_builder: &mut IRBuilder, args: &arg_s_rrrr) {
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let rm = ir_builder.emit_load_reg(args.rm as u8);
    let ra = ir_builder.emit_load_reg(args.ra as u8);

    let mul = ir_builder.emit_mla(rn, rm, args.s == 1);

    let res = ir_builder.emit_add(mul, ra, args.s == 1);
    ir_builder.emit_store_reg(args.rd as u8, res);
}

///  Translate   MLS: Rd = Ra - (Rm * Rn)
fn translate_mls(ir_builder: &mut IRBuilder, args: &arg_rrrr) {
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let rm = ir_builder.emit_load_reg(args.rm as u8);
    let ra = ir_builder.emit_load_reg(args.ra as u8);

    let mul = ir_builder.emit_mls(rn, rm);

    // Note that MLS never update the flag
    let res = ir_builder.emit_sub(ra, mul, false);
    ir_builder.emit_store_reg(args.rd as u8, res);
}

/// Translate UMULL: RdHi:RdLo = Rm * Rn (unsigned 64-bit result)

fn translate_umull(ir_builder: &mut IRBuilder, args: &arg_s_rrrr) {
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let rm = ir_builder.emit_load_reg(args.rm as u8);

    let mul = ir_builder.emit_umull(rn, rm, args.s == 1);

    ir_builder.emit_store_2regs(args.ra as u8, args.rd as u8, mul);
}

/// Translate UMLAL:RdHi:RdLo += Rn × Rm (unsigned 64-bit accumulate)
fn translate_umlal(ir_builder: &mut IRBuilder, args: &arg_s_rrrr) {
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let rm = ir_builder.emit_load_reg(args.rm as u8);

    let rd = ir_builder.emit_load_reg(args.rd as u8);
    let ra = ir_builder.emit_load_reg(args.ra as u8);

    let mul = ir_builder.emit_umlal(rn, rm, rd, ra, args.s == 1);

    ir_builder.emit_store_2regs(args.rd as u8, args.ra as u8, mul);
}

/// Translate SMULL: RdHi:RdLo = Rn × Rm (signed 64-bit)

fn translate_smull(ir_builder: &mut IRBuilder, args: &arg_s_rrrr) {
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let rm = ir_builder.emit_load_reg(args.rm as u8);

    let mul = ir_builder.emit_smull(rn, rm, args.s == 1);

    ir_builder.emit_store_2regs(args.rd as u8, args.ra as u8, mul);
}

/// Translate SMLAL: RdHi:RdLo += Rn × Rm (signed 64-bit accumulate)
fn translate_smlal(ir_builder: &mut IRBuilder, args: &arg_s_rrrr) {
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let rm = ir_builder.emit_load_reg(args.rm as u8);

    let rd = ir_builder.emit_load_reg(args.rd as u8);
    let ra = ir_builder.emit_load_reg(args.ra as u8);

    let mul = ir_builder.emit_smlal(rn, rm, rd, ra, args.s == 1);

    ir_builder.emit_store_2regs(args.rd as u8, args.ra as u8, mul);
}

///Translate UMAAL: RdHi:RdLo = (Rn × Rm) + RdLo + RdHi (unsigned)
fn translate_umaal(ir_builder: &mut IRBuilder, args: &arg_rrrr) {
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let rm = ir_builder.emit_load_reg(args.rm as u8);

    let rd = ir_builder.emit_load_reg(args.rd as u8);
    let ra = ir_builder.emit_load_reg(args.ra as u8);

    let mul = ir_builder.emit_umaal(rn, rm, rd, ra);

    ir_builder.emit_store_2regs(args.rd as u8, args.ra as u8, mul);
}
