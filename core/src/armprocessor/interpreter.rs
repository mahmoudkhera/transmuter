use std::collections::HashMap;

use crate::{
    armprocessor::{cpu::CPUState, exeinst},
    translate::{IROp, IRProgram},
};

pub struct IRInterpreter {
    pub cpu: CPUState,
    pub vregs: HashMap<u32, u32>, // Virtual register values
}

impl IRInterpreter {
    pub fn new() -> Self {
        Self {
            cpu: CPUState::new(),
            vregs: HashMap::new(),
        }
    }

    pub fn excute(&mut self, program: &IRProgram) -> Result<(), String> {
        let mut current_block = program.entry;

        println!(" Starting IR ");

        loop {
            let block = program
                .blocks
                .get(&current_block)
                .ok_or_else(|| format!("Block {} not found", current_block))?;

            let mut next_block = None;

            println!("Executing Block {}", block.id);

            // Execute each instruction in the block
            for (_, inst) in block.instructions.iter().enumerate() {
                println!("instruction {:?}", inst);
                let res = exeinst::execute_instruction(inst, self)?;

                // Store result if instruction produces one
                if let Some(output) = inst.output {
                    self.vregs.insert(output, res);
                    println!("    v{} = {}", output, res);
                }

                // Handle control flow
                match &inst.op {
                    IROp::Branch(target) => {
                        next_block = Some(*target);
                        break;
                    }
                    IROp::BranchCond(target) => {
                        if res != 0 {
                            next_block = Some(*target);
                            println!("    Branch taken to block {}", target);
                        } else {
                            next_block = Some(block.successors[0]);
                            println!(
                                "    Branch not taken, fallthrough to block {}",
                                block.successors[0]
                            );
                        }
                        break;
                    }
                    IROp::Return => {
                        println!("=== Execution Complete ===\n");
                        return Ok(());
                    }
                    _ => {}
                }
            }

            // Move to next block
            if let Some(next) = next_block {
                current_block = next;
            } else {
                // No explicit branch, check if we have a fallthrough
                if block.successors.is_empty() {
                    println!("=== Execution Complete (no successors) ===\n");
                    return Ok(());
                }
                current_block = block.successors[0];
            }

            println!();
        }
    }

    pub fn get_vreg(&self, vreg: u32) -> Result<u32, String> {
        self.vregs
            .get(&vreg)
            .copied()
            .ok_or_else(|| format!("Virtual register v{} not found", vreg))
    }
}
