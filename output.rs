// Auto-generated from a32.decode
// Do not edit manually

#![allow(non_camel_case_types)]
#![allow(clippy::all)]

// ===== Extraction Helper Functions =====

fn extract_simple(inst: u32, pos: u32, len: u32) -> u32 {
    (inst >> pos) & ((1u32 << len) - 1)
}

fn extract_signed(inst: u32, pos: u32, len: u32) -> i32 {
    let val = (inst >> pos) & ((1u32 << len) - 1);
    // Sign extend
    if (val & (1u32 << (len - 1))) != 0 {
        (val | (!((1u32 << len) - 1))) as i32
    } else {
        val as i32
    }
}

fn extract_mul2(inst: u32, pos1: u32, len1: u32, pos2: u32, len2: u32) -> u32 {
    let mask1 = (1u32 << len1) - 1;
    let mask2 = (1u32 << len2) - 1;
    let field1 = (inst >> pos1) & mask1;
    let field2 = (inst >> pos2) & mask2;
    // concatenate field1 (lower bits) and field2 (upper bits)
    field1 | (field2 << len1)
}

fn extract_mul3(
    inst: u32,
    pos1: u32,
    len1: u32,
    pos2: u32,
    len2: u32,
    pos3: u32,
    len3: u32,
) -> u32 {
    let mask1 = (1u32 << len1) - 1;
    let mask2 = (1u32 << len2) - 1;
    let mask3 = (1u32 << len3) - 1;
    let field1 = (inst >> pos1) & mask1;
    let field2 = (inst >> pos2) & mask2;
    let field3 = (inst >> pos3) & mask3;
    field1 | (field2 << len1) | (field3 << (len1 + len2))
}

fn extract_mul4(
    inst: u32,
    pos1: u32,
    len1: u32,
    pos2: u32,
    len2: u32,
    pos3: u32,
    len3: u32,
    pos4: u32,
    len4: u32,
) -> u32 {
    let mask1 = (1u32 << len1) - 1;
    let mask2 = (1u32 << len2) - 1;
    let mask3 = (1u32 << len3) - 1;
    let mask4 = (1u32 << len4) - 1;
    let field1 = (inst >> pos1) & mask1;
    let field2 = (inst >> pos2) & mask2;
    let field3 = (inst >> pos3) & mask3;
    let field4 = (inst >> pos4) & mask4;
    field1 | (field2 << len1) | (field3 << (len1 + len2)) | (field4 << (len1 + len2 + len3))
}

fn times_2(val: u32) -> u32 {
    val << 1
}

fn times_4(val: u32) -> u32 {
    val << 2
}

fn times_8(val: u32) -> u32 {
    val << 3
}

fn expand_imm(val: u32) -> u32 {
    // ARM immediate expansion logic
    let rotate = (val >> 8) & 0xF;
    let imm = val & 0xFF;
    imm.rotate_right(rotate * 2)
}

fn negate(val: u32) -> u32 {
    (!val).wrapping_add(1)
}

// ===== Argument Structures =====

