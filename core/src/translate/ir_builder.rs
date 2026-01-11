use crate::armprocessor::cpu::Condition;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum IROp {
    // Constants and Register Operations
    Const(u32),
    LoadReg(u8),
    StoreReg(u8),
    Store2Regs(u8, u8),

    // Arithmetic
    Add(bool),
    Adc(bool),
    Sub(bool),
    Sbc(bool),
    Rsb(bool),
    Rsc(bool),

    // Logical
    Mov(bool),
    And(bool),
    Orr(bool),
    Bic(bool),
    Eor(bool),
    Not(bool),
    Mvn(bool),
    Tst,
    Teq,

    // Shifts
    Lsr, //Logical Shift Right
    Asr, //Arithmetic Shift Right
    Lsl, //Logical Shift Left
    Ror, //Rotate Right

    // Comparison and Flags
    Cmp,
    Cmn,

    // Multiply and multiply accumulate
    Mul(bool),
    Mla(bool),
    Umaal,
    Mls,
    Umull(bool),
    Umlal(bool),
    Smull(bool),
    Smlal(bool),

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
impl IROp {
    pub fn name(&self) -> &'static str {
        match self {
            // Constants and Register Operations
            IROp::Const(_) => "Const",
            IROp::LoadReg(_) => "LoadReg",
            IROp::StoreReg(_) => "StoreReg",
            IROp::Store2Regs(_, _) => "Store2Regs",

            // Arithmetic
            IROp::Add(_) => "Add",
            IROp::Adc(_) => "Adc",
            IROp::Sub(_) => "Sub",
            IROp::Sbc(_) => "Sbc",
            IROp::Rsb(_) => "Rsb",
            IROp::Rsc(_) => "Rsc",
            IROp::Cmp => "Cmp",
            IROp::Cmn => "Cmn",

            // Logical
            IROp::Mov(_) => "Mov",
            IROp::And(_) => "And",
            IROp::Orr(_) => "Orr",
            IROp::Bic(_) => "Bic",
            IROp::Eor(_) => "Eor",
            IROp::Not(_) => "Not",
            IROp::Mvn(_) => "Mvn",
            IROp::Tst => "Tst",
            IROp::Teq => "Teq",

            // Shifts
            IROp::Lsr => "Lsr",
            IROp::Asr => "Asr",
            IROp::Lsl => "Lsl",
            IROp::Ror => "Ror",

            // Multiply and multiply accumulate
            IROp::Mul(_) => "MUL",
            IROp::Mla(_) => "MLA",
            IROp::Umaal => "UMAAL",
            IROp::Mls => "MLS",
            IROp::Umull(_) => "UMULL",
            IROp::Umlal(_) => "UMLAL",
            IROp::Smull(_) => "SMULL",
            IROp::Smlal(_) => "SMLAL",

            // Comparison and Flags
            IROp::SetFlag(_) => "SetFlag",
            IROp::GetFlag(_) => "GetFlag",
            IROp::EvalCondition(_) => "EvalCondition",

            // Memory
            IROp::Load => "Load",
            IROp::Store => "Store",

            // Control Flow
            IROp::Branch(_) => "Branch",
            IROp::BranchCond(_) => "BranchCond",
            IROp::Call(_) => "Call",
            IROp::Return => "Return",

            IROp::Nop => "Nop",
        }
    }
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
        self.next_block_id += 1;

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
            | IROp::Tst
            | IROp::Teq
            | IROp::Cmp
            | IROp::Cmn
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

    // sepcial emit functions

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

    pub fn emit_const(&mut self, value: u32) -> u32 {
        self.emit(IROp::Const(value), vec![]).unwrap()
    }

    pub fn emit_load_reg(&mut self, reg: u8) -> u32 {
        self.emit(IROp::LoadReg(reg), vec![]).unwrap()
    }

    pub fn emit_store_reg(&mut self, reg: u8, value: u32) {
        self.emit(IROp::StoreReg(reg), vec![value]);
    }
    pub fn emit_store_2regs(&mut self, reglow: u8, reghigh: u8, value: u32) {
        self.emit(IROp::Store2Regs(reglow, reghigh), vec![value]);
    }

    // # Data-processing
    pub fn emit_add(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Add(s), vec![a, b]).unwrap()
    }
    pub fn emit_adc(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Adc(s), vec![a, b]).unwrap()
    }

    pub fn emit_sub(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Sub(s), vec![a, b]).unwrap()
    }
    pub fn emit_sbc(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Sbc(s), vec![a, b]).unwrap()
    }
    pub fn emit_rsc(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Rsc(s), vec![a, b]).unwrap()
    }
    pub fn emit_rsb(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Rsb(s), vec![a, b]).unwrap()
    }

    pub fn emit_and(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::And(s), vec![a, b]).unwrap()
    }

    pub fn emit_eor(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Eor(s), vec![a, b]).unwrap()
    }
    pub fn emit_orr(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Orr(s), vec![a, b]).unwrap()
    }
    pub fn emit_bic(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Bic(s), vec![a, b]).unwrap()
    }
    pub fn emit_tst(&mut self, a: u32, b: u32) {
        self.emit(IROp::Tst, vec![a, b]);
    }
    pub fn emit_teq(&mut self, a: u32, b: u32) {
        self.emit(IROp::Teq, vec![a, b]);
    }
    pub fn emit_cmp(&mut self, a: u32, b: u32) {
        self.emit(IROp::Cmp, vec![a, b]);
    }
    pub fn emit_cmn(&mut self, a: u32, b: u32) {
        self.emit(IROp::Cmn, vec![a, b]);
    }

    pub fn emit_mov(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Mov(s), vec![a, b]).unwrap()
    }
    pub fn emit_mvn(&mut self, a: u32, b: u32, s: bool) -> u32 {
        self.emit(IROp::Mvn(s), vec![a, b]).unwrap()
    }

    pub fn emit_lsr(&mut self, a: u32, b: u32) -> u32 {
        self.emit(IROp::Lsr, vec![a, b]).unwrap()
    }
    pub fn emit_lsl(&mut self, a: u32, b: u32) -> u32 {
        self.emit(IROp::Lsl, vec![a, b]).unwrap()
    }
    pub fn emit_asr(&mut self, a: u32, b: u32) -> u32 {
        self.emit(IROp::Asr, vec![a, b]).unwrap()
    }
    pub fn emit_ror(&mut self, a: u32, b: u32) -> u32 {
        self.emit(IROp::Ror, vec![a, b]).unwrap()
    }

    //# Multiply and multiply accumulate

    pub fn emit_mul(&mut self, rn: u32, rm: u32, s: bool) -> u32 {
        self.emit(IROp::Mul(s), vec![rn, rm]).unwrap()
    }
    pub fn emit_mla(&mut self, rn: u32, rm: u32, s: bool) -> u32 {
        self.emit(IROp::Mla(s), vec![rn, rm]).unwrap()
    }
    pub fn emit_mls(&mut self, rn: u32, rm: u32) -> u32 {
        self.emit(IROp::Mls, vec![rn, rm]).unwrap()
    }
    pub fn emit_umull(&mut self, rn: u32, rm: u32, s: bool) -> u32 {
        self.emit(IROp::Umull(s), vec![rn, rm]).unwrap()
    }
    pub fn emit_smull(&mut self, rn: u32, rm: u32, s: bool) -> u32 {
        self.emit(IROp::Smull(s), vec![rn, rm]).unwrap()
    }
    pub fn emit_umlal(&mut self, rn: u32, rm: u32, rd: u32, ra: u32, s: bool) -> u32 {
        self.emit(IROp::Umlal(s), vec![rn, rm, rd, ra]).unwrap()
    }
    pub fn emit_smlal(&mut self, rn: u32, rm: u32, rd: u32, ra: u32, s: bool) -> u32 {
        self.emit(IROp::Smlal(s), vec![rn, rm, rd, ra]).unwrap()
    }
    pub fn emit_umaal(&mut self, rn: u32, rm: u32, rd: u32, ra: u32) -> u32 {
        self.emit(IROp::Umaal, vec![rn, rm, rd, ra]).unwrap()
    }
}
