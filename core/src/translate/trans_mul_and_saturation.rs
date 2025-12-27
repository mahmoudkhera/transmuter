//
//
//
//
//
//
//

use crate::{
    arm32arch::{Instruction, arg_s_rrrr},
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
        // Instruction::MLA { args }{

        // }
        // Instruction::UMAAL { args }{

        // }
        // Instruction::MLS { args }{

        // }
        // Instruction::UMULL { args }
        // {

        // }
        // Instruction::UMLAL { args }{

        // }
        // Instruction::SMULL { args }{

        // }
        // Instruction::SMLAL { args }{

        // }
        _ => println!("not implemented yet"),
    }

    Ok(())
}

fn translate_mul(ir_builder: &mut IRBuilder, args: &arg_s_rrrr) {
    let rn = ir_builder.emit_load_reg(args.rn as u8);
    let rm = ir_builder.emit_load_reg(args.rm as u8);
    let res = ir_builder.emit_mul(rn, rm, args.s == 1);
    ir_builder.emit_store_reg(args.rd as u8, res);
}