#[derive(Debug, Clone, PartialEq)]
pub struct arg_rrrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_ldrex {
    pub rn: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_USADA8 {
    pub rd: u32,
    pub rm: u32,
    pub rn: u32,
    pub ra: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_s_rrr_shi {
    pub s: u32,
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub shim: u32,
    pub shty: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_rrr_rot {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub rot: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_s_rrrr {
    pub s: u32,
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_msr_reg {
    pub rn: u32,
    pub r: u32,
    pub mask: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_bfi {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub msb: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_ldst_block {
    pub rn: u32,
    pub i: u32,
    pub b: u32,
    pub u: u32,
    pub w: u32,
    pub list: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_msr_i {
    pub r: u32,
    pub mask: u32,
    pub rot: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_mcrr {
    pub cp: u32,
    pub opc1: u32,
    pub crm: u32,
    pub rt: u32,
    pub rt2: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_i {
    pub imm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_mrs_reg {
    pub rd: u32,
    pub r: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_mrs_bank {
    pub rd: u32,
    pub r: u32,
    pub sysm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_bfx {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub widthm1: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_pkh {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub imm: u32,
    pub tb: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_mcr {
    pub cp: u32,
    pub opc1: u32,
    pub crn: u32,
    pub crm: u32,
    pub opc2: u32,
    pub rt: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_s_rrr_shr {
    pub s: u32,
    pub rn: u32,
    pub rd: u32,
    pub rm: u32,
    pub rs: u32,
    pub shty: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_r {
    pub rm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_msr_bank {
    pub rn: u32,
    pub r: u32,
    pub sysm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_ldst_ri {
    pub p: u32,
    pub w: u32,
    pub u: u32,
    pub rn: u32,
    pub rt: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_sat {
    pub rd: u32,
    pub rn: u32,
    pub satimm: u32,
    pub imm: u32,
    pub sh: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_s_rri_rot {
    pub s: u32,
    pub rn: u32,
    pub rd: u32,
    pub imm: u32,
    pub rot: u32,
}

// Warning: Empty argument set: empty
#[derive(Debug, Clone, PartialEq)]
pub struct arg_ri {
    pub rd: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_swp {
    pub rt2: u32,
    pub rt: u32,
    pub rn: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_rr {
    pub rd: u32,
    pub rm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_ldst_rr {
    pub p: u32,
    pub w: u32,
    pub u: u32,
    pub rn: u32,
    pub rt: u32,
    pub rm: u32,
    pub shimm: u32,
    pub shtype: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_rrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_strex {
    pub rn: u32,
    pub rd: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
}

// ===== Format Extraction Functions =====

pub fn extract_S_xri_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        rn: extract_simple(inst, 16, 4),
        s: 1,
        imm: extract_simple(inst, 0, 8),
        rot: times_2(extract_simple(inst, 8, 4)),
        rd: 0,
    }
}

pub fn extract_ldst_ri8_p1w(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        rt: extract_simple(inst, 12, 4),
        imm: extract_mul2(inst, 8, 4, 0, 4),
        p: 1,
        rn: extract_simple(inst, 16, 4),
        u: extract_simple(inst, 23, 1),
        w: extract_simple(inst, 21, 1),
    }
}

pub fn extract_s_rd0mn(inst: u32) -> arg_s_rrrr {
    arg_s_rrrr {
        s: extract_simple(inst, 20, 1),
        rm: extract_simple(inst, 8, 4),
        rd: extract_simple(inst, 16, 4),
        rn: extract_simple(inst, 0, 4),
        ra: 0,
    }
}

pub fn extract_mcr(inst: u32) -> arg_mcr {
    arg_mcr {
        opc2: extract_simple(inst, 5, 3),
        crm: extract_simple(inst, 0, 4),
        rt: extract_simple(inst, 12, 4),
        opc1: extract_simple(inst, 21, 3),
        crn: extract_simple(inst, 16, 4),
        cp: extract_simple(inst, 8, 4),
    }
}

pub fn extract_stl(inst: u32) -> arg_ldrex {
    arg_ldrex {
        rn: extract_simple(inst, 16, 4),
        rt: extract_simple(inst, 0, 4),
        rt2: 15,
        imm: 0,
    }
}

pub fn extract_s_rdamn(inst: u32) -> arg_s_rrrr {
    arg_s_rrrr {
        rm: extract_simple(inst, 8, 4),
        ra: extract_simple(inst, 12, 4),
        s: extract_simple(inst, 20, 1),
        rd: extract_simple(inst, 16, 4),
        rn: extract_simple(inst, 0, 4),
    }
}

pub fn extract_branch(inst: u32) -> arg_i {
    arg_i {
        imm: times_4(extract_signed(inst, 0, 24) as u32),
    }
}

pub fn extract_ldst_rs_p0w1(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        u: extract_simple(inst, 23, 1),
        p: 0,
        shtype: extract_simple(inst, 5, 2),
        rt: extract_simple(inst, 12, 4),
        rm: extract_simple(inst, 0, 4),
        rn: extract_simple(inst, 16, 4),
        shimm: extract_simple(inst, 7, 5),
        w: 0,
    }
}

pub fn extract_ldst_ri12_pw0(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        rt: extract_simple(inst, 12, 4),
        imm: extract_simple(inst, 0, 12),
        p: 0,
        u: extract_simple(inst, 23, 1),
        rn: extract_simple(inst, 16, 4),
        w: 0,
    }
}

pub fn extract_sat16(inst: u32) -> arg_sat {
    arg_sat {
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 0, 4),
        imm: 0,
        sh: 0,
        satimm: extract_simple(inst, 16, 4),
    }
}

pub fn extract_rdmn(inst: u32) -> arg_rrr {
    arg_rrr {
        rn: extract_simple(inst, 0, 4),
        rd: extract_simple(inst, 16, 4),
        rm: extract_simple(inst, 8, 4),
    }
}

pub fn extract_ldst_ri8_pw0(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        imm: extract_mul2(inst, 8, 4, 0, 4),
        p: 0,
        rn: extract_simple(inst, 16, 4),
        w: 0,
        rt: extract_simple(inst, 12, 4),
        u: extract_simple(inst, 23, 1),
    }
}

pub fn extract_rdm(inst: u32) -> arg_rr {
    arg_rr {
        rd: extract_simple(inst, 12, 4),
        rm: extract_simple(inst, 0, 4),
    }
}

pub fn extract_i16(inst: u32) -> arg_i {
    arg_i {
        imm: extract_mul2(inst, 8, 12, 0, 4),
    }
}

pub fn extract_rd0mn(inst: u32) -> arg_rrrr {
    arg_rrrr {
        rd: extract_simple(inst, 16, 4),
        rn: extract_simple(inst, 0, 4),
        ra: 0,
        rm: extract_simple(inst, 8, 4),
    }
}

pub fn extract_ldst_rr_p0w1(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        rn: extract_simple(inst, 16, 4),
        p: 0,
        u: extract_simple(inst, 23, 1),
        rt: extract_simple(inst, 12, 4),
        w: 0,
        shimm: 0,
        rm: extract_simple(inst, 0, 4),
        shtype: 0,
    }
}

pub fn extract_s_rrr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        rs: extract_simple(inst, 8, 4),
        rn: extract_simple(inst, 16, 4),
        s: extract_simple(inst, 20, 1),
        shty: extract_simple(inst, 5, 2),
        rm: extract_simple(inst, 0, 4),
        rd: extract_simple(inst, 12, 4),
    }
}

pub fn extract_ldst_ri12_p1w(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        rn: extract_simple(inst, 16, 4),
        u: extract_simple(inst, 23, 1),
        p: 1,
        rt: extract_simple(inst, 12, 4),
        imm: extract_simple(inst, 0, 12),
        w: extract_simple(inst, 21, 1),
    }
}

pub fn extract_strex(inst: u32) -> arg_strex {
    arg_strex {
        rt2: 15,
        rt: extract_simple(inst, 0, 4),
        imm: 0,
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
    }
}

pub fn extract_ldrex(inst: u32) -> arg_ldrex {
    arg_ldrex {
        rn: extract_simple(inst, 16, 4),
        rt: extract_simple(inst, 12, 4),
        imm: 0,
        rt2: 15,
    }
}

pub fn extract_rdamn(inst: u32) -> arg_rrrr {
    arg_rrrr {
        rm: extract_simple(inst, 8, 4),
        rn: extract_simple(inst, 0, 4),
        ra: extract_simple(inst, 12, 4),
        rd: extract_simple(inst, 16, 4),
    }
}

pub fn extract_msr_i(inst: u32) -> arg_msr_i {
    arg_msr_i {
        rot: extract_simple(inst, 8, 4),
        imm: extract_simple(inst, 0, 8),
        mask: extract_simple(inst, 16, 4),
    }
}

pub fn extract_ldst_ri12_p0w1(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        w: 0,
        imm: extract_simple(inst, 0, 12),
        rn: extract_simple(inst, 16, 4),
        rt: extract_simple(inst, 12, 4),
        p: 0,
        u: extract_simple(inst, 23, 1),
    }
}

pub fn extract_swp(inst: u32) -> arg_swp {
    arg_swp {
        rt2: extract_simple(inst, 0, 4),
        rt: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
    }
}

pub fn extract_S_xrr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        rn: extract_simple(inst, 16, 4),
        rs: extract_simple(inst, 8, 4),
        rm: extract_simple(inst, 0, 4),
        s: 1,
        shty: extract_simple(inst, 5, 2),
        rd: 0,
    }
}

pub fn extract_rndm(inst: u32) -> arg_rrr {
    arg_rrr {
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
        rm: extract_simple(inst, 0, 4),
    }
}

pub fn extract_S_xrr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        rd: 0,
        rn: extract_simple(inst, 16, 4),
        s: 1,
        shty: extract_simple(inst, 5, 2),
        shim: extract_simple(inst, 7, 5),
        rm: extract_simple(inst, 0, 4),
    }
}

pub fn extract_s_rri_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        rot: times_2(extract_simple(inst, 8, 4)),
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
        s: extract_simple(inst, 20, 1),
        imm: extract_simple(inst, 0, 8),
    }
}

pub fn extract_ldst_rr_p1w(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        shtype: 0,
        u: extract_simple(inst, 23, 1),
        rn: extract_simple(inst, 16, 4),
        rt: extract_simple(inst, 12, 4),
        w: extract_simple(inst, 21, 1),
        p: 1,
        shimm: 0,
        rm: extract_simple(inst, 0, 4),
    }
}

pub fn extract_bfx(inst: u32) -> arg_bfx {
    arg_bfx {
        rn: extract_simple(inst, 0, 4),
        lsb: extract_simple(inst, 7, 5),
        rd: extract_simple(inst, 12, 4),
        widthm1: extract_simple(inst, 16, 5),
    }
}

pub fn extract_ldst_rs_pw0(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        shimm: extract_simple(inst, 7, 5),
        rn: extract_simple(inst, 16, 4),
        w: 0,
        shtype: extract_simple(inst, 5, 2),
        p: 0,
        rm: extract_simple(inst, 0, 4),
        rt: extract_simple(inst, 12, 4),
        u: extract_simple(inst, 23, 1),
    }
}

pub fn extract_rrr_rot(inst: u32) -> arg_rrr_rot {
    arg_rrr_rot {
        rd: extract_simple(inst, 12, 4),
        rm: extract_simple(inst, 0, 4),
        rn: extract_simple(inst, 16, 4),
        rot: extract_simple(inst, 10, 2),
    }
}

pub fn extract_s_rxr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        rs: extract_simple(inst, 8, 4),
        rm: extract_simple(inst, 0, 4),
        s: extract_simple(inst, 20, 1),
        rn: 0,
        shty: extract_simple(inst, 5, 2),
        rd: extract_simple(inst, 12, 4),
    }
}

pub fn extract_ldst_rr_pw0(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        rm: extract_simple(inst, 0, 4),
        rn: extract_simple(inst, 16, 4),
        shimm: 0,
        rt: extract_simple(inst, 12, 4),
        p: 0,
        w: 0,
        u: extract_simple(inst, 23, 1),
        shtype: 0,
    }
}

pub fn extract_s_rrr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        rn: extract_simple(inst, 16, 4),
        shim: extract_simple(inst, 7, 5),
        rd: extract_simple(inst, 12, 4),
        s: extract_simple(inst, 20, 1),
        shty: extract_simple(inst, 5, 2),
        rm: extract_simple(inst, 0, 4),
    }
}

pub fn extract_sat(inst: u32) -> arg_sat {
    arg_sat {
        rd: extract_simple(inst, 12, 4),
        satimm: extract_simple(inst, 16, 5),
        imm: extract_simple(inst, 7, 5),
        rn: extract_simple(inst, 0, 4),
        sh: extract_simple(inst, 6, 1),
    }
}

