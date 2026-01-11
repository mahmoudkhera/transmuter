use crate::{
    armprocessor::interpreter::IRInterpreter,
    translate::{IRInst, IROp},
};

pub fn execute_instruction(inst: &IRInst, interpreter: &mut IRInterpreter) -> Result<u64, String> {
    match inst.op {
        IROp::Const(val) => Ok(val as u64),
        IROp::LoadReg(reg) => {
            let val = interpreter.cpu.read_reg(reg);

            Ok(val as u64)
        }
        IROp::StoreReg(reg) => {
            let val = interpreter.get_vreg(inst.inputs[0])?;
            interpreter.cpu.write_reg(reg, val as u32);
            println!("reg{}  = 0x{:x}", reg, val);

            Ok(0)
        }
        IROp::Store2Regs(low, high) => {
            let val = interpreter.get_vreg(inst.inputs[0])?;
            let rdlo_value = val as u32; // Low 32 bits
            let rdhi_value = (val >> 32) as u32; // High 32 bits
            println!(
                "reg{}  = 0x{:x} reg{}  = 0x{:x}",
                low, rdlo_value, high, rdhi_value
            );

            interpreter.cpu.write_reg(low, rdlo_value);
            interpreter.cpu.write_reg(high, rdhi_value);

            Ok(0)
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
            Ok(result as u64)
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
        //# Multiply and multiply accumulate
        IROp::Mul(s) => execite_mul(interpreter, &inst.inputs, IROp::Mul(s)),
        IROp::Mls => execite_mul(interpreter, &inst.inputs, IROp::Mls),
        IROp::Mla(s) => execite_mul(interpreter, &inst.inputs, IROp::Mla(s)),
        IROp::Umull(s) => execite_mul(interpreter, &inst.inputs, IROp::Umull(s)),
        IROp::Umlal(s) => execite_mul(interpreter, &inst.inputs, IROp::Umlal(s)),
        IROp::Smull(s) => execite_mul(interpreter, &inst.inputs, IROp::Smull(s)),

        _ => Err(format!("not implemented or can not be excuted")),
    }
}

fn execute_arthimitic(
    interpreter: &mut IRInterpreter,
    inst_inputs: &Vec<u32>,
    ir: IROp,
) -> Result<u64, String> {
    let a = interpreter.get_vreg(inst_inputs[0])?;
    let b = interpreter.get_vreg(inst_inputs[1])?;

    let a_u32 = a as u32;
    let b_u32 = b as u32;
    let c_in = if interpreter.cpu.cpsr.c { 1u64 } else { 0u64 };

    let (s, result, carry, overflow) = match ir {
        // ADD: Rd = Rn + Operand2
        IROp::Add(s) => {
            let wide = a + b;
            let res: u32 = wide as u32;
            let c = wide > 0xFFFF_FFFF;
            let v = ((!(a_u32 ^ b_u32) & (a_u32 ^ res)) & 0x8000_0000) != 0;

            (s, res, c, v)
        }
        // ADC: Rd = Rn + Operand2 + Carry
        IROp::Adc(s) => {
            let wide: u64 = a + b + c_in;
            let res = wide as u32;
            let c = wide > 0xFFFF_FFFF;
            let b_eff = b_u32.wrapping_add(c_in as u32);
            let v = ((!(a_u32 ^ b_eff) & (a_u32 ^ res)) & 0x8000_0000) != 0;
            println!(" c {} v {} ", c, v);

            (s, res, c, v)
        }
        // SUB: Rd = Rn - Operand2
        IROp::Sub(s) => {
            let res = a_u32.wrapping_sub(b_u32);

            let c = a_u32 >= b_u32;

            let v: bool = (((a_u32 ^ a_u32) & (a_u32 ^ res)) & 0x8000_0000) != 0;

            (s, res, c, v)
        }
        // SBC: Rd = Rn - Operand2 - !Carry
        IROp::Sbc(s) => {
            let borrow = if c_in == 1 { 0u32 } else { 1u32 };

            let res = a_u32.wrapping_sub(b_u32).wrapping_sub(borrow);

            let wide_sub = (b_u32 as u64).wrapping_add(borrow as u64);
            let c = (a_u32 as u64) >= wide_sub;

            let v: bool = (((a_u32 ^ b_u32) & (a_u32 ^ res)) & 0x8000_0000) != 0;
            (s, res, c, v)
        }

        // RSB: Rd = Operand2 - Rn
        IROp::Rsb(s) => {
            let res = b_u32.wrapping_sub(a_u32);
            let c = b_u32 >= a_u32;

            let v = (((b_u32 ^ a_u32) & (b_u32 ^ res)) & 0x8000_0000) != 0;
            (s, res, c, v)
        }
        // RSC: Rd = Rm - Rn - !Carry
        IROp::Rsc(s) => {
            let borrow = if c_in == 1 { 0u32 } else { 1u32 };
            let res = b_u32.wrapping_sub(a_u32).wrapping_sub(borrow);

            let wide_res = (a_u32 as u64).wrapping_add(borrow as u64);
            let c = (b_u32 as u64) >= wide_res;

            let v = (((b_u32 ^ a_u32) & (b_u32 ^ res)) & 0x8000_0000) != 0;
            (s, res, c, v)
        }

        _ => return Err("not an arithmetic instruction".to_string()),
    };
    if s {
        let n = (result & 0x8000_0000) != 0;
        let z = result == 0;
        interpreter.cpu.cpsr.set_nzcv(n, z, carry, overflow);
    }

    Ok(result as u64)
}

fn execute_logical(
    interpreter: &mut IRInterpreter,
    inst_inputs: &Vec<u32>,
    ir: IROp,
) -> Result<u64, String> {
    let a = interpreter.get_vreg(inst_inputs[0])? as u32;
    let b = interpreter.get_vreg(inst_inputs[1])? as u32;

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
        Ok(result as u64)
    } else {
        Ok(0) // TST / TEQ do not write back
    }
}

