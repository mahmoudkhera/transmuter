use crate::{
    armprocessor::interpreter::IRInterpreter,
    translate::{IRInst, IROp},
};

pub fn execute_instruction(inst: &IRInst, interpreter: &mut IRInterpreter) -> Result<u32, String> {
    match inst.op {
        IROp::Const(val) => Ok(val),
        IROp::LoadReg(reg) => {
            let val = interpreter.cpu.read_reg(reg);

            Ok(val)
        }
        IROp::StoreReg(reg) => {
            let val = interpreter.get_vreg(inst.inputs[0])?;
            interpreter.cpu.write_reg(reg, val);
            println!("reg{}  = 0x{:x}", reg, val);

            Ok(inst.inputs[0])
        }
        IROp::And(s) => execute_logical(interpreter, &inst.inputs, IROp::And(s)),
        IROp::Orr(s) => execute_logical(interpreter, &inst.inputs, IROp::Orr(s)),
        IROp::Eor(s) => execute_logical(interpreter, &inst.inputs, IROp::Eor(s)),
        IROp::Bic(s) => execute_logical(interpreter, &inst.inputs, IROp::Bic(s)),
        IROp::Not(s) => execute_logical(interpreter, &inst.inputs, IROp::Not(s)),
        IROp::Tst => execute_logical(interpreter, &inst.inputs, IROp::Tst),
        IROp::Teq => execute_logical(interpreter, &inst.inputs, IROp::Teq),
        IROp::Add(s) => execute_arthimitic(interpreter, &inst.inputs, IROp::Add(s)),
        IROp::Adc(s) => execute_arthimitic(interpreter, &inst.inputs, IROp::Adc(s)),
        IROp::Sub(s) => execute_arthimitic(interpreter, &inst.inputs, IROp::Sub(s)),
        IROp::Sbc(s) => execute_arthimitic(interpreter, &inst.inputs, IROp::Sbc(s)),
        IROp::Rsb(s) => execute_arthimitic(interpreter, &inst.inputs, IROp::Rsb(s)),
        IROp::Rsc(s) => execute_arthimitic(interpreter, &inst.inputs, IROp::Rsc(s)),
        IROp::Cmp => execute_compare(interpreter, &inst.inputs, IROp::Cmp),
        IROp::Cmn => execute_compare(interpreter, &inst.inputs, IROp::Cmn),

        IROp::Mov(s) => {
            let val = interpreter.get_vreg(inst.inputs[1])?;
            if s {
                interpreter.cpu.cpsr.z = val == 0;
            }
            Ok(val)
        }
        IROp::Mvn(s) => {
            let val = !interpreter.get_vreg(inst.inputs[1])?;
            if s {
                interpreter.cpu.cpsr.z = val == 0;
            }
            Ok(val)
        }

        IROp::EvalCondition(cond) => {
            let result = interpreter.cpu.cpsr.evaluat_cond(cond);
            Ok(result as u32)
        }
        IROp::BranchCond(_) => {
            let cond = interpreter.get_vreg(inst.inputs[0])?;
            Ok(cond)
        }
        IROp::Lsl => execute_shift(interpreter, &inst.inputs, IROp::Lsl),
        IROp::Lsr => execute_shift(interpreter, &inst.inputs, IROp::Lsr),
        IROp::Asr => execute_shift(interpreter, &inst.inputs, IROp::Asr),
        IROp::Ror => execute_shift(interpreter, &inst.inputs, IROp::Ror),
        IROp::Branch(_) | IROp::Call(_) | IROp::Return | IROp::Nop => Ok(0),

        _ => Err(format!("not implemented or can not be excuted")),
    }
}

fn execute_arthimitic(
    interpreter: &mut IRInterpreter,
    inst_inputs: &Vec<u32>,
    ir: IROp,
) -> Result<u32, String> {
    let a_u32 = interpreter.get_vreg(inst_inputs[0])?;
    let b_u32 = interpreter.get_vreg(inst_inputs[1])?;

    let a = a_u32 as u64;
    let b = b_u32 as u64;
    let c_in = interpreter.cpu.cpsr.c as u64;

    let (s, result, carry, overflow) = match ir {
        IROp::Add(s) => {
            let wide = a + b;
            let res = wide as u32;
            let c = wide > 0xFFFF_FFFF;
            let v = ((!(a_u32 ^ b_u32) & (a_u32 ^ res)) & 0x8000_0000) != 0;
            (s, res, c, v)
        }
        IROp::Adc(s) => {
            let wide = a + b + c_in;
            let res = wide as u32;
            let c = wide > 0xFFFF_FFFF;
            let v = ((!(a_u32 ^ (b_u32 + c_in as u32)) & (a_u32 ^ res)) & 0x8000_0000) != 0;
            (s, res, c, v)
        }
        IROp::Sbc(s) => {
            let borrow = 1 - c_in as u32;
            let res = a_u32.wrapping_sub(b_u32).wrapping_sub(borrow);

            let full_rn = (a_u32 as u64).wrapping_add(borrow as u64);
            let c = (b_u32 as u64) >= full_rn;

            let v = ((b_u32 ^ a_u32) & (b_u32 ^ res) & 0x8000_0000) != 0;
            (s, res, c, v)
        }
        IROp::Sub(s) => {
            let res = a_u32.wrapping_sub(b_u32);
            let c = a_u32 >= b_u32; // NOT borrow
            let v = (((a_u32 ^ b_u32) & (a_u32 ^ res)) & 0x8000_0000) != 0;
            (s, res, c, v)
        }
        IROp::Rsb(s) => {
            let res = 1 - b_u32.wrapping_sub(a_u32);
            let c = b_u32 >= a_u32;
            let v = (((b_u32 ^ a_u32) & (b_u32 ^ res)) & 0x8000_0000) != 0;
            (s, res, c, v)
        }

        IROp::Rsc(s) => {
            let borrow = 1 - c_in as u32;
            let res = b_u32.wrapping_sub(a_u32).wrapping_sub(borrow);

            let full_rn = (a_u32 as u64).wrapping_add(borrow as u64);
            let c = (b_u32 as u64) >= full_rn;

            let v = ((b_u32 ^ a_u32) & (b_u32 ^ res) & 0x8000_0000) != 0;
            (s, res, c, v)
        }

        _ => return Err("not an arithmetic instruction".to_string()),
    };
    if s {
        let n = (result & 0x8000_0000) != 0;
        let z = result == 0;
        interpreter.cpu.cpsr.set_nzcv(n, z, carry, overflow);
    }

    Ok(result)
}