pub fn extract_ldst_ri8_p0w1(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        rt: extract_simple(inst, 12, 4),
        p: 0,
        rn: extract_simple(inst, 16, 4),
        u: extract_simple(inst, 23, 1),
        w: 0,
        imm: extract_mul2(inst, 8, 4, 0, 4),
    }
}

pub fn extract_mov16(inst: u32) -> arg_ri {
    arg_ri {
        rd: extract_simple(inst, 12, 4),
        imm: extract_mul2(inst, 16, 4, 0, 12),
    }
}

pub fn extract_s_rxi_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        rd: extract_simple(inst, 12, 4),
        rn: 0,
        imm: extract_simple(inst, 0, 8),
        s: extract_simple(inst, 20, 1),
        rot: times_2(extract_simple(inst, 8, 4)),
    }
}

pub fn extract_ldst_rs_p1w(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        rn: extract_simple(inst, 16, 4),
        u: extract_simple(inst, 23, 1),
        shtype: extract_simple(inst, 5, 2),
        w: extract_simple(inst, 21, 1),
        p: 1,
        rm: extract_simple(inst, 0, 4),
        shimm: extract_simple(inst, 7, 5),
        rt: extract_simple(inst, 12, 4),
    }
}

pub fn extract_mcrr(inst: u32) -> arg_mcrr {
    arg_mcrr {
        cp: extract_simple(inst, 8, 4),
        opc1: extract_simple(inst, 4, 4),
        crm: extract_simple(inst, 0, 4),
        rt: extract_simple(inst, 12, 4),
        rt2: extract_simple(inst, 16, 4),
    }
}

pub fn extract_s_rxr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        shim: extract_simple(inst, 7, 5),
        rm: extract_simple(inst, 0, 4),
        s: extract_simple(inst, 20, 1),
        shty: extract_simple(inst, 5, 2),
        rn: 0,
        rd: extract_simple(inst, 12, 4),
    }
}

pub fn extract_rm(inst: u32) -> arg_r {
    arg_r {
        rm: extract_simple(inst, 0, 4),
    }
}

// ===== Pattern Group Structure =====

// Overlap Group {
//   AND_rrri
//   EOR_rrri
//   SUB_rrri
//   RSB_rrri
//   ADD_rrri
//   ADC_rrri
//   SBC_rrri
//   RSC_rrri
//   TST_xrri
//   TEQ_xrri
//   CMP_xrri
//   CMN_xrri
//   ORR_rrri
//   MOV_rxri
//   BIC_rrri
//   MVN_rxri
//   MOVW
//   MOVT
//   AND_rrrr
//   EOR_rrrr
//   SUB_rrrr
//   RSB_rrrr
//   ADD_rrrr
//   ADC_rrrr
//   SBC_rrrr
//   RSC_rrrr
//   TST_xrrr
//   TEQ_xrrr
//   CMP_xrrr
//   CMN_xrrr
//   ORR_rrrr
//   MOV_rxrr
//   BIC_rrrr
//   MVN_rxrr
//   AND_rri
//   EOR_rri
//   SUB_rri
//   RSB_rri
//   ADD_rri
//   ADC_rri
//   SBC_rri
//   RSC_rri
//   TST_xri
//   TEQ_xri
//   CMP_xri
//   CMN_xri
//   ORR_rri
//   MOV_rxi
//   BIC_rri
//   MVN_rxi
//   MUL
//   MLA
//   UMAAL
//   MLS
//   UMULL
//   UMLAL
//   SMULL
//   SMLAL
//   QADD
//   QSUB
//   QDADD
//   QDSUB
//   SMLABB
//   SMLABT
//   SMLATB
//   SMLATT
//   SMLAWB
//   SMULWB
//   SMLAWT
//   SMULWT
//   SMLALBB
//   SMLALBT
//   SMLALTB
//   SMLALTT
//   SMULBB
//   SMULBT
//   SMULTB
//   SMULTT
//   MSR_imm
//   CRC32B
//   CRC32H
//   CRC32W
//   CRC32CB
//   CRC32CH
//   CRC32CW
//   MRS_bank
//   MSR_bank
//   MRS_reg
//   MSR_reg
//   BX
//   BXJ
//   BLX_r
//   CLZ
//   ERET
//   HLT
//   BKPT
//   HVC
//   SMC
//   STRH_rr
//   STRH_rr
//   LDRD_rr
//   LDRD_rr
//   STRD_rr
//   STRD_rr
//   LDRH_rr
//   LDRH_rr
//   LDRSB_rr
//   LDRSB_rr
//   LDRSH_rr
//   LDRSH_rr
//   STRHT_rr
//   LDRHT_rr
//   LDRSBT_rr
//   LDRSHT_rr
//   STR_rr
//   STR_rr
//   STRB_rr
//   STRB_rr
//   LDR_rr
//   LDR_rr
//   LDRB_rr
//   LDRB_rr
//   STRT_rr
//   STRBT_rr
//   LDRT_rr
//   LDRBT_rr
//   STRH_ri
//   STRH_ri
//   LDRD_ri_a32
//   LDRD_ri_a32
//   STRD_ri_a32
//   STRD_ri_a32
//   LDRH_ri
//   LDRH_ri
//   LDRSB_ri
//   LDRSB_ri
//   LDRSH_ri
//   LDRSH_ri
//   STRHT_ri
//   LDRHT_ri
//   LDRSBT_ri
//   LDRSHT_ri
//   STR_ri
//   STR_ri
//   STRB_ri
//   STRB_ri
//   LDR_ri
//   LDR_ri
//   LDRB_ri
//   LDRB_ri
//   STRT_ri
//   STRBT_ri
//   LDRT_ri
//   LDRBT_ri
//   SWP
//   SWPB
//   STREX
//   STREXD_a32
//   STREXB
//   STREXH
//   STLEX
//   STLEXD_a32
//   STLEXB
//   STLEXH
//   STL
//   STLB
//   STLH
//   LDREX
//   LDREXD_a32
//   LDREXB
//   LDREXH
//   LDAEX
//   LDAEXD_a32
//   LDAEXB
//   LDAEXH
//   LDA
//   LDAB
//   LDAH
//   USADA8
//   SBFX
//   UBFX
//   BFCI
//   UDF
//   SADD16
//   SASX
//   SSAX
//   SSUB16
//   SADD8
//   SSUB8
//   QADD16
//   QASX
//   QSAX
//   QSUB16
//   QADD8
//   QSUB8
//   SHADD16
//   SHASX
//   SHSAX
//   SHSUB16
//   SHADD8
//   SHSUB8
//   UADD16
//   UASX
//   USAX
//   USUB16
//   UADD8
//   USUB8
//   UQADD16
//   UQASX
//   UQSAX
//   UQSUB16
//   UQADD8
//   UQSUB8
//   UHADD16
//   UHASX
//   UHSAX
//   UHSUB16
//   UHADD8
//   UHSUB8
//   PKH
//   SSAT
//   USAT
//   SSAT16
//   USAT16
//   SXTAB16
//   SXTAB
//   SXTAH
//   UXTAB16
//   UXTAB
//   UXTAH
//   SEL
//   REV
//   REV16
//   REVSH
//   RBIT
//   SMLAD
//   SMLADX
//   SMLSD
//   SMLSDX
//   SDIV
//   UDIV
//   SMLALD
//   SMLALDX
//   SMLSLD
//   SMLSLDX
//   SMMLA
//   SMMLAR
//   SMMLS
//   SMMLSR
//   STM
//   LDM_a32
//   B
//   BL
//   MCRR
//   MRRC
//   MCR
//   MRC
//   SVC
//   Overlap Group {
//     MSR_imm
//     Overlap Group {
//       NOP
//       No-Overlap Group {
//         YIELD
//         WFE
//         WFI
//         ESB
//       }
//     }
//   }
// }

