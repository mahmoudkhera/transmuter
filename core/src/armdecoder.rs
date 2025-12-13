

/// Data processing instruction fields
#[derive(Debug)]
pub struct DataProcFields {
    pub opcode: u32,
    pub s: bool,
    pub rn: u32,
    pub rd: u32,
    pub shift_imm: u32,
    pub shift_type: u32,
    pub rm: u32,
}

impl DataProcFields {
    pub fn from_insn(insn: u32) -> Self {
        Self {
            opcode: (insn >> 21) & 0xF,
            s: (insn >> 20) & 1 == 1,
            rn: (insn >> 16) & 0xF,
            rd: (insn >> 12) & 0xF,
            shift_imm: (insn >> 7) & 0x1F,
            shift_type: (insn >> 5) & 0x3,
            rm: insn & 0xF,
        }
    }
}

/// Load/Store fields
#[derive(Debug)]
pub struct LoadStoreFields {
    pub p: bool, // Pre/post index
    pub u: bool, // Up/down
    pub b: bool, // Byte/word
    pub w: bool, // Writeback
    pub l: bool, // Load/store
    pub rn: u32,
    pub rd: u32,
    pub offset: u32,
}

impl LoadStoreFields {
    pub fn from_insn(insn: u32) -> Self {
        Self {
            p: (insn >> 24) & 1 == 1,
            u: (insn >> 23) & 1 == 1,
            b: (insn >> 22) & 1 == 1,
            w: (insn >> 21) & 1 == 1,
            l: (insn >> 20) & 1 == 1,
            rn: (insn >> 16) & 0xF,
            rd: (insn >> 12) & 0xF,
            offset: insn & 0xFFF,
        }
    }
}

/// Branch fields
#[derive(Debug)]
pub struct BranchFields {
    pub link: bool,  //BL (save return address in LR)
    pub offset: i32, //Signed jump distance (relative to PC)
}
impl BranchFields {
    pub fn from_insn(insn: u32) -> Self {
        let link = (insn >> 24) & 1 == 1;
        let offset = ((insn & 0x00FFFFFF) << 2) as i32;
        // Sign extend from 26 bits
        let offset = (offset << 6) >> 6;
        Self { link, offset }
    }
}
