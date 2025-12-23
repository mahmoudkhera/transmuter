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
            println!(
                "reg  {}  value  after write {}",
                reg,
                interpreter.cpu.read_reg(reg)
            );
            Ok(0)
        }
        IROp::And(s) => {
            let a = interpreter.get_vreg(inst.inputs[0])?;
            let b = interpreter.get_vreg(inst.inputs[1])?;
            let res = a & b;

            if s {
                interpreter.cpu.cpsr.z = res == 0;
            }
            Ok(res)
        }
        IROp::Orr(s) => {
            let a = interpreter.get_vreg(inst.inputs[0])?;
            let b = interpreter.get_vreg(inst.inputs[1])?;
            let res = a | b;

            if s {
                interpreter.cpu.cpsr.z = res == 0;
            }
            Ok(res)
        }
        IROp::Eor(s) => {
            let a = interpreter.get_vreg(inst.inputs[0])?;
            let b = interpreter.get_vreg(inst.inputs[1])?;
            let res = a ^ b;

            if s {
                interpreter.cpu.cpsr.z = res == 0;
            }
            Ok(res)
        }

        IROp::Add(s) => {
            let a = interpreter.get_vreg(inst.inputs[0])?;
            let b = interpreter.get_vreg(inst.inputs[1])?;
            let result = a.wrapping_add(b);

            if s {
                let c = (a as u64 + b as u64) > 0xFFFF_FFFF;
                let v = ((a ^ result) & (b ^ result)) & 0x8000_0000 != 0;
                let z = result == 0;
                let n = (result >> 31) & 1 == 1;
                interpreter.cpu.cpsr.set_nzcv(n, z, c, v);
            }

            println!(
                "  the add must update the carry flag {result}   c{}",
                interpreter.cpu.cpsr.n
            );
            Ok(result)
        }

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
            println!("evaluation result {}", result);
            Ok(result as u32)
        }
        IROp::BranchCond(_) => {
            let cond = interpreter.get_vreg(inst.inputs[0])?;
            Ok(cond)
        }
        IROp::Lsl(_, _) => {
            let reg = interpreter.get_vreg(inst.inputs[0])?;
            let shim = interpreter.get_vreg(inst.inputs[2])?;
            Ok(reg << (shim))
        }
        IROp::Branch(_) | IROp::Call(_) | IROp::Return | IROp::Nop => Ok(0),

        _ => Err(format!("not implemented or can not be excuted")),
    }
}
