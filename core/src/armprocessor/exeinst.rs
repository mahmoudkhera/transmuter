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
        IROp::Add => {
            let a = interpreter.get_vreg(inst.inputs[0])?;
            let b = interpreter.get_vreg(inst.inputs[1])?;
            let result = a.wrapping_add(b);
            Ok(result)
        }
        IROp::Mov => {
            let val = interpreter.get_vreg(inst.inputs[1])?;
            interpreter.cpu.write_reg(inst.inputs[1] as u8, val);
            println!(
                "reg  {}  value  after write {}",
                inst.inputs[1],
                interpreter.cpu.read_reg(inst.inputs[1] as u8)
            );
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