// ===== Decoder Function (skeleton) =====

#[derive(Debug, Clone)]
pub enum Instruction {
    // Add instruction variants here
}

pub fn decode_instruction(inst: u32) -> Option<Instruction> {
    // Overlap group - check in order
    // Overlap group - check in order
    // Overlap group - check in order
    // No-overlap group - mutually exclusive patterns
    match inst & 0x0f000000 {
        0x0f000000 => {
            // YIELD
            let args = extract_YIELD(inst);
            return Some(Instruction::YIELD { args });
        }
        0x0f000000 => {
            // WFE
            let args = extract_WFE(inst);
            return Some(Instruction::WFE { args });
        }
        0x0f000000 => {
            // WFI
            let args = extract_WFI(inst);
            return Some(Instruction::WFI { args });
        }
        0x0f000000 => {
            // ESB
            let args = extract_ESB(inst);
            return Some(Instruction::ESB { args });
        }
        _ => {}
    }
    if (inst & 0x0f000000) == 0x0f000000 {
        // Matched: NOP
        let args = extract_NOP(inst);
        return Some(Instruction::NOP { args });
    }
    if (inst & 0x00f00000) == 0x00f00000 {
        // Matched: MSR_imm
        let args = extract_MSR_imm(inst);
        return Some(Instruction::MSR_imm { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: AND_rrri
        let args = extract_AND_rrri(inst);
        return Some(Instruction::AND_rrri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: EOR_rrri
        let args = extract_EOR_rrri(inst);
        return Some(Instruction::EOR_rrri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SUB_rrri
        let args = extract_SUB_rrri(inst);
        return Some(Instruction::SUB_rrri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: RSB_rrri
        let args = extract_RSB_rrri(inst);
        return Some(Instruction::RSB_rrri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: ADD_rrri
        let args = extract_ADD_rrri(inst);
        return Some(Instruction::ADD_rrri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: ADC_rrri
        let args = extract_ADC_rrri(inst);
        return Some(Instruction::ADC_rrri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SBC_rrri
        let args = extract_SBC_rrri(inst);
        return Some(Instruction::SBC_rrri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: RSC_rrri
        let args = extract_RSC_rrri(inst);
        return Some(Instruction::RSC_rrri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: TST_xrri
        let args = extract_TST_xrri(inst);
        return Some(Instruction::TST_xrri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: TEQ_xrri
        let args = extract_TEQ_xrri(inst);
        return Some(Instruction::TEQ_xrri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: CMP_xrri
        let args = extract_CMP_xrri(inst);
        return Some(Instruction::CMP_xrri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: CMN_xrri
        let args = extract_CMN_xrri(inst);
        return Some(Instruction::CMN_xrri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: ORR_rrri
        let args = extract_ORR_rrri(inst);
        return Some(Instruction::ORR_rrri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: MOV_rxri
        let args = extract_MOV_rxri(inst);
        return Some(Instruction::MOV_rxri { args });
    }
    if (inst & 0x0e000000) == 0x0e000000 {
        // Matched: BIC_rrri
        let args = extract_BIC_rrri(inst);
        return Some(Instruction::BIC_rrri { args });
    }
    if (inst & 0x0f000000) == 0x0f000000 {
        // Matched: MVN_rxri
        let args = extract_MVN_rxri(inst);
        return Some(Instruction::MVN_rxri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: MOVW
        let args = extract_MOVW(inst);
        return Some(Instruction::MOVW { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: MOVT
        let args = extract_MOVT(inst);
        return Some(Instruction::MOVT { args });
    }
    if (inst & 0x00001000) == 0x00001000 {
        // Matched: AND_rrrr
        let args = extract_AND_rrrr(inst);
        return Some(Instruction::AND_rrrr { args });
    }
    if (inst & 0x00001000) == 0x00001000 {
        // Matched: EOR_rrrr
        let args = extract_EOR_rrrr(inst);
        return Some(Instruction::EOR_rrrr { args });
    }
    if (inst & 0x00001000) == 0x00001000 {
        // Matched: SUB_rrrr
        let args = extract_SUB_rrrr(inst);
        return Some(Instruction::SUB_rrrr { args });
    }
    if (inst & 0x00001000) == 0x00001000 {
        // Matched: RSB_rrrr
        let args = extract_RSB_rrrr(inst);
        return Some(Instruction::RSB_rrrr { args });
    }
    if (inst & 0x00001000) == 0x00001000 {
        // Matched: ADD_rrrr
        let args = extract_ADD_rrrr(inst);
        return Some(Instruction::ADD_rrrr { args });
    }
    if (inst & 0x00001000) == 0x00001000 {
        // Matched: ADC_rrrr
        let args = extract_ADC_rrrr(inst);
        return Some(Instruction::ADC_rrrr { args });
    }
    if (inst & 0x00001000) == 0x00001000 {
        // Matched: SBC_rrrr
        let args = extract_SBC_rrrr(inst);
        return Some(Instruction::SBC_rrrr { args });
    }
    if (inst & 0x00001000) == 0x00001000 {
        // Matched: RSC_rrrr
        let args = extract_RSC_rrrr(inst);
        return Some(Instruction::RSC_rrrr { args });
    }
    if (inst & 0x0c008000) == 0x0c008000 {
        // Matched: TST_xrrr
        let args = extract_TST_xrrr(inst);
        return Some(Instruction::TST_xrrr { args });
    }
    if (inst & 0x0c008000) == 0x0c008000 {
        // Matched: TEQ_xrrr
        let args = extract_TEQ_xrrr(inst);
        return Some(Instruction::TEQ_xrrr { args });
    }
    if (inst & 0x0c008000) == 0x0c008000 {
        // Matched: CMP_xrrr
        let args = extract_CMP_xrrr(inst);
        return Some(Instruction::CMP_xrrr { args });
    }
    if (inst & 0x0c008000) == 0x0c008000 {
        // Matched: CMN_xrrr
        let args = extract_CMN_xrrr(inst);
        return Some(Instruction::CMN_xrrr { args });
    }
    if (inst & 0x0c000400) == 0x0c000400 {
        // Matched: ORR_rrrr
        let args = extract_ORR_rrrr(inst);
        return Some(Instruction::ORR_rrrr { args });
    }
    if (inst & 0x0c004000) == 0x0c004000 {
        // Matched: MOV_rxrr
        let args = extract_MOV_rxrr(inst);
        return Some(Instruction::MOV_rxrr { args });
    }
    if (inst & 0x0e000200) == 0x0e000200 {
        // Matched: BIC_rrrr
        let args = extract_BIC_rrrr(inst);
        return Some(Instruction::BIC_rrrr { args });
    }
    if (inst & 0x0f001000) == 0x0f001000 {
        // Matched: MVN_rxrr
        let args = extract_MVN_rxrr(inst);
        return Some(Instruction::MVN_rxrr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: AND_rri
        let args = extract_AND_rri(inst);
        return Some(Instruction::AND_rri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: EOR_rri
        let args = extract_EOR_rri(inst);
        return Some(Instruction::EOR_rri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SUB_rri
        let args = extract_SUB_rri(inst);
        return Some(Instruction::SUB_rri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: RSB_rri
        let args = extract_RSB_rri(inst);
        return Some(Instruction::RSB_rri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: ADD_rri
        let args = extract_ADD_rri(inst);
        return Some(Instruction::ADD_rri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: ADC_rri
        let args = extract_ADC_rri(inst);
        return Some(Instruction::ADC_rri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SBC_rri
        let args = extract_SBC_rri(inst);
        return Some(Instruction::SBC_rri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: RSC_rri
        let args = extract_RSC_rri(inst);
        return Some(Instruction::RSC_rri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: TST_xri
        let args = extract_TST_xri(inst);
        return Some(Instruction::TST_xri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: TEQ_xri
        let args = extract_TEQ_xri(inst);
        return Some(Instruction::TEQ_xri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: CMP_xri
        let args = extract_CMP_xri(inst);
        return Some(Instruction::CMP_xri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: CMN_xri
        let args = extract_CMN_xri(inst);
        return Some(Instruction::CMN_xri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: ORR_rri
        let args = extract_ORR_rri(inst);
        return Some(Instruction::ORR_rri { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: MOV_rxi
        let args = extract_MOV_rxi(inst);
        return Some(Instruction::MOV_rxi { args });
    }
    if (inst & 0x0e000000) == 0x0e000000 {
        // Matched: BIC_rri
        let args = extract_BIC_rri(inst);
        return Some(Instruction::BIC_rri { args });
    }
    if (inst & 0x0f000000) == 0x0f000000 {
        // Matched: MVN_rxi
        let args = extract_MVN_rxi(inst);
        return Some(Instruction::MVN_rxi { args });
    }
    if (inst & 0x00040000) == 0x00040000 {
        // Matched: MUL
        let args = extract_MUL(inst);
        return Some(Instruction::MUL { args });
    }
    if (inst & 0x00004000) == 0x00004000 {
        // Matched: MLA
        let args = extract_MLA(inst);
        return Some(Instruction::MLA { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: UMAAL
        let args = extract_UMAAL(inst);
        return Some(Instruction::UMAAL { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: MLS
        let args = extract_MLS(inst);
        return Some(Instruction::MLS { args });
    }
    if (inst & 0x08002000) == 0x08002000 {
        // Matched: UMULL
        let args = extract_UMULL(inst);
        return Some(Instruction::UMULL { args });
    }
    if (inst & 0x08002000) == 0x08002000 {
        // Matched: UMLAL
        let args = extract_UMLAL(inst);
        return Some(Instruction::UMLAL { args });
    }
    if (inst & 0x0c001000) == 0x0c001000 {
        // Matched: SMULL
        let args = extract_SMULL(inst);
        return Some(Instruction::SMULL { args });
    }
    if (inst & 0x0e000800) == 0x0e000800 {
        // Matched: SMLAL
        let args = extract_SMLAL(inst);
        return Some(Instruction::SMLAL { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: QADD
        let args = extract_QADD(inst);
        return Some(Instruction::QADD { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: QSUB
        let args = extract_QSUB(inst);
        return Some(Instruction::QSUB { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: QDADD
        let args = extract_QDADD(inst);
        return Some(Instruction::QDADD { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: QDSUB
        let args = extract_QDSUB(inst);
        return Some(Instruction::QDSUB { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: SMLABB
        let args = extract_SMLABB(inst);
        return Some(Instruction::SMLABB { args });
    }
    if (inst & 0x0000c000) == 0x0000c000 {
        // Matched: SMLABT
        let args = extract_SMLABT(inst);
        return Some(Instruction::SMLABT { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: SMLATB
        let args = extract_SMLATB(inst);
        return Some(Instruction::SMLATB { args });
    }
    if (inst & 0x0000e000) == 0x0000e000 {
        // Matched: SMLATT
        let args = extract_SMLATT(inst);
        return Some(Instruction::SMLATT { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: SMLAWB
        let args = extract_SMLAWB(inst);
        return Some(Instruction::SMLAWB { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: SMULWB
        let args = extract_SMULWB(inst);
        return Some(Instruction::SMULWB { args });
    }
    if (inst & 0x0000c000) == 0x0000c000 {
        // Matched: SMLAWT
        let args = extract_SMLAWT(inst);
        return Some(Instruction::SMLAWT { args });
    }
    if (inst & 0x000e0000) == 0x000e0000 {
        // Matched: SMULWT
        let args = extract_SMULWT(inst);
        return Some(Instruction::SMULWT { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: SMLALBB
        let args = extract_SMLALBB(inst);
        return Some(Instruction::SMLALBB { args });
    }
    if (inst & 0x0000c000) == 0x0000c000 {
        // Matched: SMLALBT
        let args = extract_SMLALBT(inst);
        return Some(Instruction::SMLALBT { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: SMLALTB
        let args = extract_SMLALTB(inst);
        return Some(Instruction::SMLALTB { args });
    }
    if (inst & 0x0000e000) == 0x0000e000 {
        // Matched: SMLALTT
        let args = extract_SMLALTT(inst);
        return Some(Instruction::SMLALTT { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: SMULBB
        let args = extract_SMULBB(inst);
        return Some(Instruction::SMULBB { args });
    }
    if (inst & 0x000c0000) == 0x000c0000 {
        // Matched: SMULBT
        let args = extract_SMULBT(inst);
        return Some(Instruction::SMULBT { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: SMULTB
        let args = extract_SMULTB(inst);
        return Some(Instruction::SMULTB { args });
    }
    if (inst & 0x000e0000) == 0x000e0000 {
        // Matched: SMULTT
        let args = extract_SMULTT(inst);
        return Some(Instruction::SMULTT { args });
    }
    if (inst & 0x00f00000) == 0x00f00000 {
        // Matched: MSR_imm
        let args = extract_MSR_imm(inst);
        return Some(Instruction::MSR_imm { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: CRC32B
        let args = extract_CRC32B(inst);
        return Some(Instruction::CRC32B { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: CRC32H
        let args = extract_CRC32H(inst);
        return Some(Instruction::CRC32H { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: CRC32W
        let args = extract_CRC32W(inst);
        return Some(Instruction::CRC32W { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: CRC32CB
        let args = extract_CRC32CB(inst);
        return Some(Instruction::CRC32CB { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: CRC32CH
        let args = extract_CRC32CH(inst);
        return Some(Instruction::CRC32CH { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: CRC32CW
        let args = extract_CRC32CW(inst);
        return Some(Instruction::CRC32CW { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: MRS_bank
        let args = extract_mrs_bank(inst);
        return Some(Instruction::MRS_bank { args });
    }
    if (inst & 0x043c0000) == 0x043c0000 {
        // Matched: MSR_bank
        let args = extract_msr_bank(inst);
        return Some(Instruction::MSR_bank { args });
    }
    if (inst & 0x07800000) == 0x07800000 {
        // Matched: MRS_reg
        let args = extract_mrs_reg(inst);
        return Some(Instruction::MRS_reg { args });
    }
    if (inst & 0x043c0000) == 0x043c0000 {
        // Matched: MSR_reg
        let args = extract_msr_reg(inst);
        return Some(Instruction::MSR_reg { args });
    }
    if (inst & 0x0fff0000) == 0x0fff0000 {
        // Matched: BX
        let args = extract_BX(inst);
        return Some(Instruction::BX { args });
    }
    if (inst & 0x0fff0000) == 0x0fff0000 {
        // Matched: BXJ
        let args = extract_BXJ(inst);
        return Some(Instruction::BXJ { args });
    }
    if (inst & 0x0fff0000) == 0x0fff0000 {
        // Matched: BLX_r
        let args = extract_BLX_r(inst);
        return Some(Instruction::BLX_r { args });
    }
    if (inst & 0x0f0f0000) == 0x0f0f0000 {
        // Matched: CLZ
        let args = extract_CLZ(inst);
        return Some(Instruction::CLZ { args });
    }
    if (inst & 0x0e000000) == 0x0e000000 {
        // Matched: ERET
        let args = extract_ERET(inst);
        return Some(Instruction::ERET { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: HLT
        let args = extract_HLT(inst);
        return Some(Instruction::HLT { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: BKPT
        let args = extract_BKPT(inst);
        return Some(Instruction::BKPT { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: HVC
        let args = extract_HVC(inst);
        return Some(Instruction::HVC { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMC
        let args = extract_i(inst);
        return Some(Instruction::SMC { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: STRH_rr
        let args = extract_STRH_rr(inst);
        return Some(Instruction::STRH_rr { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: STRH_rr
        let args = extract_STRH_rr(inst);
        return Some(Instruction::STRH_rr { args });
    }
    if (inst & 0x000c0000) == 0x000c0000 {
        // Matched: LDRD_rr
        let args = extract_LDRD_rr(inst);
        return Some(Instruction::LDRD_rr { args });
    }
    if (inst & 0x000c0000) == 0x000c0000 {
        // Matched: LDRD_rr
        let args = extract_LDRD_rr(inst);
        return Some(Instruction::LDRD_rr { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: STRD_rr
        let args = extract_STRD_rr(inst);
        return Some(Instruction::STRD_rr { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: STRD_rr
        let args = extract_STRD_rr(inst);
        return Some(Instruction::STRD_rr { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: LDRH_rr
        let args = extract_LDRH_rr(inst);
        return Some(Instruction::LDRH_rr { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: LDRH_rr
        let args = extract_LDRH_rr(inst);
        return Some(Instruction::LDRH_rr { args });
    }
    if (inst & 0x000c0000) == 0x000c0000 {
        // Matched: LDRSB_rr
        let args = extract_LDRSB_rr(inst);
        return Some(Instruction::LDRSB_rr { args });
    }
    if (inst & 0x000c0000) == 0x000c0000 {
        // Matched: LDRSB_rr
        let args = extract_LDRSB_rr(inst);
        return Some(Instruction::LDRSB_rr { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: LDRSH_rr
        let args = extract_LDRSH_rr(inst);
        return Some(Instruction::LDRSH_rr { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: LDRSH_rr
        let args = extract_LDRSH_rr(inst);
        return Some(Instruction::LDRSH_rr { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: STRHT_rr
        let args = extract_STRHT_rr(inst);
        return Some(Instruction::STRHT_rr { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: LDRHT_rr
        let args = extract_LDRHT_rr(inst);
        return Some(Instruction::LDRHT_rr { args });
    }
    if (inst & 0x000c0000) == 0x000c0000 {
        // Matched: LDRSBT_rr
        let args = extract_LDRSBT_rr(inst);
        return Some(Instruction::LDRSBT_rr { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: LDRSHT_rr
        let args = extract_LDRSHT_rr(inst);
        return Some(Instruction::LDRSHT_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STR_rr
        let args = extract_STR_rr(inst);
        return Some(Instruction::STR_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STR_rr
        let args = extract_STR_rr(inst);
        return Some(Instruction::STR_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STRB_rr
        let args = extract_STRB_rr(inst);
        return Some(Instruction::STRB_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STRB_rr
        let args = extract_STRB_rr(inst);
        return Some(Instruction::STRB_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDR_rr
        let args = extract_LDR_rr(inst);
        return Some(Instruction::LDR_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDR_rr
        let args = extract_LDR_rr(inst);
        return Some(Instruction::LDR_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDRB_rr
        let args = extract_LDRB_rr(inst);
        return Some(Instruction::LDRB_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDRB_rr
        let args = extract_LDRB_rr(inst);
        return Some(Instruction::LDRB_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STRT_rr
        let args = extract_STRT_rr(inst);
        return Some(Instruction::STRT_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STRBT_rr
        let args = extract_STRBT_rr(inst);
        return Some(Instruction::STRBT_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDRT_rr
        let args = extract_LDRT_rr(inst);
        return Some(Instruction::LDRT_rr { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDRBT_rr
        let args = extract_LDRBT_rr(inst);
        return Some(Instruction::LDRBT_rr { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: STRH_ri
        let args = extract_STRH_ri(inst);
        return Some(Instruction::STRH_ri { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: STRH_ri
        let args = extract_STRH_ri(inst);
        return Some(Instruction::STRH_ri { args });
    }
    if (inst & 0x0000c000) == 0x0000c000 {
        // Matched: LDRD_ri_a32
        let args = extract_LDRD_ri_a32(inst);
        return Some(Instruction::LDRD_ri_a32 { args });
    }
    if (inst & 0x0000c000) == 0x0000c000 {
        // Matched: LDRD_ri_a32
        let args = extract_LDRD_ri_a32(inst);
        return Some(Instruction::LDRD_ri_a32 { args });
    }
    if (inst & 0x0000f000) == 0x0000f000 {
        // Matched: STRD_ri_a32
        let args = extract_STRD_ri_a32(inst);
        return Some(Instruction::STRD_ri_a32 { args });
    }
    if (inst & 0x0000f000) == 0x0000f000 {
        // Matched: STRD_ri_a32
        let args = extract_STRD_ri_a32(inst);
        return Some(Instruction::STRD_ri_a32 { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: LDRH_ri
        let args = extract_LDRH_ri(inst);
        return Some(Instruction::LDRH_ri { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: LDRH_ri
        let args = extract_LDRH_ri(inst);
        return Some(Instruction::LDRH_ri { args });
    }
    if (inst & 0x0000c000) == 0x0000c000 {
        // Matched: LDRSB_ri
        let args = extract_LDRSB_ri(inst);
        return Some(Instruction::LDRSB_ri { args });
    }
    if (inst & 0x0000c000) == 0x0000c000 {
        // Matched: LDRSB_ri
        let args = extract_LDRSB_ri(inst);
        return Some(Instruction::LDRSB_ri { args });
    }
    if (inst & 0x0000f000) == 0x0000f000 {
        // Matched: LDRSH_ri
        let args = extract_LDRSH_ri(inst);
        return Some(Instruction::LDRSH_ri { args });
    }
    if (inst & 0x0000f000) == 0x0000f000 {
        // Matched: LDRSH_ri
        let args = extract_LDRSH_ri(inst);
        return Some(Instruction::LDRSH_ri { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: STRHT_ri
        let args = extract_STRHT_ri(inst);
        return Some(Instruction::STRHT_ri { args });
    }
    if (inst & 0x00008000) == 0x00008000 {
        // Matched: LDRHT_ri
        let args = extract_LDRHT_ri(inst);
        return Some(Instruction::LDRHT_ri { args });
    }
    if (inst & 0x0000c000) == 0x0000c000 {
        // Matched: LDRSBT_ri
        let args = extract_LDRSBT_ri(inst);
        return Some(Instruction::LDRSBT_ri { args });
    }
    if (inst & 0x0000f000) == 0x0000f000 {
        // Matched: LDRSHT_ri
        let args = extract_LDRSHT_ri(inst);
        return Some(Instruction::LDRSHT_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STR_ri
        let args = extract_STR_ri(inst);
        return Some(Instruction::STR_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STR_ri
        let args = extract_STR_ri(inst);
        return Some(Instruction::STR_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STRB_ri
        let args = extract_STRB_ri(inst);
        return Some(Instruction::STRB_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STRB_ri
        let args = extract_STRB_ri(inst);
        return Some(Instruction::STRB_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDR_ri
        let args = extract_LDR_ri(inst);
        return Some(Instruction::LDR_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDR_ri
        let args = extract_LDR_ri(inst);
        return Some(Instruction::LDR_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDRB_ri
        let args = extract_LDRB_ri(inst);
        return Some(Instruction::LDRB_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDRB_ri
        let args = extract_LDRB_ri(inst);
        return Some(Instruction::LDRB_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STRT_ri
        let args = extract_STRT_ri(inst);
        return Some(Instruction::STRT_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: STRBT_ri
        let args = extract_STRBT_ri(inst);
        return Some(Instruction::STRBT_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDRT_ri
        let args = extract_LDRT_ri(inst);
        return Some(Instruction::LDRT_ri { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: LDRBT_ri
        let args = extract_LDRBT_ri(inst);
        return Some(Instruction::LDRBT_ri { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: SWP
        let args = extract_SWP(inst);
        return Some(Instruction::SWP { args });
    }
    if (inst & 0x00080000) == 0x00080000 {
        // Matched: SWPB
        let args = extract_SWPB(inst);
        return Some(Instruction::SWPB { args });
    }
    if (inst & 0x0807c000) == 0x0807c000 {
        // Matched: STREX
        let args = extract_STREX(inst);
        return Some(Instruction::STREX { args });
    }
    if (inst & 0x0807c000) == 0x0807c000 {
        // Matched: STREXD_a32
        let args = extract_STREXD_a32(inst);
        return Some(Instruction::STREXD_a32 { args });
    }
    if (inst & 0x0c03e000) == 0x0c03e000 {
        // Matched: STREXB
        let args = extract_STREXB(inst);
        return Some(Instruction::STREXB { args });
    }
    if (inst & 0x0e01f000) == 0x0e01f000 {
        // Matched: STREXH
        let args = extract_STREXH(inst);
        return Some(Instruction::STREXH { args });
    }
    if (inst & 0x08078000) == 0x08078000 {
        // Matched: STLEX
        let args = extract_STLEX(inst);
        return Some(Instruction::STLEX { args });
    }
    if (inst & 0x08078000) == 0x08078000 {
        // Matched: STLEXD_a32
        let args = extract_STLEXD_a32(inst);
        return Some(Instruction::STLEXD_a32 { args });
    }
    if (inst & 0x0c03c000) == 0x0c03c000 {
        // Matched: STLEXB
        let args = extract_STLEXB(inst);
        return Some(Instruction::STLEXB { args });
    }
    if (inst & 0x0e01e000) == 0x0e01e000 {
        // Matched: STLEXH
        let args = extract_STLEXH(inst);
        return Some(Instruction::STLEXH { args });
    }
    if (inst & 0x087f0000) == 0x087f0000 {
        // Matched: STL
        let args = extract_STL(inst);
        return Some(Instruction::STL { args });
    }
    if (inst & 0x0c3f8000) == 0x0c3f8000 {
        // Matched: STLB
        let args = extract_STLB(inst);
        return Some(Instruction::STLB { args });
    }
    if (inst & 0x0e1fc000) == 0x0e1fc000 {
        // Matched: STLH
        let args = extract_STLH(inst);
        return Some(Instruction::STLH { args });
    }
    if (inst & 0x0807fc00) == 0x0807fc00 {
        // Matched: LDREX
        let args = extract_LDREX(inst);
        return Some(Instruction::LDREX { args });
    }
    if (inst & 0x0807fc00) == 0x0807fc00 {
        // Matched: LDREXD_a32
        let args = extract_LDREXD_a32(inst);
        return Some(Instruction::LDREXD_a32 { args });
    }
    if (inst & 0x0c03fe00) == 0x0c03fe00 {
        // Matched: LDREXB
        let args = extract_LDREXB(inst);
        return Some(Instruction::LDREXB { args });
    }
    if (inst & 0x0f00ff80) == 0x0f00ff80 {
        // Matched: LDREXH
        let args = extract_LDREXH(inst);
        return Some(Instruction::LDREXH { args });
    }
    if (inst & 0x0807f800) == 0x0807f800 {
        // Matched: LDAEX
        let args = extract_LDAEX(inst);
        return Some(Instruction::LDAEX { args });
    }
    if (inst & 0x0807f800) == 0x0807f800 {
        // Matched: LDAEXD_a32
        let args = extract_LDAEXD_a32(inst);
        return Some(Instruction::LDAEXD_a32 { args });
    }
    if (inst & 0x0c03fc00) == 0x0c03fc00 {
        // Matched: LDAEXB
        let args = extract_LDAEXB(inst);
        return Some(Instruction::LDAEXB { args });
    }
    if (inst & 0x0f00ff00) == 0x0f00ff00 {
        // Matched: LDAEXH
        let args = extract_LDAEXH(inst);
        return Some(Instruction::LDAEXH { args });
    }
    if (inst & 0x0807f000) == 0x0807f000 {
        // Matched: LDA
        let args = extract_LDA(inst);
        return Some(Instruction::LDA { args });
    }
    if (inst & 0x0c03f800) == 0x0c03f800 {
        // Matched: LDAB
        let args = extract_LDAB(inst);
        return Some(Instruction::LDAB { args });
    }
    if (inst & 0x0f00fe00) == 0x0f00fe00 {
        // Matched: LDAH
        let args = extract_LDAH(inst);
        return Some(Instruction::LDAH { args });
    }
    if (inst & 0x08000000) == 0x08000000 {
        // Matched: USADA8
        let args = extract_USADA8(inst);
        return Some(Instruction::USADA8 { args });
    }
    if (inst & 0x08001000) == 0x08001000 {
        // Matched: SBFX
        let args = extract_SBFX(inst);
        return Some(Instruction::SBFX { args });
    }
    if (inst & 0x0e000400) == 0x0e000400 {
        // Matched: UBFX
        let args = extract_UBFX(inst);
        return Some(Instruction::UBFX { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: BFCI
        let args = extract_bfi(inst);
        return Some(Instruction::BFCI { args });
    }
    if (inst & 0xfe001e00) == 0xfe001e00 {
        // Matched: UDF
        let args = extract_UDF(inst);
        return Some(Instruction::UDF { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: SADD16
        let args = extract_SADD16(inst);
        return Some(Instruction::SADD16 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: SASX
        let args = extract_SASX(inst);
        return Some(Instruction::SASX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: SSAX
        let args = extract_SSAX(inst);
        return Some(Instruction::SSAX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: SSUB16
        let args = extract_SSUB16(inst);
        return Some(Instruction::SSUB16 { args });
    }
    if (inst & 0x000f8000) == 0x000f8000 {
        // Matched: SADD8
        let args = extract_SADD8(inst);
        return Some(Instruction::SADD8 { args });
    }
    if (inst & 0x000ff000) == 0x000ff000 {
        // Matched: SSUB8
        let args = extract_SSUB8(inst);
        return Some(Instruction::SSUB8 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: QADD16
        let args = extract_QADD16(inst);
        return Some(Instruction::QADD16 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: QASX
        let args = extract_QASX(inst);
        return Some(Instruction::QASX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: QSAX
        let args = extract_QSAX(inst);
        return Some(Instruction::QSAX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: QSUB16
        let args = extract_QSUB16(inst);
        return Some(Instruction::QSUB16 { args });
    }
    if (inst & 0x000f8000) == 0x000f8000 {
        // Matched: QADD8
        let args = extract_QADD8(inst);
        return Some(Instruction::QADD8 { args });
    }
    if (inst & 0x000ff000) == 0x000ff000 {
        // Matched: QSUB8
        let args = extract_QSUB8(inst);
        return Some(Instruction::QSUB8 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: SHADD16
        let args = extract_SHADD16(inst);
        return Some(Instruction::SHADD16 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: SHASX
        let args = extract_SHASX(inst);
        return Some(Instruction::SHASX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: SHSAX
        let args = extract_SHSAX(inst);
        return Some(Instruction::SHSAX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: SHSUB16
        let args = extract_SHSUB16(inst);
        return Some(Instruction::SHSUB16 { args });
    }
    if (inst & 0x000f8000) == 0x000f8000 {
        // Matched: SHADD8
        let args = extract_SHADD8(inst);
        return Some(Instruction::SHADD8 { args });
    }
    if (inst & 0x000ff000) == 0x000ff000 {
        // Matched: SHSUB8
        let args = extract_SHSUB8(inst);
        return Some(Instruction::SHSUB8 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: UADD16
        let args = extract_UADD16(inst);
        return Some(Instruction::UADD16 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: UASX
        let args = extract_UASX(inst);
        return Some(Instruction::UASX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: USAX
        let args = extract_USAX(inst);
        return Some(Instruction::USAX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: USUB16
        let args = extract_USUB16(inst);
        return Some(Instruction::USUB16 { args });
    }
    if (inst & 0x000f8000) == 0x000f8000 {
        // Matched: UADD8
        let args = extract_UADD8(inst);
        return Some(Instruction::UADD8 { args });
    }
    if (inst & 0x000ff000) == 0x000ff000 {
        // Matched: USUB8
        let args = extract_USUB8(inst);
        return Some(Instruction::USUB8 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: UQADD16
        let args = extract_UQADD16(inst);
        return Some(Instruction::UQADD16 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: UQASX
        let args = extract_UQASX(inst);
        return Some(Instruction::UQASX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: UQSAX
        let args = extract_UQSAX(inst);
        return Some(Instruction::UQSAX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: UQSUB16
        let args = extract_UQSUB16(inst);
        return Some(Instruction::UQSUB16 { args });
    }
    if (inst & 0x000f8000) == 0x000f8000 {
        // Matched: UQADD8
        let args = extract_UQADD8(inst);
        return Some(Instruction::UQADD8 { args });
    }
    if (inst & 0x000ff000) == 0x000ff000 {
        // Matched: UQSUB8
        let args = extract_UQSUB8(inst);
        return Some(Instruction::UQSUB8 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: UHADD16
        let args = extract_UHADD16(inst);
        return Some(Instruction::UHADD16 { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: UHASX
        let args = extract_UHASX(inst);
        return Some(Instruction::UHASX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: UHSAX
        let args = extract_UHSAX(inst);
        return Some(Instruction::UHSAX { args });
    }
    if (inst & 0x000f0000) == 0x000f0000 {
        // Matched: UHSUB16
        let args = extract_UHSUB16(inst);
        return Some(Instruction::UHSUB16 { args });
    }
    if (inst & 0x000f8000) == 0x000f8000 {
        // Matched: UHADD8
        let args = extract_UHADD8(inst);
        return Some(Instruction::UHADD8 { args });
    }
    if (inst & 0x000ff000) == 0x000ff000 {
        // Matched: UHSUB8
        let args = extract_UHSUB8(inst);
        return Some(Instruction::UHSUB8 { args });
    }
    if (inst & 0x08000000) == 0x08000000 {
        // Matched: PKH
        let args = extract_pkh(inst);
        return Some(Instruction::PKH { args });
    }
    if (inst & 0x08000000) == 0x08000000 {
        // Matched: SSAT
        let args = extract_SSAT(inst);
        return Some(Instruction::SSAT { args });
    }
    if (inst & 0x0e000000) == 0x0e000000 {
        // Matched: USAT
        let args = extract_USAT(inst);
        return Some(Instruction::USAT { args });
    }
    if (inst & 0x08078000) == 0x08078000 {
        // Matched: SSAT16
        let args = extract_SSAT16(inst);
        return Some(Instruction::SSAT16 { args });
    }
    if (inst & 0x0e01e000) == 0x0e01e000 {
        // Matched: USAT16
        let args = extract_USAT16(inst);
        return Some(Instruction::USAT16 { args });
    }
    if (inst & 0x08000000) == 0x08000000 {
        // Matched: SXTAB16
        let args = extract_SXTAB16(inst);
        return Some(Instruction::SXTAB16 { args });
    }
    if (inst & 0x08000000) == 0x08000000 {
        // Matched: SXTAB
        let args = extract_SXTAB(inst);
        return Some(Instruction::SXTAB { args });
    }
    if (inst & 0x08000000) == 0x08000000 {
        // Matched: SXTAH
        let args = extract_SXTAH(inst);
        return Some(Instruction::SXTAH { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: UXTAB16
        let args = extract_UXTAB16(inst);
        return Some(Instruction::UXTAB16 { args });
    }
    if (inst & 0x0e000000) == 0x0e000000 {
        // Matched: UXTAB
        let args = extract_UXTAB(inst);
        return Some(Instruction::UXTAB { args });
    }
    if (inst & 0x0f000000) == 0x0f000000 {
        // Matched: UXTAH
        let args = extract_UXTAH(inst);
        return Some(Instruction::UXTAH { args });
    }
    if (inst & 0x0807c000) == 0x0807c000 {
        // Matched: SEL
        let args = extract_SEL(inst);
        return Some(Instruction::SEL { args });
    }
    if (inst & 0x0f878000) == 0x0f878000 {
        // Matched: REV
        let args = extract_REV(inst);
        return Some(Instruction::REV { args });
    }
    if (inst & 0x0f87c000) == 0x0f87c000 {
        // Matched: REV16
        let args = extract_REV16(inst);
        return Some(Instruction::REV16 { args });
    }
    if (inst & 0x0ff0f800) == 0x0ff0f800 {
        // Matched: REVSH
        let args = extract_REVSH(inst);
        return Some(Instruction::REVSH { args });
    }
    if (inst & 0x0ff0f000) == 0x0ff0f000 {
        // Matched: RBIT
        let args = extract_RBIT(inst);
        return Some(Instruction::RBIT { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMLAD
        let args = extract_SMLAD(inst);
        return Some(Instruction::SMLAD { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMLADX
        let args = extract_SMLADX(inst);
        return Some(Instruction::SMLADX { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMLSD
        let args = extract_SMLSD(inst);
        return Some(Instruction::SMLSD { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMLSDX
        let args = extract_SMLSDX(inst);
        return Some(Instruction::SMLSDX { args });
    }
    if (inst & 0x00f00000) == 0x00f00000 {
        // Matched: SDIV
        let args = extract_SDIV(inst);
        return Some(Instruction::SDIV { args });
    }
    if (inst & 0x00f00000) == 0x00f00000 {
        // Matched: UDIV
        let args = extract_UDIV(inst);
        return Some(Instruction::UDIV { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMLALD
        let args = extract_SMLALD(inst);
        return Some(Instruction::SMLALD { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMLALDX
        let args = extract_SMLALDX(inst);
        return Some(Instruction::SMLALDX { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMLSLD
        let args = extract_SMLSLD(inst);
        return Some(Instruction::SMLSLD { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMLSLDX
        let args = extract_SMLSLDX(inst);
        return Some(Instruction::SMLSLDX { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMMLA
        let args = extract_SMMLA(inst);
        return Some(Instruction::SMMLA { args });
    }
    if (inst & 0x00000000) == 0x00000000 {
        // Matched: SMMLAR
        let args = extract_SMMLAR(inst);
        return Some(Instruction::SMMLAR { args });
    }
    if (inst & 0x0000c000) == 0x0000c000 {
        // Matched: SMMLS
        let args = extract_SMMLS(inst);
        return Some(Instruction::SMMLS { args });
    }
    if (inst & 0x0000f000) == 0x0000f000 {
        // Matched: SMMLSR
        let args = extract_SMMLSR(inst);
        return Some(Instruction::SMMLSR { args });
    }
    if (inst & 0x08000000) == 0x08000000 {
        // Matched: STM
        let args = extract_ldst_block(inst);
        return Some(Instruction::STM { args });
    }
    if (inst & 0x08400000) == 0x08400000 {
        // Matched: LDM_a32
        let args = extract_ldst_block(inst);
        return Some(Instruction::LDM_a32 { args });
    }
    if (inst & 0x08000000) == 0x08000000 {
        // Matched: B
        let args = extract_B(inst);
        return Some(Instruction::B { args });
    }
    if (inst & 0x08000000) == 0x08000000 {
        // Matched: BL
        let args = extract_BL(inst);
        return Some(Instruction::BL { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: MCRR
        let args = extract_MCRR(inst);
        return Some(Instruction::MCRR { args });
    }
    if (inst & 0x0c000000) == 0x0c000000 {
        // Matched: MRRC
        let args = extract_MRRC(inst);
        return Some(Instruction::MRRC { args });
    }
    if (inst & 0x0e000040) == 0x0e000040 {
        // Matched: MCR
        let args = extract_MCR(inst);
        return Some(Instruction::MCR { args });
    }
    if (inst & 0x0e200020) == 0x0e200020 {
        // Matched: MRC
        let args = extract_MRC(inst);
        return Some(Instruction::MRC { args });
    }
    if (inst & 0x0f000000) == 0x0f000000 {
        // Matched: SVC
        let args = extract_i(inst);
        return Some(Instruction::SVC { args });
    }
    None
}