fn execute_compare(
    interpreter: &mut IRInterpreter,
    inst_inputs: &Vec<u32>,
    ir: IROp,
) -> Result<u64, String> {
    // Get the operand values
    let a = interpreter.get_vreg(inst_inputs[0])? as u32;
    let b = interpreter.get_vreg(inst_inputs[1])? as u32;

    // Carry and overflow calculation
    let (n, z, c, v) = match ir {
        IROp::Cmp => {
            // CMP: a - b
            let res = a.wrapping_sub(b);
            let result32 = res as u32;

            println!("a {} b{} res {:x}", a, b, res);

            let n = (result32 & 0x8000_0000) != 0;
            let z = result32 == 0;
            let c = a >= b; // borrow not occurred
            let v = ((a as u32 ^ b as u32) & (a as u32 ^ result32) & 0x8000_0000) != 0;

            (n, z, c, v)
        }
        IROp::Cmn => {
            // CMN: a + b
            let res = (a).wrapping_add(b);
            let result32 = res as u32;
            println!("a {:x} b{} res {:x}", a, b, res);

            let n = (result32 & 0x8000_0000) != 0;
            let z = result32 == 0;
            let c = a as u64 + b as u64 > 0xFFFF_FFFF; // carry out
            let v = ((!(a ^ b) & (a ^ result32)) & 0x8000_0000) != 0;

            println!("n z {} {}", n, z);

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
) -> Result<u64, String> {
    let a = interpreter.get_vreg(inst_inputs[0])? as u32;
    let b = interpreter.get_vreg(inst_inputs[1])? as u32;

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

    Ok(result as u64)
}

fn execite_mul(
    interpreter: &mut IRInterpreter,
    inst_inputs: &Vec<u32>,
    ir: IROp,
) -> Result<u64, String> {
    let rn = interpreter.get_vreg(inst_inputs[0])?;
    let rm = interpreter.get_vreg(inst_inputs[1])?;

    match ir {
        IROp::Mul(s) => {
            let result = (rn as u32).wrapping_mul(rm as u32);
            if s {
                interpreter.cpu.cpsr.n = (result as i32) < 0;
                interpreter.cpu.cpsr.z = result == 0;
                // C is UNPREDICTABLE, V is unchanged
            }
            Ok(result as u64)
        }
        IROp::Mla(_) | IROp::Mls => {
            let result = (rn as u32).wrapping_mul(rm as u32);

            Ok(result as u64)
        }
        IROp::Umull(s) => {
            let result = (rn).wrapping_mul(rm);

            if s {
                // N = RdHi[31]
                interpreter.cpu.cpsr.n = result & 0x8000_0000 == 1;

                interpreter.cpu.cpsr.z = result == 0;
                // C, V are UNPREDICTABLE
            }

            Ok(result)
        }
        IROp::Umlal(s) => {
            let mul = (rn).wrapping_mul(rm);

            // Get current accumulator value from rd:ra
            let rdlo_current = interpreter.get_vreg(inst_inputs[2])?;
            let rdhi_current = interpreter.get_vreg(inst_inputs[3])?;
            let accumulator = ((rdhi_current as u64) << 32) | (rdlo_current as u64);

            // Add to accumulator

            let result = accumulator.wrapping_add(mul);

            if s {
                // N = RdHi[31]
                interpreter.cpu.cpsr.n = result & 0x8000_0000 == 1;

                interpreter.cpu.cpsr.z = result == 0;
                // C, V are UNPREDICTABLE
            }
            Ok(result)
        }
        IROp::Smull(s) => {
            let result = ((rn as u32) as i32 as i64) * ((rm as u32) as i32 as i64);

            if s {
                // N = RdHi[31]
                interpreter.cpu.cpsr.n = result < 0;

                interpreter.cpu.cpsr.z = result == 0;
                // C, V are UNPREDICTABLE
            }

            Ok(result as u64)
        }
        IROp::Smlal(s) => {
            // Get current accumulator value from rd:ra
            let rdlo_current = interpreter.get_vreg(inst_inputs[2])?;
            let rdhi_current = interpreter.get_vreg(inst_inputs[3])?;
            let accumulator = ((rdhi_current as u64) << 32) | (rdlo_current as u64);

            let accumulator = accumulator as i64;

            let mul = (rn as i64) * (rm as i64);

            let result = accumulator.wrapping_add(mul);

            // Update flags if S=1
            if s {
                interpreter.cpu.cpsr.n = result < 0;
                interpreter.cpu.cpsr.z = result == 0;
                // C and V are UNPREDICTABLE
            }

            Ok(result as u64)
        }
        IROp::Umaal => {
            // Get current accumulator value from rd:ra
            let rdlo_current = interpreter.get_vreg(inst_inputs[2])?;
            let rdhi_current = interpreter.get_vreg(inst_inputs[3])?;

            let mul = rn * rm;

            let result = mul.wrapping_add(rdlo_current).wrapping_add(rdhi_current);

            // UMAAL NEVER updates flags (no S bit)

            Ok(result)
        }

        _ => return Err("not a mul or saturation instruction".to_string()),
    }
}