fn execute_logical(
    interpreter: &mut IRInterpreter,
    inst_inputs: &Vec<u32>,
    ir: IROp,
) -> Result<u32, String> {
    let a = interpreter.get_vreg(inst_inputs[0])?;
    let b = interpreter.get_vreg(inst_inputs[1])?;

    // Carry comes from shifter, NOT from logic
    let carry = interpreter.cpu.cpsr.c;

    let (s, result, write_result) = match ir {
        IROp::And(s) => (s, a & b, true),
        IROp::Eor(s) => (s, a ^ b, true),
        IROp::Orr(s) => (s, a | b, true),
        IROp::Bic(s) => (s, a & !b, true),
        IROp::Tst => (true, a & b, false),
        IROp::Teq => (true, a ^ b, false),

        _ => return Err("not a logical instruction".to_string()),
    };

    if s {
        let n = (result & 0x8000_0000) != 0;
        let z = result == 0;

        // V is unchanged
        interpreter
            .cpu
            .cpsr
            .set_nzcv(n, z, carry, interpreter.cpu.cpsr.v);
    }

    if write_result {
        Ok(result)
    } else {
        Ok(0) // TST / TEQ do not write back
    }
}

fn execute_compare(
    interpreter: &mut IRInterpreter,
    inst_inputs: &Vec<u32>,
    ir: IROp,
) -> Result<u32, String> {
    // Get the operand values
    let a = interpreter.get_vreg(inst_inputs[0])? as u64;
    let b = interpreter.get_vreg(inst_inputs[1])? as u64;

    // Carry and overflow calculation
    let (n, z, c, v) = match ir {
        IROp::Cmp => {
            // CMP: a - b
            let res = a.wrapping_sub(b);
            let result32 = res as u32;

            let n = (result32 & 0x8000_0000) != 0;
            let z = result32 == 0;
            let c = a >= b; // borrow not occurred
            let v = ((a as u32 ^ b as u32) & (a as u32 ^ result32) & 0x8000_0000) != 0;

            (n, z, c, v)
        }
        IROp::Cmn => {
            // CMN: a + b
            let res = a.wrapping_add(b);
            let result32 = res as u32;

            let n = (result32 & 0x8000_0000) != 0;
            let z = result32 == 0;
            let c = res > 0xFFFF_FFFF; // carry out
            let v = ((!(a as u32) ^ b as u32) & (a as u32 ^ result32) & 0x8000_0000) != 0;

            (n, z, c, v)
        }
        _ => return Err("not a compare instruction".to_string()),
    };

    interpreter.cpu.cpsr.set_nzcv(n, z, c, v);

    Ok(0) // CMP / CMN do not write to a register
}

fn execute_shift(
    interpreter: &mut IRInterpreter,
    inst_inputs: &Vec<u32>,
    ir: IROp,
) -> Result<u32, String> {
    let a = interpreter.get_vreg(inst_inputs[0])?;
    let b = inst_inputs[1];

    //  note that in ARM, only the bottom 8 bits of the shift register are used
    let shift_amount: u32 = b & 0xFF;

    let result = match ir {
        // Logical Shift Left
        IROp::Lsl => {
            if shift_amount >= 32 {
                0
            } else {
                a << shift_amount
            }
        }

        // Logical Shift Right
        IROp::Lsr => {
            if shift_amount >= 32 {
                0
            } else {
                a >> shift_amount
            }
        }

        // Arithmetic Shift Right (preserves the sign bit)
        IROp::Asr => {
            if shift_amount >= 32 {
                if (a as i32) < 0 { 0xFFFFFFFF } else { 0 }
            } else {
                ((a as i32) >> shift_amount) as u32
            }
        }

        // Rotate Right
        IROp::Ror => {
            let amount = shift_amount % 32;
            if amount == 0 {
                a
            } else {
                (a >> amount) | (a << (32 - amount))
            }
        }
        _ => return Err("not a shift instruction".to_string()),
    };

    Ok(result)
}
