use crate::{
    arm32arch::arg_s_rrr_shi,
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
pub fn emit_add_rrri(ir_builder: &mut IRBuilder, arg: arg_s_rrr_shi) {
    let rm_val = ir_builder.emit_load_reg(arg.rm as u8);

    let res = rrri_shift(ir_builder, rm_val, arg.shim, arg.shty);

    let result = ir_builder.emit_add(arg.rn, res);
    ir_builder.emit_store_reg(arg.rd as u8, result);

    if arg.s == 1 {
        ir_builder.emit(IROp::SetFlag(Flag::Z), vec![result]);
        ir_builder.emit(IROp::SetFlag(Flag::N), vec![result]);
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
