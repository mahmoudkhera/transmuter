use std::collections::HashMap;

use crate::{arm32arch::arg_s_rrr_shi, armprocessor::Condition};

#[derive(Debug, Clone, Copy)]
pub enum IROp {
    // Constants and Register Operations
    Const(u32),
    LoadReg(u8),
    StoreReg(u8),

    // Arithmetic
    Add,
    Sub,
    Mul,
    Neg,

    // Logical
    And,
    Or,
    Xor,
    Not,

    // Shifts
    Lsr(u32, u32), //Logical Shift Right
    Asr(u32, u32), //Arithmetic Shift Right
    Lsl(u32, u32), //Logical Shift Left
    Ror(u32, u32), //Rotate Right

    // Comparison and Flags
    Cmp,
    SetFlag(Flag),
    GetFlag(Flag),
    EvalCondition(Condition),

    // Memory
    Load,
    Store,

    // Control Flow
    Branch(usize),
    BranchCond(usize),
    Call(usize),
    Return,

    Nop,
}
#[derive(Debug, Clone, Copy)]
pub enum Flag {
    N,
    Z,
    C,
    V,
}

// IR Instruction with metadata
#[derive(Debug, Clone)]
pub struct IRInst {
    pub op: IROp,
    pub inputs: Vec<u32>,
    pub output: Option<u32>,
}

// Basic Block
#[derive(Debug)]
pub struct BasicBlock {
    pub id: usize,
    pub instructions: Vec<IRInst>,
    pub successors: Vec<usize>,
}

// Complete IR Program
#[derive(Debug)]
pub struct IRProgram {
    pub blocks: HashMap<usize, BasicBlock>,
    pub entry: usize,
}

//IR Builder
pub struct IRBuilder {
    blocks: HashMap<usize, BasicBlock>,
    current_block: usize,
    next_block_id: usize,
    next_vreg: u32,
}

impl IRBuilder {
    pub fn new() -> Self {
        let start_block = BasicBlock {
            id: 0,

            instructions: Vec::new(),
            successors: Vec::new(),
        };
        let mut blocks = HashMap::new();
        blocks.insert(0, start_block);

        Self {
            blocks,
            current_block: 0,
            next_block_id: 1,
            next_vreg: 0,
        }
    }

    fn alloc_vreg(&mut self) -> u32 {
        let v = self.next_vreg;
        self.next_vreg += 1;
        v
    }

    pub fn create_blcok(&mut self) -> usize {
        let id = self.next_block_id;

        self.blocks.insert(
            id,
            BasicBlock {
                id,
                instructions: Vec::new(),
                successors: Vec::new(),
            },
        );

        id
    }

    pub fn switch_to_block(&mut self, block_id: usize) {
        self.current_block = block_id;
    }

    pub fn emit(&mut self, op: IROp, inputs: Vec<u32>) -> Option<u32> {
        let output = match &op {
            IROp::StoreReg(_)
            | IROp::Store
            | IROp::Branch(_)
            | IROp::BranchCond(_)
            | IROp::Call(_)
            | IROp::Return
            | IROp::Nop => None,
            _ => Some(self.alloc_vreg()),
        };

        let inst = IRInst { op, inputs, output };

        self.blocks
            .get_mut(&self.current_block)
            .unwrap()
            .instructions
            .push(inst);

        output
    }

    pub fn emit_const(&mut self, value: u32) -> u32 {
        self.emit(IROp::Const(value), vec![]).unwrap()
    }

    pub fn emit_load_reg(&mut self, reg: u8) -> u32 {
        self.emit(IROp::LoadReg(reg), vec![]).unwrap()
    }

    pub fn emit_store_reg(&mut self, reg: u8, value: u32) {
        self.emit(IROp::StoreReg(reg), vec![value]);
    }

    pub fn emit_add(&mut self, a: u32, b: u32) -> u32 {
        self.emit(IROp::Add, vec![a, b]).unwrap()
    }

    pub fn emit_sub(&mut self, a: u32, b: u32) -> u32 {
        self.emit(IROp::Sub, vec![a, b]).unwrap()
    }

    pub fn emit_and(&mut self, a: u32, b: u32) -> u32 {
        self.emit(IROp::And, vec![a, b]).unwrap()
    }

    pub fn emit_lsr(&mut self, reg:u32,shty: u32, shim: u32) -> u32 {
        self.emit(IROp::Lsr(shty, shim), vec![reg,shty, shim]).unwrap()
    }
    pub fn emit_lsl(&mut self, reg:u32,shty: u32, shim: u32) -> u32 {
        self.emit(IROp::Lsl(shty, shim), vec![reg,shty, shim]).unwrap()
    }
    pub fn emit_asr(&mut self,reg:u32, shty: u32, shim: u32) -> u32 {
        self.emit(IROp::Asr(shty, shim), vec![reg,shty, shim]).unwrap()
    }
    pub fn emit_ror(&mut self, reg:u32,shty: u32, shim: u32) -> u32 {
        self.emit(IROp::Ror(shty, shim), vec![reg,shty, shim]).unwrap()
    }

    pub fn emit_branch(&mut self, target: usize) {
        self.emit(IROp::Branch(target), vec![]);
        self.blocks
            .get_mut(&self.current_block)
            .unwrap()
            .successors
            .push(target);
    }

    pub fn emit_branch_cond(&mut self, cond: u32, target: usize, fallthrough: usize) {
        self.emit(IROp::BranchCond(target), vec![cond]);
        let block = self.blocks.get_mut(&self.current_block).unwrap();
        block.successors.push(target);
        block.successors.push(fallthrough);
    }

    pub fn emit_eval_condition(&mut self, cond: Condition) -> u32 {
        self.emit(IROp::EvalCondition(cond), vec![]).unwrap()
    }

    pub fn finalize(self) -> IRProgram {
        IRProgram {
            blocks: self.blocks,
            entry: 0,
        }
    }
}
