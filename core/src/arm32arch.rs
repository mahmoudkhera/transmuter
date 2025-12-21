// Auto-generated from a32.decode
// Do not edit manually

#![allow(non_camel_case_types)]
#![allow(clippy::all)]

// ===== Extraction Helpers =====

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

// ===== Argument Sets =====
#[derive(Debug, Clone, PartialEq)]
pub struct arg_msr_reg {
    pub rn: u32,
    pub r: u32,
    pub mask: u32,
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
pub struct arg_rrrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_empty {}

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
pub struct arg_r {
    pub rm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_s_rri_rot {
    pub s: u32,
    pub rn: u32,
    pub rd: u32,
    pub imm: u32,
    pub rot: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_bfi {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub msb: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_rrr_rot {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub rot: u32,
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
pub struct arg_ri {
    pub rd: u32,
    pub imm: u32,
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
pub struct arg_pkh {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub imm: u32,
    pub tb: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_msr_bank {
    pub rn: u32,
    pub r: u32,
    pub sysm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_ldrex {
    pub rn: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
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
pub struct arg_i {
    pub imm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_bfx {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub widthm1: u32,
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
pub struct arg_rrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_swp {
    pub rt: u32,
    pub rn: u32,
    pub rt2: u32,
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
pub struct arg_strex {
    pub rn: u32,
    pub rd: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_msr_i {
    pub r: u32,
    pub mask: u32,
    pub rot: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_rr {
    pub rd: u32,
    pub rm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_USADA8 {
    pub rd: u32,
    pub ra: u32,
    pub rm: u32,
    pub rn: u32,
}

// ===== Format Extraction =====
pub fn extract_mcr(inst: u32) -> arg_mcr {
    arg_mcr {
        rt: extract_simple(inst, 12, 4),
        crn: extract_simple(inst, 16, 4),
        opc2: extract_simple(inst, 5, 3),
        crm: extract_simple(inst, 0, 4),
        cp: extract_simple(inst, 8, 4),
        opc1: extract_simple(inst, 21, 3),
    }
}

pub fn extract_rdm(inst: u32) -> arg_rr {
    arg_rr {
        rm: extract_simple(inst, 0, 4),
        rd: extract_simple(inst, 12, 4),
    }
}

pub fn extract_mrs_reg(inst: u32) -> arg_mrs_reg {
    arg_mrs_reg {
        rd: extract_simple(inst, 12, 4),
        r: extract_simple(inst, 22, 1),
    }
}

pub fn extract_swp(inst: u32) -> arg_swp {
    arg_swp {
        rt: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
        rt2: extract_simple(inst, 0, 4),
    }
}

pub fn extract_ldst_rs_p0w1(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        rn: extract_simple(inst, 20, 4),
        rm: extract_simple(inst, 4, 4),
        w: 0,
        shtype: extract_simple(inst, 9, 2),
        shimm: extract_simple(inst, 11, 5),
        p: 0,
        rt: extract_simple(inst, 16, 4),
        u: extract_simple(inst, 27, 1),
    }
}

pub fn extract_bfi(inst: u32) -> arg_bfi {
    arg_bfi {
        rd: extract_simple(inst, 12, 4),
        lsb: extract_simple(inst, 7, 5),
        rn: extract_simple(inst, 0, 4),
        msb: extract_simple(inst, 16, 5),
    }
}

pub fn extract_mrs_bank(inst: u32) -> arg_mrs_bank {
    arg_mrs_bank {
        rd: extract_simple(inst, 12, 4),
        r: extract_simple(inst, 22, 1),
        sysm: extract_mul2(inst, 8, 1, 16, 4),
    }
}

pub fn extract_i16(inst: u32) -> arg_i {
    arg_i {
        imm: extract_mul2(inst, 8, 12, 0, 4),
    }
}

pub fn extract_stl(inst: u32) -> arg_ldrex {
    arg_ldrex {
        rn: extract_simple(inst, 16, 4),
        rt2: 15,
        imm: 0,
        rt: extract_simple(inst, 0, 4),
    }
}

pub fn extract_rdamn(inst: u32) -> arg_rrrr {
    arg_rrrr {
        ra: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 0, 4),
        rm: extract_simple(inst, 8, 4),
        rd: extract_simple(inst, 16, 4),
    }
}

pub fn extract_sat16(inst: u32) -> arg_sat {
    arg_sat {
        satimm: extract_simple(inst, 16, 4),
        imm: 0,
        rn: extract_simple(inst, 0, 4),
        rd: extract_simple(inst, 12, 4),
        sh: 0,
    }
}

pub fn extract_msr_i(inst: u32) -> arg_msr_i {
    arg_msr_i {
        imm: extract_simple(inst, 0, 8),
        rot: extract_simple(inst, 8, 4),
        mask: extract_simple(inst, 16, 4),
        r: 1,
    }
}

pub fn extract_ldst_block(inst: u32) -> arg_ldst_block {
    arg_ldst_block {
        u: extract_simple(inst, 22, 1),
        b: extract_simple(inst, 24, 1),
        w: extract_simple(inst, 21, 1),
        i: extract_simple(inst, 23, 1),
        list: extract_simple(inst, 0, 16),
        rn: extract_simple(inst, 16, 4),
    }
}

pub fn extract_s_rd0mn(inst: u32) -> arg_s_rrrr {
    arg_s_rrrr {
        rm: extract_simple(inst, 8, 4),
        s: extract_simple(inst, 20, 1),
        rd: extract_simple(inst, 16, 4),
        rn: extract_simple(inst, 0, 4),
        ra: 0,
    }
}

pub fn extract_ldst_ri8_p1w(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        u: extract_simple(inst, 27, 1),
        imm: extract_mul2(inst, 8, 4, 0, 4),
        rn: extract_simple(inst, 20, 4),
        w: extract_simple(inst, 25, 1),
        rt: extract_simple(inst, 16, 4),
        p: 1,
    }
}

pub fn extract_strex(inst: u32) -> arg_strex {
    arg_strex {
        rt: extract_simple(inst, 0, 4),
        rt2: 15,
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
        imm: 0,
    }
}

pub fn extract_msr_bank(inst: u32) -> arg_msr_bank {
    arg_msr_bank {
        r: extract_simple(inst, 22, 1),
        sysm: extract_mul2(inst, 8, 1, 16, 4),
        rn: extract_simple(inst, 0, 4),
    }
}

pub fn extract_s_rrr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        rm: extract_simple(inst, 0, 4),
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
        rs: extract_simple(inst, 8, 4),
        s: extract_simple(inst, 20, 1),
        shty: extract_simple(inst, 5, 2),
    }
}

pub fn extract_pkh(inst: u32) -> arg_pkh {
    arg_pkh {
        rn: extract_simple(inst, 16, 4),
        imm: extract_simple(inst, 7, 5),
        rm: extract_simple(inst, 0, 4),
        tb: extract_simple(inst, 6, 1),
        rd: extract_simple(inst, 12, 4),
    }
}

pub fn extract_rrr_rot(inst: u32) -> arg_rrr_rot {
    arg_rrr_rot {
        rm: extract_simple(inst, 0, 4),
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
        rot: extract_simple(inst, 10, 2),
    }
}

pub fn extract_ldst_ri12_p1w(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        imm: extract_simple(inst, 4, 12),
        p: 1,
        u: extract_simple(inst, 27, 1),
        rn: extract_simple(inst, 20, 4),
        rt: extract_simple(inst, 16, 4),
        w: extract_simple(inst, 25, 1),
    }
}

pub fn extract_s_rxi_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        rn: 0,
        rot: times_2(extract_simple(inst, 8, 4)),
        rd: extract_simple(inst, 12, 4),
        s: extract_simple(inst, 20, 1),
        imm: extract_simple(inst, 0, 8),
    }
}

pub fn extract_ldst_ri8_pw0(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        u: extract_simple(inst, 27, 1),
        rn: extract_simple(inst, 21, 4),
        rt: extract_simple(inst, 17, 4),
        imm: extract_mul2(inst, 8, 4, 0, 4),
        w: 0,
        p: 0,
    }
}

pub fn extract_rndm(inst: u32) -> arg_rrr {
    arg_rrr {
        rm: extract_simple(inst, 0, 4),
        rn: extract_simple(inst, 16, 4),
        rd: extract_simple(inst, 12, 4),
    }
}

pub fn extract_branch(inst: u32) -> arg_i {
    arg_i {
        imm: times_4(extract_signed(inst, 0, 24) as u32),
    }
}

pub fn extract_rm(inst: u32) -> arg_r {
    arg_r {
        rm: extract_simple(inst, 0, 4),
    }
}

pub fn extract_s_rri_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        rot: times_2(extract_simple(inst, 8, 4)),
        s: extract_simple(inst, 20, 1),
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
        imm: extract_simple(inst, 0, 8),
    }
}

pub fn extract_ldst_rr_p0w1(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        w: 0,
        rn: extract_simple(inst, 20, 4),
        rt: extract_simple(inst, 16, 4),
        p: 0,
        shtype: 0,
        u: extract_simple(inst, 27, 1),
        rm: extract_simple(inst, 4, 4),
        shimm: 0,
    }
}

pub fn extract_USADA8(inst: u32) -> arg_USADA8 {
    arg_USADA8 {
        rd: extract_simple(inst, 16, 4),
        ra: extract_simple(inst, 12, 4),
        rm: extract_simple(inst, 8, 4),
        rn: extract_simple(inst, 0, 4),
    }
}

pub fn extract_sat(inst: u32) -> arg_sat {
    arg_sat {
        rd: extract_simple(inst, 12, 4),
        imm: extract_simple(inst, 7, 5),
        sh: extract_simple(inst, 6, 1),
        satimm: extract_simple(inst, 16, 5),
        rn: extract_simple(inst, 0, 4),
    }
}

pub fn extract_s_rxr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        shty: extract_simple(inst, 5, 2),
        rm: extract_simple(inst, 0, 4),
        rs: extract_simple(inst, 8, 4),
        rd: extract_simple(inst, 12, 4),
        rn: 0,
        s: extract_simple(inst, 20, 1),
    }
}

pub fn extract_ldst_ri8_p0w1(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        w: 0,
        p: 0,
        imm: extract_mul2(inst, 8, 4, 0, 4),
        rn: extract_simple(inst, 20, 4),
        u: extract_simple(inst, 27, 1),
        rt: extract_simple(inst, 16, 4),
    }
}

pub fn extract_rd0mn(inst: u32) -> arg_rrrr {
    arg_rrrr {
        ra: 0,
        rm: extract_simple(inst, 8, 4),
        rd: extract_simple(inst, 16, 4),
        rn: extract_simple(inst, 0, 4),
    }
}

pub fn extract_s_rrr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        rn: extract_simple(inst, 16, 4),
        rd: extract_simple(inst, 12, 4),
        rm: extract_simple(inst, 0, 4),
        shim: extract_simple(inst, 7, 5),
        s: extract_simple(inst, 20, 1),
        shty: extract_simple(inst, 5, 2),
    }
}

pub fn extract_ldst_ri12_pw0(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        imm: extract_simple(inst, 5, 12),
        rt: extract_simple(inst, 17, 4),
        w: 0,
        u: extract_simple(inst, 27, 1),
        p: 0,
        rn: extract_simple(inst, 21, 4),
    }
}

pub fn extract_ldst_rr_pw0(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        shtype: 0,
        rt: extract_simple(inst, 17, 4),
        shimm: 0,
        p: 0,
        rn: extract_simple(inst, 21, 4),
        rm: extract_simple(inst, 5, 4),
        w: 0,
        u: extract_simple(inst, 27, 1),
    }
}

pub fn extract_msr_reg(inst: u32) -> arg_msr_reg {
    arg_msr_reg {
        mask: extract_simple(inst, 16, 4),
        rn: extract_simple(inst, 0, 4),
        r: extract_simple(inst, 22, 1),
    }
}

pub fn extract_ldst_rr_p1w(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        shtype: 0,
        rt: extract_simple(inst, 16, 4),
        rn: extract_simple(inst, 20, 4),
        shimm: 0,
        p: 1,
        rm: extract_simple(inst, 4, 4),
        w: extract_simple(inst, 25, 1),
        u: extract_simple(inst, 27, 1),
    }
}

pub fn extract_s_rdamn(inst: u32) -> arg_s_rrrr {
    arg_s_rrrr {
        rd: extract_simple(inst, 16, 4),
        ra: extract_simple(inst, 12, 4),
        rm: extract_simple(inst, 8, 4),
        rn: extract_simple(inst, 0, 4),
        s: extract_simple(inst, 20, 1),
    }
}

pub fn extract_S_xrr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        rd: 0,
        s: 1,
        rn: extract_simple(inst, 16, 4),
        shty: extract_simple(inst, 5, 2),
        rs: extract_simple(inst, 8, 4),
        rm: extract_simple(inst, 0, 4),
    }
}

pub fn extract_mcrr(inst: u32) -> arg_mcrr {
    arg_mcrr {
        rt: extract_simple(inst, 12, 4),
        cp: extract_simple(inst, 8, 4),
        opc1: extract_simple(inst, 4, 4),
        rt2: extract_simple(inst, 16, 4),
        crm: extract_simple(inst, 0, 4),
    }
}

pub fn extract_ldst_rs_p1w(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        shimm: extract_simple(inst, 11, 5),
        w: extract_simple(inst, 25, 1),
        rt: extract_simple(inst, 16, 4),
        shtype: extract_simple(inst, 9, 2),
        rn: extract_simple(inst, 20, 4),
        rm: extract_simple(inst, 4, 4),
        p: 1,
        u: extract_simple(inst, 27, 1),
    }
}

pub fn extract_s_rxr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        rm: extract_simple(inst, 0, 4),
        shim: extract_simple(inst, 7, 5),
        shty: extract_simple(inst, 5, 2),
        rd: extract_simple(inst, 12, 4),
        s: extract_simple(inst, 20, 1),
        rn: 0,
    }
}

pub fn extract_S_xri_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        rot: times_2(extract_simple(inst, 8, 4)),
        rn: extract_simple(inst, 16, 4),
        imm: extract_simple(inst, 0, 8),
        s: 1,
        rd: 0,
    }
}

pub fn extract_i(inst: u32) -> arg_i {
    arg_i {
        imm: extract_simple(inst, 0, 24),
    }
}

pub fn extract_ldrex(inst: u32) -> arg_ldrex {
    arg_ldrex {
        rt: extract_simple(inst, 12, 4),
        imm: 0,
        rn: extract_simple(inst, 16, 4),
        rt2: 15,
    }
}

pub fn extract_mov16(inst: u32) -> arg_ri {
    arg_ri {
        imm: extract_mul2(inst, 16, 4, 0, 12),
        rd: extract_simple(inst, 12, 4),
    }
}

pub fn extract_bfx(inst: u32) -> arg_bfx {
    arg_bfx {
        rn: extract_simple(inst, 0, 4),
        rd: extract_simple(inst, 12, 4),
        widthm1: extract_simple(inst, 16, 5),
        lsb: extract_simple(inst, 7, 5),
    }
}

pub fn extract_S_xrr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        rm: extract_simple(inst, 0, 4),
        s: 1,
        shty: extract_simple(inst, 5, 2),
        rn: extract_simple(inst, 16, 4),
        rd: 0,
        shim: extract_simple(inst, 7, 5),
    }
}

pub fn extract_ldst_rs_pw0(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        shimm: extract_simple(inst, 12, 5),
        w: 0,
        rt: extract_simple(inst, 17, 4),
        u: extract_simple(inst, 27, 1),
        rn: extract_simple(inst, 21, 4),
        rm: extract_simple(inst, 5, 4),
        p: 0,
        shtype: extract_simple(inst, 10, 2),
    }
}

pub fn extract_ldst_ri12_p0w1(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        imm: extract_simple(inst, 4, 12),
        rn: extract_simple(inst, 20, 4),
        u: extract_simple(inst, 27, 1),
        rt: extract_simple(inst, 16, 4),
        p: 0,
        w: 0,
    }
}

pub fn extract_rdmn(inst: u32) -> arg_rrr {
    arg_rrr {
        rm: extract_simple(inst, 8, 4),
        rd: extract_simple(inst, 16, 4),
        rn: extract_simple(inst, 0, 4),
    }
}

// ===== Pattern Group =====
#[derive(Debug, Clone)]
pub enum Instruction {
    ADC_rri { args: arg_s_rri_rot },
    ADC_rrri { args: arg_s_rrr_shi },
    ADC_rrrr { args: arg_s_rrr_shr },
    ADD_rri { args: arg_s_rri_rot },
    ADD_rrri { args: arg_s_rrr_shi },
    ADD_rrrr { args: arg_s_rrr_shr },
    AND_rri { args: arg_s_rri_rot },
    AND_rrri { args: arg_s_rrr_shi },
    AND_rrrr { args: arg_s_rrr_shr },
    B { args: arg_i },
    BFCI { args: arg_bfi },
    BIC_rri { args: arg_s_rri_rot },
    BIC_rrri { args: arg_s_rrr_shi },
    BIC_rrrr { args: arg_s_rrr_shr },
    BKPT { args: arg_i },
    BL { args: arg_i },
    BLX_r { args: arg_r },
    BX { args: arg_r },
    BXJ { args: arg_r },
    CLZ { args: arg_rr },
    CMN_xri { args: arg_s_rri_rot },
    CMN_xrri { args: arg_s_rrr_shi },
    CMN_xrrr { args: arg_s_rrr_shr },
    CMP_xri { args: arg_s_rri_rot },
    CMP_xrri { args: arg_s_rrr_shi },
    CMP_xrrr { args: arg_s_rrr_shr },
    CRC32B { args: arg_rrr },
    CRC32CB { args: arg_rrr },
    CRC32CH { args: arg_rrr },
    CRC32CW { args: arg_rrr },
    CRC32H { args: arg_rrr },
    CRC32W { args: arg_rrr },
    EOR_rri { args: arg_s_rri_rot },
    EOR_rrri { args: arg_s_rrr_shi },
    EOR_rrrr { args: arg_s_rrr_shr },
    ERET,
    ESB,
    HLT { args: arg_i },
    HVC { args: arg_i },
    LDA { args: arg_ldrex },
    LDAB { args: arg_ldrex },
    LDAEX { args: arg_ldrex },
    LDAEXB { args: arg_ldrex },
    LDAEXD_a32 { args: arg_ldrex },
    LDAEXH { args: arg_ldrex },
    LDAH { args: arg_ldrex },
    LDM_a32 { args: arg_ldst_block },
    LDRBT_ri { args: arg_ldst_ri },
    LDRBT_rr { args: arg_ldst_rr },
    LDRB_ri { args: arg_ldst_ri },
    LDRB_rr { args: arg_ldst_rr },
    LDRD_ri_a32 { args: arg_ldst_ri },
    LDRD_rr { args: arg_ldst_rr },
    LDREX { args: arg_ldrex },
    LDREXB { args: arg_ldrex },
    LDREXD_a32 { args: arg_ldrex },
    LDREXH { args: arg_ldrex },
    LDRHT_ri { args: arg_ldst_ri },
    LDRHT_rr { args: arg_ldst_rr },
    LDRH_ri { args: arg_ldst_ri },
    LDRH_rr { args: arg_ldst_rr },
    LDRSBT_ri { args: arg_ldst_ri },
    LDRSBT_rr { args: arg_ldst_rr },
    LDRSB_ri { args: arg_ldst_ri },
    LDRSB_rr { args: arg_ldst_rr },
    LDRSHT_ri { args: arg_ldst_ri },
    LDRSHT_rr { args: arg_ldst_rr },
    LDRSH_ri { args: arg_ldst_ri },
    LDRSH_rr { args: arg_ldst_rr },
    LDRT_ri { args: arg_ldst_ri },
    LDRT_rr { args: arg_ldst_rr },
    LDR_ri { args: arg_ldst_ri },
    LDR_rr { args: arg_ldst_rr },
    MCR { args: arg_mcr },
    MCRR { args: arg_mcrr },
    MLA { args: arg_s_rrrr },
    MLS { args: arg_rrrr },
    MOVT { args: arg_ri },
    MOVW { args: arg_ri },
    MOV_rxi { args: arg_s_rri_rot },
    MOV_rxri { args: arg_s_rrr_shi },
    MOV_rxrr { args: arg_s_rrr_shr },
    MRC { args: arg_mcr },
    MRRC { args: arg_mcrr },
    MRS_bank { args: arg_mrs_bank },
    MRS_reg { args: arg_mrs_reg },
    MSR_bank { args: arg_msr_bank },
    MSR_imm { args: arg_msr_i },
    MSR_reg { args: arg_msr_reg },
    MUL { args: arg_s_rrrr },
    MVN_rxi { args: arg_s_rri_rot },
    MVN_rxri { args: arg_s_rrr_shi },
    MVN_rxrr { args: arg_s_rrr_shr },
    NOP,
    ORR_rri { args: arg_s_rri_rot },
    ORR_rrri { args: arg_s_rrr_shi },
    ORR_rrrr { args: arg_s_rrr_shr },
    PKH { args: arg_pkh },
    QADD { args: arg_rrr },
    QADD16 { args: arg_rrr },
    QADD8 { args: arg_rrr },
    QASX { args: arg_rrr },
    QDADD { args: arg_rrr },
    QDSUB { args: arg_rrr },
    QSAX { args: arg_rrr },
    QSUB { args: arg_rrr },
    QSUB16 { args: arg_rrr },
    QSUB8 { args: arg_rrr },
    RBIT { args: arg_rr },
    REV { args: arg_rr },
    REV16 { args: arg_rr },
    REVSH { args: arg_rr },
    RSB_rri { args: arg_s_rri_rot },
    RSB_rrri { args: arg_s_rrr_shi },
    RSB_rrrr { args: arg_s_rrr_shr },
    RSC_rri { args: arg_s_rri_rot },
    RSC_rrri { args: arg_s_rrr_shi },
    RSC_rrrr { args: arg_s_rrr_shr },
    SADD16 { args: arg_rrr },
    SADD8 { args: arg_rrr },
    SASX { args: arg_rrr },
    SBC_rri { args: arg_s_rri_rot },
    SBC_rrri { args: arg_s_rrr_shi },
    SBC_rrrr { args: arg_s_rrr_shr },
    SBFX { args: arg_bfx },
    SDIV { args: arg_rrr },
    SEL { args: arg_rrr },
    SHADD16 { args: arg_rrr },
    SHADD8 { args: arg_rrr },
    SHASX { args: arg_rrr },
    SHSAX { args: arg_rrr },
    SHSUB16 { args: arg_rrr },
    SHSUB8 { args: arg_rrr },
    SMC { args: arg_i },
    SMLABB { args: arg_rrrr },
    SMLABT { args: arg_rrrr },
    SMLAD { args: arg_rrrr },
    SMLADX { args: arg_rrrr },
    SMLAL { args: arg_s_rrrr },
    SMLALBB { args: arg_rrrr },
    SMLALBT { args: arg_rrrr },
    SMLALD { args: arg_rrrr },
    SMLALDX { args: arg_rrrr },
    SMLALTB { args: arg_rrrr },
    SMLALTT { args: arg_rrrr },
    SMLATB { args: arg_rrrr },
    SMLATT { args: arg_rrrr },
    SMLAWB { args: arg_rrrr },
    SMLAWT { args: arg_rrrr },
    SMLSD { args: arg_rrrr },
    SMLSDX { args: arg_rrrr },
    SMLSLD { args: arg_rrrr },
    SMLSLDX { args: arg_rrrr },
    SMMLA { args: arg_rrrr },
    SMMLAR { args: arg_rrrr },
    SMMLS { args: arg_rrrr },
    SMMLSR { args: arg_rrrr },
    SMULBB { args: arg_rrrr },
    SMULBT { args: arg_rrrr },
    SMULL { args: arg_s_rrrr },
    SMULTB { args: arg_rrrr },
    SMULTT { args: arg_rrrr },
    SMULWB { args: arg_rrrr },
    SMULWT { args: arg_rrrr },
    SSAT { args: arg_sat },
    SSAT16 { args: arg_sat },
    SSAX { args: arg_rrr },
    SSUB16 { args: arg_rrr },
    SSUB8 { args: arg_rrr },
    STL { args: arg_ldrex },
    STLB { args: arg_ldrex },
    STLEX { args: arg_strex },
    STLEXB { args: arg_strex },
    STLEXD_a32 { args: arg_strex },
    STLEXH { args: arg_strex },
    STLH { args: arg_ldrex },
    STM { args: arg_ldst_block },
    STRBT_ri { args: arg_ldst_ri },
    STRBT_rr { args: arg_ldst_rr },
    STRB_ri { args: arg_ldst_ri },
    STRB_rr { args: arg_ldst_rr },
    STRD_ri_a32 { args: arg_ldst_ri },
    STRD_rr { args: arg_ldst_rr },
    STREX { args: arg_strex },
    STREXB { args: arg_strex },
    STREXD_a32 { args: arg_strex },
    STREXH { args: arg_strex },
    STRHT_ri { args: arg_ldst_ri },
    STRHT_rr { args: arg_ldst_rr },
    STRH_ri { args: arg_ldst_ri },
    STRH_rr { args: arg_ldst_rr },
    STRT_ri { args: arg_ldst_ri },
    STRT_rr { args: arg_ldst_rr },
    STR_ri { args: arg_ldst_ri },
    STR_rr { args: arg_ldst_rr },
    SUB_rri { args: arg_s_rri_rot },
    SUB_rrri { args: arg_s_rrr_shi },
    SUB_rrrr { args: arg_s_rrr_shr },
    SVC { args: arg_i },
    SWP { args: arg_swp },
    SWPB { args: arg_swp },
    SXTAB { args: arg_rrr_rot },
    SXTAB16 { args: arg_rrr_rot },
    SXTAH { args: arg_rrr_rot },
    TEQ_xri { args: arg_s_rri_rot },
    TEQ_xrri { args: arg_s_rrr_shi },
    TEQ_xrrr { args: arg_s_rrr_shr },
    TST_xri { args: arg_s_rri_rot },
    TST_xrri { args: arg_s_rrr_shi },
    TST_xrrr { args: arg_s_rrr_shr },
    UADD16 { args: arg_rrr },
    UADD8 { args: arg_rrr },
    UASX { args: arg_rrr },
    UBFX { args: arg_bfx },
    UDF,
    UDIV { args: arg_rrr },
    UHADD16 { args: arg_rrr },
    UHADD8 { args: arg_rrr },
    UHASX { args: arg_rrr },
    UHSAX { args: arg_rrr },
    UHSUB16 { args: arg_rrr },
    UHSUB8 { args: arg_rrr },
    UMAAL { args: arg_rrrr },
    UMLAL { args: arg_s_rrrr },
    UMULL { args: arg_s_rrrr },
    UQADD16 { args: arg_rrr },
    UQADD8 { args: arg_rrr },
    UQASX { args: arg_rrr },
    UQSAX { args: arg_rrr },
    UQSUB16 { args: arg_rrr },
    UQSUB8 { args: arg_rrr },
    USADA8 { args: arg_USADA8 },
    USAT { args: arg_sat },
    USAT16 { args: arg_sat },
    USAX { args: arg_rrr },
    USUB16 { args: arg_rrr },
    USUB8 { args: arg_rrr },
    UXTAB { args: arg_rrr_rot },
    UXTAB16 { args: arg_rrr_rot },
    UXTAH { args: arg_rrr_rot },
    WFE,
    WFI,
    YIELD,
}
// ===== Decoder Function (skeleton) =====

pub fn decode_instruction(inst: u32) -> Option<Instruction> {
    // Overlap group - check in order
    // Overlap group - check in order
    // Overlap group - check in order
    // No-overlap group - mutually exclusive patterns
    match inst & 0x0ffff0ff {
        0x0320f002 => {
            // WFE
            return Some(Instruction::WFE);
        }
        0x0320f003 => {
            // WFI
            return Some(Instruction::WFI);
        }
        0x0320f010 => {
            // ESB
            return Some(Instruction::ESB);
        }
        0x0320f001 => {
            // YIELD
            return Some(Instruction::YIELD);
        }
        _ => {}
    }
    if (inst & 0x0ffff000) == 0x0320f000 {
        // Matched: NOP
        return Some(Instruction::NOP);
    }
    if (inst & 0x0ff0f000) == 0x0320f000 {
        // Matched: MSR_imm
        let args = extract_msr_i(inst);
        return Some(Instruction::MSR_imm { args });
    }
    if (inst & 0x0ff00ff0) == 0x06600f70 {
        // Matched: UQSUB16
        let args = extract_rndm(inst);
        return Some(Instruction::UQSUB16 { args });
    }
    if (inst & 0x0ff00000) == 0x0c400000 {
        // Matched: MCRR
        let args = extract_mcrr(inst);
        return Some(Instruction::MCRR { args });
    }
    if (inst & 0x0e500ff0) == 0x000000d0 {
        // Matched: LDRD_rr
        let args = extract_ldst_rr_p1w(inst);
        return Some(Instruction::LDRD_rr { args });
    }
    if (inst & 0x0ff000f0) == 0x075000f0 {
        // Matched: SMMLSR
        let args = extract_rdamn(inst);
        return Some(Instruction::SMMLSR { args });
    }
    if (inst & 0x0fef0090) == 0x01a00010 {
        // Matched: MOV_rxrr
        let args = extract_s_rxr_shr(inst);
        return Some(Instruction::MOV_rxrr { args });
    }
    if (inst & 0x0fe00000) == 0x02e00000 {
        // Matched: RSC_rri
        let args = extract_s_rri_rot(inst);
        return Some(Instruction::RSC_rri { args });
    }
    if (inst & 0x0fe00000) == 0x02600000 {
        // Matched: RSB_rri
        let args = extract_s_rri_rot(inst);
        return Some(Instruction::RSB_rri { args });
    }
    if (inst & 0x0fe00000) == 0x02000000 {
        // Matched: AND_rri
        let args = extract_s_rri_rot(inst);
        return Some(Instruction::AND_rri { args });
    }
    if (inst & 0x0fef0000) == 0x03e00000 {
        // Matched: MVN_rxi
        let args = extract_s_rxi_rot(inst);
        return Some(Instruction::MVN_rxi { args });
    }
    if (inst & 0x0ff00ff0) == 0x06700f50 {
        // Matched: UHSAX
        let args = extract_rndm(inst);
        return Some(Instruction::UHSAX { args });
    }
    if (inst & 0x0ff00fff) == 0x01d00e9f {
        // Matched: LDAEXB
        let args = extract_ldrex(inst);
        return Some(Instruction::LDAEXB { args });
    }
    if (inst & 0x0ff00fff) == 0x01b00f9f {
        // Matched: LDREXD_a32
        let args = extract_ldrex(inst);
        return Some(Instruction::LDREXD_a32 { args });
    }
    if (inst & 0x0ff00ff0) == 0x06500ff0 {
        // Matched: USUB8
        let args = extract_rndm(inst);
        return Some(Instruction::USUB8 { args });
    }
    if (inst & 0x0ff000f0) == 0x075000d0 {
        // Matched: SMMLS
        let args = extract_rdamn(inst);
        return Some(Instruction::SMMLS { args });
    }
    if (inst & 0x0ff00ff0) == 0x06300f70 {
        // Matched: SHSUB16
        let args = extract_rndm(inst);
        return Some(Instruction::SHSUB16 { args });
    }
    if (inst & 0x0e500010) == 0x06000000 {
        // Matched: STR_rr
        let args = extract_ldst_rs_p1w(inst);
        return Some(Instruction::STR_rr { args });
    }
    if (inst & 0x0e500010) == 0x06500000 {
        // Matched: LDRBT_rr
        let args = extract_ldst_rs_p0w1(inst);
        return Some(Instruction::LDRBT_rr { args });
    }
    if (inst & 0x0ff0f000) == 0x03500000 {
        // Matched: CMP_xri
        let args = extract_S_xri_rot(inst);
        return Some(Instruction::CMP_xri { args });
    }
    if (inst & 0x0ff000f0) == 0x07000050 {
        // Matched: SMLSD
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLSD { args });
    }
    if (inst & 0x0ff000f0) == 0x00600090 {
        // Matched: MLS
        let args = extract_rdamn(inst);
        return Some(Instruction::MLS { args });
    }
    if (inst & 0x0e5000f0) == 0x005000f0 {
        // Matched: LDRSHT_ri
        let args = extract_ldst_ri8_p0w1(inst);
        return Some(Instruction::LDRSHT_ri { args });
    }
    if (inst & 0x0ff000f0) == 0x07400010 {
        // Matched: SMLALD
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLALD { args });
    }
    if (inst & 0x0e500ff0) == 0x001000d0 {
        // Matched: LDRSB_rr
        let args = extract_ldst_rr_p1w(inst);
        return Some(Instruction::LDRSB_rr { args });
    }
    if (inst & 0x0ff0f0f0) == 0x0730f010 {
        // Matched: UDIV
        let args = extract_rdmn(inst);
        return Some(Instruction::UDIV { args });
    }
    if (inst & 0x0fe00090) == 0x00c00010 {
        // Matched: SBC_rrrr
        let args = extract_s_rrr_shr(inst);
        return Some(Instruction::SBC_rrrr { args });
    }
    if (inst & 0x0fef0010) == 0x01e00000 {
        // Matched: MVN_rxri
        let args = extract_s_rxr_shi(inst);
        return Some(Instruction::MVN_rxri { args });
    }
    if (inst & 0x0ff0fff0) == 0x01c0fc90 {
        // Matched: STLB
        let args = extract_stl(inst);
        return Some(Instruction::STLB { args });
    }
    if (inst & 0x0ff0f010) == 0x01100000 {
        // Matched: TST_xrri
        let args = extract_S_xrr_shi(inst);
        return Some(Instruction::TST_xrri { args });
    }
    if (inst & 0x0fff0ff0) == 0x016f0f10 {
        // Matched: CLZ
        let args = extract_rdm(inst);
        return Some(Instruction::CLZ { args });
    }
    if (inst & 0xfff000f0) == 0xe7f000f0 {
        // Matched: UDF
        return Some(Instruction::UDF);
    }
    if (inst & 0x0fe00090) == 0x00600010 {
        // Matched: RSB_rrrr
        let args = extract_s_rrr_shr(inst);
        return Some(Instruction::RSB_rrrr { args });
    }
    if (inst & 0x0e5000f0) == 0x004000f0 {
        // Matched: STRD_ri_a32
        let args = extract_ldst_ri8_p1w(inst);
        return Some(Instruction::STRD_ri_a32 { args });
    }
    if (inst & 0x0fe00000) == 0x03800000 {
        // Matched: ORR_rri
        let args = extract_s_rri_rot(inst);
        return Some(Instruction::ORR_rri { args });
    }
    if (inst & 0x0e500ff0) == 0x000000b0 {
        // Matched: STRH_rr
        let args = extract_ldst_rr_p1w(inst);
        return Some(Instruction::STRH_rr { args });
    }
    if (inst & 0x0ffffff0) == 0x012fff10 {
        // Matched: BX
        let args = extract_rm(inst);
        return Some(Instruction::BX { args });
    }
    if (inst & 0x0ff00ff0) == 0x01400040 {
        // Matched: CRC32W
        let args = extract_rndm(inst);
        return Some(Instruction::CRC32W { args });
    }
    if (inst & 0x0ff000f0) == 0x01200080 {
        // Matched: SMLAWB
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLAWB { args });
    }
    if (inst & 0x0e500ff0) == 0x001000b0 {
        // Matched: LDRHT_rr
        let args = extract_ldst_rr_p0w1(inst);
        return Some(Instruction::LDRHT_rr { args });
    }
    if (inst & 0x0ff00030) == 0x06800010 {
        // Matched: PKH
        let args = extract_pkh(inst);
        return Some(Instruction::PKH { args });
    }
    if (inst & 0x0fe00090) == 0x00800010 {
        // Matched: ADD_rrrr
        let args = extract_s_rrr_shr(inst);
        return Some(Instruction::ADD_rrrr { args });
    }
    if (inst & 0x0fe00010) == 0x00200000 {
        // Matched: EOR_rrri
        let args = extract_s_rrr_shi(inst);
        return Some(Instruction::EOR_rrri { args });
    }
    if (inst & 0x0fef0000) == 0x03a00000 {
        // Matched: MOV_rxi
        let args = extract_s_rxi_rot(inst);
        return Some(Instruction::MOV_rxi { args });
    }
    if (inst & 0x0ff00ff0) == 0x06700f10 {
        // Matched: UHADD16
        let args = extract_rndm(inst);
        return Some(Instruction::UHADD16 { args });
    }
    if (inst & 0x0e500010) == 0x06500000 {
        // Matched: LDRB_rr
        let args = extract_ldst_rs_p1w(inst);
        return Some(Instruction::LDRB_rr { args });
    }
    if (inst & 0x0ff00ff0) == 0x06200f50 {
        // Matched: QSAX
        let args = extract_rndm(inst);
        return Some(Instruction::QSAX { args });
    }
    if (inst & 0x0ff00ff0) == 0x06500f70 {
        // Matched: USUB16
        let args = extract_rndm(inst);
        return Some(Instruction::USUB16 { args });
    }
    if (inst & 0x0ff00ff0) == 0x01c00f90 {
        // Matched: STREXB
        let args = extract_strex(inst);
        return Some(Instruction::STREXB { args });
    }
    if (inst & 0x0ff0f000) == 0x03300000 {
        // Matched: TEQ_xri
        let args = extract_S_xri_rot(inst);
        return Some(Instruction::TEQ_xri { args });
    }
    if (inst & 0x0fe00090) == 0x01c00010 {
        // Matched: BIC_rrrr
        let args = extract_s_rrr_shr(inst);
        return Some(Instruction::BIC_rrrr { args });
    }
    if (inst & 0x0ff000f0) == 0x010000c0 {
        // Matched: SMLABT
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLABT { args });
    }
    if (inst & 0x0e500000) == 0x04100000 {
        // Matched: LDR_ri
        let args = extract_ldst_ri12_pw0(inst);
        return Some(Instruction::LDR_ri { args });
    }
    if (inst & 0x0e500000) == 0x04000000 {
        // Matched: STRT_ri
        let args = extract_ldst_ri12_p0w1(inst);
        return Some(Instruction::STRT_ri { args });
    }
    if (inst & 0x0fe00090) == 0x00200010 {
        // Matched: EOR_rrrr
        let args = extract_s_rrr_shr(inst);
        return Some(Instruction::EOR_rrrr { args });
    }
    if (inst & 0x0e500000) == 0x04400000 {
        // Matched: STRB_ri
        let args = extract_ldst_ri12_pw0(inst);
        return Some(Instruction::STRB_ri { args });
    }
    if (inst & 0x0ff00000) == 0x0c500000 {
        // Matched: MRRC
        let args = extract_mcrr(inst);
        return Some(Instruction::MRRC { args });
    }
    if (inst & 0x0ff000f0) == 0x01200070 {
        // Matched: BKPT
        let args = extract_i16(inst);
        return Some(Instruction::BKPT { args });
    }
    if (inst & 0x0fe00000) == 0x03c00000 {
        // Matched: BIC_rri
        let args = extract_s_rri_rot(inst);
        return Some(Instruction::BIC_rri { args });
    }
    if (inst & 0x0ff00ff0) == 0x06500f10 {
        // Matched: UADD16
        let args = extract_rndm(inst);
        return Some(Instruction::UADD16 { args });
    }
    if (inst & 0x0ff000f0) == 0x07400070 {
        // Matched: SMLSLDX
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLSLDX { args });
    }
    if (inst & 0x0ff000f0) == 0x010000a0 {
        // Matched: SMLATB
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLATB { args });
    }
    if (inst & 0x0ff00ff0) == 0x06700f70 {
        // Matched: UHSUB16
        let args = extract_rndm(inst);
        return Some(Instruction::UHSUB16 { args });
    }
    if (inst & 0x0ff0f0f0) == 0x012000a0 {
        // Matched: SMULWB
        let args = extract_rd0mn(inst);
        return Some(Instruction::SMULWB { args });
    }
    if (inst & 0x0ff0f090) == 0x01100010 {
        // Matched: TST_xrrr
        let args = extract_S_xrr_shr(inst);
        return Some(Instruction::TST_xrrr { args });
    }
    if (inst & 0x0ff00ff0) == 0x01000050 {
        // Matched: QADD
        let args = extract_rndm(inst);
        return Some(Instruction::QADD { args });
    }
    if (inst & 0x0ff00ff0) == 0x01200040 {
        // Matched: CRC32H
        let args = extract_rndm(inst);
        return Some(Instruction::CRC32H { args });
    }
    if (inst & 0x0e5000f0) == 0x005000d0 {
        // Matched: LDRSB_ri
        let args = extract_ldst_ri8_p1w(inst);
        return Some(Instruction::LDRSB_ri { args });
    }
    if (inst & 0x0ff00fff) == 0x01900e9f {
        // Matched: LDAEX
        let args = extract_ldrex(inst);
        return Some(Instruction::LDAEX { args });
    }
    if (inst & 0x0fff0ff0) == 0x06bf0fb0 {
        // Matched: REV16
        let args = extract_rdm(inst);
        return Some(Instruction::REV16 { args });
    }
    if (inst & 0x0e5000f0) == 0x004000b0 {
        // Matched: STRH_ri
        let args = extract_ldst_ri8_p1w(inst);
        return Some(Instruction::STRH_ri { args });
    }
    if (inst & 0x0ff00ff0) == 0x06500f50 {
        // Matched: USAX
        let args = extract_rndm(inst);
        return Some(Instruction::USAX { args });
    }
    if (inst & 0x0f100010) == 0x0e000010 {
        // Matched: MCR
        let args = extract_mcr(inst);
        return Some(Instruction::MCR { args });
    }
    if (inst & 0x0fe00070) == 0x07a00050 {
        // Matched: SBFX
        let args = extract_bfx(inst);
        return Some(Instruction::SBFX { args });
    }
    if (inst & 0x0ff00ff0) == 0x01a00e90 {
        // Matched: STLEXD_a32
        let args = extract_strex(inst);
        return Some(Instruction::STLEXD_a32 { args });
    }
    if (inst & 0x0f000000) == 0x0b000000 {
        // Matched: BL
        let args = extract_branch(inst);
        return Some(Instruction::BL { args });
    }
    if (inst & 0x0e500010) == 0x06100000 {
        // Matched: LDR_rr
        let args = extract_ldst_rs_p1w(inst);
        return Some(Instruction::LDR_rr { args });
    }
    if (inst & 0x0ff000f0) == 0x014000c0 {
        // Matched: SMLALBT
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLALBT { args });
    }
    if (inst & 0x0ff003f0) == 0x06b00070 {
        // Matched: SXTAH
        let args = extract_rrr_rot(inst);
        return Some(Instruction::SXTAH { args });
    }
    if (inst & 0x0ff000f0) == 0x07500030 {
        // Matched: SMMLAR
        let args = extract_rdamn(inst);
        return Some(Instruction::SMMLAR { args });
    }
    if (inst & 0x0e500000) == 0x04400000 {
        // Matched: STRBT_ri
        let args = extract_ldst_ri12_p0w1(inst);
        return Some(Instruction::STRBT_ri { args });
    }
    if (inst & 0x0ff000f0) == 0x07000070 {
        // Matched: SMLSDX
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLSDX { args });
    }
    if (inst & 0x0ff000f0) == 0x07400050 {
        // Matched: SMLSLD
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLSLD { args });
    }
    if (inst & 0x0ff00ff0) == 0x06300f30 {
        // Matched: SHASX
        let args = extract_rndm(inst);
        return Some(Instruction::SHASX { args });
    }
    if (inst & 0x0ff000f0) == 0x014000e0 {
        // Matched: SMLALTT
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLALTT { args });
    }
    if (inst & 0x0ff000f0) == 0x014000a0 {
        // Matched: SMLALTB
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLALTB { args });
    }
    if (inst & 0x0f000000) == 0x0a000000 {
        // Matched: B
        let args = extract_branch(inst);
        return Some(Instruction::B { args });
    }
    if (inst & 0x0ff00ff0) == 0x06200f10 {
        // Matched: QADD16
        let args = extract_rndm(inst);
        return Some(Instruction::QADD16 { args });
    }
    if (inst & 0x0ff00ff0) == 0x06700f30 {
        // Matched: UHASX
        let args = extract_rndm(inst);
        return Some(Instruction::UHASX { args });
    }
    if (inst & 0x0ff00ff0) == 0x01400050 {
        // Matched: QDADD
        let args = extract_rndm(inst);
        return Some(Instruction::QDADD { args });
    }
    if (inst & 0x0ff000f0) == 0x012000c0 {
        // Matched: SMLAWT
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLAWT { args });
    }
    if (inst & 0x0ff000f0) == 0x07000030 {
        // Matched: SMLADX
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLADX { args });
    }
    if (inst & 0x0ff00ff0) == 0x06800fb0 {
        // Matched: SEL
        let args = extract_rndm(inst);
        return Some(Instruction::SEL { args });
    }
    if (inst & 0x0e500ff0) == 0x001000b0 {
        // Matched: LDRH_rr
        let args = extract_ldst_rr_p1w(inst);
        return Some(Instruction::LDRH_rr { args });
    }
    if (inst & 0x0ff00ff0) == 0x06500f30 {
        // Matched: UASX
        let args = extract_rndm(inst);
        return Some(Instruction::UASX { args });
    }
    if (inst & 0x0ff0f010) == 0x01700000 {
        // Matched: CMN_xrri
        let args = extract_S_xrr_shi(inst);
        return Some(Instruction::CMN_xrri { args });
    }
    if (inst & 0x0ff000f0) == 0x07800010 {
        // Matched: USADA8
        let args = extract_USADA8(inst);
        return Some(Instruction::USADA8 { args });
    }
    if (inst & 0x0e500ff0) == 0x001000d0 {
        // Matched: LDRSBT_rr
        let args = extract_ldst_rr_p0w1(inst);
        return Some(Instruction::LDRSBT_rr { args });
    }
    if (inst & 0x0ff00ff0) == 0x01e00e90 {
        // Matched: STLEXH
        let args = extract_strex(inst);
        return Some(Instruction::STLEXH { args });
    }
    if (inst & 0x0ff00ff0) == 0x06100f30 {
        // Matched: SASX
        let args = extract_rndm(inst);
        return Some(Instruction::SASX { args });
    }
    if (inst & 0x0ff003f0) == 0x06a00070 {
        // Matched: SXTAB
        let args = extract_rrr_rot(inst);
        return Some(Instruction::SXTAB { args });
    }
    if (inst & 0x0ff0f0f0) == 0x016000e0 {
        // Matched: SMULTT
        let args = extract_rd0mn(inst);
        return Some(Instruction::SMULTT { args });
    }
    if (inst & 0x0fe00070) == 0x07c00010 {
        // Matched: BFCI
        let args = extract_bfi(inst);
        return Some(Instruction::BFCI { args });
    }
    if (inst & 0x0fb0fef0) == 0x0120f200 {
        // Matched: MSR_bank
        let args = extract_msr_bank(inst);
        return Some(Instruction::MSR_bank { args });
    }
    if (inst & 0x0fe000f0) == 0x00c00090 {
        // Matched: SMULL
        let args = extract_s_rdamn(inst);
        return Some(Instruction::SMULL { args });
    }
    if (inst & 0x0ff00ff0) == 0x06200f70 {
        // Matched: QSUB16
        let args = extract_rndm(inst);
        return Some(Instruction::QSUB16 { args });
    }
    if (inst & 0x0ff0f0f0) == 0x016000c0 {
        // Matched: SMULBT
        let args = extract_rd0mn(inst);
        return Some(Instruction::SMULBT { args });
    }
    if (inst & 0x0ff00ff0) == 0x01600050 {
        // Matched: QDSUB
        let args = extract_rndm(inst);
        return Some(Instruction::QDSUB { args });
    }
    if (inst & 0x0ff00ff0) == 0x01000240 {
        // Matched: CRC32CB
        let args = extract_rndm(inst);
        return Some(Instruction::CRC32CB { args });
    }
    if (inst & 0x0ff00ff0) == 0x01200240 {
        // Matched: CRC32CH
        let args = extract_rndm(inst);
        return Some(Instruction::CRC32CH { args });
    }
    if (inst & 0x0ff00ff0) == 0x01400240 {
        // Matched: CRC32CW
        let args = extract_rndm(inst);
        return Some(Instruction::CRC32CW { args });
    }
    if (inst & 0x0ff00ff0) == 0x01800f90 {
        // Matched: STREX
        let args = extract_strex(inst);
        return Some(Instruction::STREX { args });
    }
    if (inst & 0x0e500010) == 0x06000000 {
        // Matched: STRT_rr
        let args = extract_ldst_rs_p0w1(inst);
        return Some(Instruction::STRT_rr { args });
    }
    if (inst & 0x0ff0f0f0) == 0x01600080 {
        // Matched: SMULBB
        let args = extract_rd0mn(inst);
        return Some(Instruction::SMULBB { args });
    }
    if (inst & 0x0ff00fff) == 0x01f00e9f {
        // Matched: LDAEXH
        let args = extract_ldrex(inst);
        return Some(Instruction::LDAEXH { args });
    }
    if (inst & 0x0f000000) == 0x0f000000 {
        // Matched: SVC
        let args = extract_i(inst);
        return Some(Instruction::SVC { args });
    }
    if (inst & 0x0fe00030) == 0x06a00010 {
        // Matched: SSAT
        let args = extract_sat(inst);
        return Some(Instruction::SSAT { args });
    }
    if (inst & 0x0e500ff0) == 0x000000f0 {
        // Matched: STRD_rr
        let args = extract_ldst_rr_p1w(inst);
        return Some(Instruction::STRD_rr { args });
    }
    if (inst & 0x0fe00070) == 0x07e00050 {
        // Matched: UBFX
        let args = extract_bfx(inst);
        return Some(Instruction::UBFX { args });
    }
    if (inst & 0x0ff000f0) == 0x07400030 {
        // Matched: SMLALDX
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLALDX { args });
    }
    if (inst & 0x0ff00ff0) == 0x01000040 {
        // Matched: CRC32B
        let args = extract_rndm(inst);
        return Some(Instruction::CRC32B { args });
    }
    if (inst & 0x0ff00ff0) == 0x06300f90 {
        // Matched: SHADD8
        let args = extract_rndm(inst);
        return Some(Instruction::SHADD8 { args });
    }
    if (inst & 0x0ff000f0) == 0x01400080 {
        // Matched: SMLALBB
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLALBB { args });
    }
    if (inst & 0x0ff0f0f0) == 0x0710f010 {
        // Matched: SDIV
        let args = extract_rdmn(inst);
        return Some(Instruction::SDIV { args });
    }
    if (inst & 0x0fe00010) == 0x00c00000 {
        // Matched: SBC_rrri
        let args = extract_s_rrr_shi(inst);
        return Some(Instruction::SBC_rrri { args });
    }
    if (inst & 0x0ff00ff0) == 0x06e00f30 {
        // Matched: USAT16
        let args = extract_sat16(inst);
        return Some(Instruction::USAT16 { args });
    }
    if (inst & 0x0e500000) == 0x04000000 {
        // Matched: STR_ri
        let args = extract_ldst_ri12_pw0(inst);
        return Some(Instruction::STR_ri { args });
    }
    if (inst & 0x0fe00010) == 0x00e00000 {
        // Matched: RSC_rrri
        let args = extract_s_rrr_shi(inst);
        return Some(Instruction::RSC_rrri { args });
    }
    if (inst & 0x0e5000f0) == 0x005000b0 {
        // Matched: LDRHT_ri
        let args = extract_ldst_ri8_p0w1(inst);
        return Some(Instruction::LDRHT_ri { args });
    }
    if (inst & 0x0ff0f010) == 0x01300000 {
        // Matched: TEQ_xrri
        let args = extract_S_xrr_shi(inst);
        return Some(Instruction::TEQ_xrri { args });
    }
    if (inst & 0x0ff00fff) == 0x01d00c9f {
        // Matched: LDAB
        let args = extract_ldrex(inst);
        return Some(Instruction::LDAB { args });
    }
    if (inst & 0x0ff0f090) == 0x01500010 {
        // Matched: CMP_xrrr
        let args = extract_S_xrr_shr(inst);
        return Some(Instruction::CMP_xrrr { args });
    }
    if (inst & 0x0ff00ff0) == 0x01800e90 {
        // Matched: STLEX
        let args = extract_strex(inst);
        return Some(Instruction::STLEX { args });
    }
    if (inst & 0x0e5000f0) == 0x004000b0 {
        // Matched: STRHT_ri
        let args = extract_ldst_ri8_p0w1(inst);
        return Some(Instruction::STRHT_ri { args });
    }
    if (inst & 0x0ff00ff0) == 0x06200f30 {
        // Matched: QASX
        let args = extract_rndm(inst);
        return Some(Instruction::QASX { args });
    }
    if (inst & 0x0fe000f0) == 0x00200090 {
        // Matched: MLA
        let args = extract_s_rdamn(inst);
        return Some(Instruction::MLA { args });
    }
    if (inst & 0x0ff00ff0) == 0x06300f10 {
        // Matched: SHADD16
        let args = extract_rndm(inst);
        return Some(Instruction::SHADD16 { args });
    }
    if (inst & 0x0ff00ff0) == 0x06600f30 {
        // Matched: UQASX
        let args = extract_rndm(inst);
        return Some(Instruction::UQASX { args });
    }
    if (inst & 0x0e5000f0) == 0x005000f0 {
        // Matched: LDRSH_ri
        let args = extract_ldst_ri8_p1w(inst);
        return Some(Instruction::LDRSH_ri { args });
    }
    if (inst & 0x0ff00ff0) == 0x06300f50 {
        // Matched: SHSAX
        let args = extract_rndm(inst);
        return Some(Instruction::SHSAX { args });
    }
    if (inst & 0x0fe00010) == 0x00400000 {
        // Matched: SUB_rrri
        let args = extract_s_rrr_shi(inst);
        return Some(Instruction::SUB_rrri { args });
    }
    if (inst & 0x0fbf0fff) == 0x010f0000 {
        // Matched: MRS_reg
        let args = extract_mrs_reg(inst);
        return Some(Instruction::MRS_reg { args });
    }
    if (inst & 0x0e500000) == 0x04500000 {
        // Matched: LDRB_ri
        let args = extract_ldst_ri12_pw0(inst);
        return Some(Instruction::LDRB_ri { args });
    }
    if (inst & 0x0ff0fff0) == 0x0180fc90 {
        // Matched: STL
        let args = extract_stl(inst);
        return Some(Instruction::STL { args });
    }
    if (inst & 0x0ff0fff0) == 0x01e0fc90 {
        // Matched: STLH
        let args = extract_stl(inst);
        return Some(Instruction::STLH { args });
    }
    if (inst & 0x0ff003f0) == 0x06800070 {
        // Matched: SXTAB16
        let args = extract_rrr_rot(inst);
        return Some(Instruction::SXTAB16 { args });
    }
    if (inst & 0x0ff00fff) == 0x01f00c9f {
        // Matched: LDAH
        let args = extract_ldrex(inst);
        return Some(Instruction::LDAH { args });
    }
    if (inst & 0x0e100000) == 0x08000000 {
        // Matched: STM
        let args = extract_ldst_block(inst);
        return Some(Instruction::STM { args });
    }
    if (inst & 0x0fe00000) == 0x02800000 {
        // Matched: ADD_rri
        let args = extract_s_rri_rot(inst);
        return Some(Instruction::ADD_rri { args });
    }
    if (inst & 0x0ff0f000) == 0x03700000 {
        // Matched: CMN_xri
        let args = extract_S_xri_rot(inst);
        return Some(Instruction::CMN_xri { args });
    }
    if (inst & 0x0ff00ff0) == 0x06300ff0 {
        // Matched: SHSUB8
        let args = extract_rndm(inst);
        return Some(Instruction::SHSUB8 { args });
    }
    if (inst & 0x0fe00010) == 0x00800000 {
        // Matched: ADD_rrri
        let args = extract_s_rrr_shi(inst);
        return Some(Instruction::ADD_rrri { args });
    }
    if (inst & 0x0fef0090) == 0x01e00010 {
        // Matched: MVN_rxrr
        let args = extract_s_rxr_shr(inst);
        return Some(Instruction::MVN_rxrr { args });
    }
    if (inst & 0x0e500ff0) == 0x001000f0 {
        // Matched: LDRSH_rr
        let args = extract_ldst_rr_p1w(inst);
        return Some(Instruction::LDRSH_rr { args });
    }
    if (inst & 0x0ff000f0) == 0x010000e0 {
        // Matched: SMLATT
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLATT { args });
    }
    if (inst & 0x0ff00fff) == 0x01900f9f {
        // Matched: LDREX
        let args = extract_ldrex(inst);
        return Some(Instruction::LDREX { args });
    }
    if (inst & 0x0fe000f0) == 0x00800090 {
        // Matched: UMULL
        let args = extract_s_rdamn(inst);
        return Some(Instruction::UMULL { args });
    }
    if (inst & 0x0fe00000) == 0x02200000 {
        // Matched: EOR_rri
        let args = extract_s_rri_rot(inst);
        return Some(Instruction::EOR_rri { args });
    }
    if (inst & 0x0ff00ff0) == 0x06600ff0 {
        // Matched: UQSUB8
        let args = extract_rndm(inst);
        return Some(Instruction::UQSUB8 { args });
    }
    if (inst & 0x0e500ff0) == 0x000000b0 {
        // Matched: STRHT_rr
        let args = extract_ldst_rr_p0w1(inst);
        return Some(Instruction::STRHT_rr { args });
    }
    if (inst & 0x0e500010) == 0x06400000 {
        // Matched: STRB_rr
        let args = extract_ldst_rs_p1w(inst);
        return Some(Instruction::STRB_rr { args });
    }
    if (inst & 0x0ff0f000) == 0x0360f000 {
        // Matched: MSR_imm
        let args = extract_msr_i(inst);
        return Some(Instruction::MSR_imm { args });
    }
    if (inst & 0x0f100010) == 0x0e100010 {
        // Matched: MRC
        let args = extract_mcr(inst);
        return Some(Instruction::MRC { args });
    }
    if (inst & 0x0fff0ff0) == 0x06ff0fb0 {
        // Matched: REVSH
        let args = extract_rdm(inst);
        return Some(Instruction::REVSH { args });
    }
    if (inst & 0x0ff000f0) == 0x01000070 {
        // Matched: HLT
        let args = extract_i16(inst);
        return Some(Instruction::HLT { args });
    }
    if (inst & 0x0fe00010) == 0x00600000 {
        // Matched: RSB_rrri
        let args = extract_s_rrr_shi(inst);
        return Some(Instruction::RSB_rrri { args });
    }
    if (inst & 0x0fe00000) == 0x02400000 {
        // Matched: SUB_rri
        let args = extract_s_rri_rot(inst);
        return Some(Instruction::SUB_rri { args });
    }
    if (inst & 0x0ff0f000) == 0x03100000 {
        // Matched: TST_xri
        let args = extract_S_xri_rot(inst);
        return Some(Instruction::TST_xri { args });
    }
    if (inst & 0x0fe00090) == 0x00e00010 {
        // Matched: RSC_rrrr
        let args = extract_s_rrr_shr(inst);
        return Some(Instruction::RSC_rrrr { args });
    }
    if (inst & 0x0ff00ff0) == 0x06100f10 {
        // Matched: SADD16
        let args = extract_rndm(inst);
        return Some(Instruction::SADD16 { args });
    }
    if (inst & 0x0ff00ff0) == 0x01400090 {
        // Matched: SWPB
        let args = extract_swp(inst);
        return Some(Instruction::SWPB { args });
    }
    if (inst & 0x0ff00000) == 0x03400000 {
        // Matched: MOVT
        let args = extract_mov16(inst);
        return Some(Instruction::MOVT { args });
    }
    if (inst & 0x0ff00ff0) == 0x06600f50 {
        // Matched: UQSAX
        let args = extract_rndm(inst);
        return Some(Instruction::UQSAX { args });
    }
    if (inst & 0x0fff0ff0) == 0x06ff0f30 {
        // Matched: RBIT
        let args = extract_rdm(inst);
        return Some(Instruction::RBIT { args });
    }
    if (inst & 0x0ff000f0) == 0x07000010 {
        // Matched: SMLAD
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLAD { args });
    }
    if (inst & 0x0e500000) == 0x04100000 {
        // Matched: LDRT_ri
        let args = extract_ldst_ri12_p0w1(inst);
        return Some(Instruction::LDRT_ri { args });
    }
    if (inst & 0x0e100000) == 0x08100000 {
        // Matched: LDM_a32
        let args = extract_ldst_block(inst);
        return Some(Instruction::LDM_a32 { args });
    }
    if (inst & 0x0fe00010) == 0x00000000 {
        // Matched: AND_rrri
        let args = extract_s_rrr_shi(inst);
        return Some(Instruction::AND_rrri { args });
    }
    if (inst & 0x0ff00ff0) == 0x06100f70 {
        // Matched: SSUB16
        let args = extract_rndm(inst);
        return Some(Instruction::SSUB16 { args });
    }
    if (inst & 0x0ff00000) == 0x03000000 {
        // Matched: MOVW
        let args = extract_mov16(inst);
        return Some(Instruction::MOVW { args });
    }
    if (inst & 0x0e5000f0) == 0x005000b0 {
        // Matched: LDRH_ri
        let args = extract_ldst_ri8_p1w(inst);
        return Some(Instruction::LDRH_ri { args });
    }
    if (inst & 0x0ff00ff0) == 0x01c00e90 {
        // Matched: STLEXB
        let args = extract_strex(inst);
        return Some(Instruction::STLEXB { args });
    }
    if (inst & 0x0ff003f0) == 0x06c00070 {
        // Matched: UXTAB16
        let args = extract_rrr_rot(inst);
        return Some(Instruction::UXTAB16 { args });
    }
    if (inst & 0x0e500010) == 0x06400000 {
        // Matched: STRBT_rr
        let args = extract_ldst_rs_p0w1(inst);
        return Some(Instruction::STRBT_rr { args });
    }
    if (inst & 0x0fe00010) == 0x01c00000 {
        // Matched: BIC_rrri
        let args = extract_s_rrr_shi(inst);
        return Some(Instruction::BIC_rrri { args });
    }
    if (inst & 0x0fe0f0f0) == 0x00000090 {
        // Matched: MUL
        let args = extract_s_rd0mn(inst);
        return Some(Instruction::MUL { args });
    }
    if (inst & 0x0ff00ff0) == 0x06100ff0 {
        // Matched: SSUB8
        let args = extract_rndm(inst);
        return Some(Instruction::SSUB8 { args });
    }
    if (inst & 0x0e500ff0) == 0x001000f0 {
        // Matched: LDRSHT_rr
        let args = extract_ldst_rr_p0w1(inst);
        return Some(Instruction::LDRSHT_rr { args });
    }
    if (inst & 0x0ff00ff0) == 0x01a00f90 {
        // Matched: STREXD_a32
        let args = extract_strex(inst);
        return Some(Instruction::STREXD_a32 { args });
    }
    if (inst & 0x0ff00ff0) == 0x06700ff0 {
        // Matched: UHSUB8
        let args = extract_rndm(inst);
        return Some(Instruction::UHSUB8 { args });
    }
    if (inst & 0x0ff0f0f0) == 0x016000a0 {
        // Matched: SMULTB
        let args = extract_rd0mn(inst);
        return Some(Instruction::SMULTB { args });
    }
    if (inst & 0x0fe000f0) == 0x00a00090 {
        // Matched: UMLAL
        let args = extract_s_rdamn(inst);
        return Some(Instruction::UMLAL { args });
    }
    if (inst & 0x0ff00ff0) == 0x06700f90 {
        // Matched: UHADD8
        let args = extract_rndm(inst);
        return Some(Instruction::UHADD8 { args });
    }
    if (inst & 0x0ff00ff0) == 0x06600f10 {
        // Matched: UQADD16
        let args = extract_rndm(inst);
        return Some(Instruction::UQADD16 { args });
    }
    if (inst & 0x0ff0f090) == 0x01700010 {
        // Matched: CMN_xrrr
        let args = extract_S_xrr_shr(inst);
        return Some(Instruction::CMN_xrrr { args });
    }
    if (inst & 0x0fe00010) == 0x01800000 {
        // Matched: ORR_rrri
        let args = extract_s_rrr_shi(inst);
        return Some(Instruction::ORR_rrri { args });
    }
    if (inst & 0x0fe00090) == 0x01800010 {
        // Matched: ORR_rrrr
        let args = extract_s_rrr_shr(inst);
        return Some(Instruction::ORR_rrrr { args });
    }
    if (inst & 0x0fe00000) == 0x02a00000 {
        // Matched: ADC_rri
        let args = extract_s_rri_rot(inst);
        return Some(Instruction::ADC_rri { args });
    }
    if (inst & 0x0ff0f0f0) == 0x012000e0 {
        // Matched: SMULWT
        let args = extract_rd0mn(inst);
        return Some(Instruction::SMULWT { args });
    }
    if (inst & 0x0ff00ff0) == 0x01200050 {
        // Matched: QSUB
        let args = extract_rndm(inst);
        return Some(Instruction::QSUB { args });
    }
    if (inst & 0x0ff00ff0) == 0x06100f50 {
        // Matched: SSAX
        let args = extract_rndm(inst);
        return Some(Instruction::SSAX { args });
    }
    if (inst & 0x0ffffff0) == 0x012fff20 {
        // Matched: BXJ
        let args = extract_rm(inst);
        return Some(Instruction::BXJ { args });
    }
    if (inst & 0x0ff00ff0) == 0x06100f90 {
        // Matched: SADD8
        let args = extract_rndm(inst);
        return Some(Instruction::SADD8 { args });
    }
    if (inst & 0x0ff000f0) == 0x01000080 {
        // Matched: SMLABB
        let args = extract_rdamn(inst);
        return Some(Instruction::SMLABB { args });
    }
    if (inst & 0x0fffffff) == 0x0160006e {
        // Matched: ERET
        return Some(Instruction::ERET);
    }
    if (inst & 0x0fe00090) == 0x00400010 {
        // Matched: SUB_rrrr
        let args = extract_s_rrr_shr(inst);
        return Some(Instruction::SUB_rrrr { args });
    }
    if (inst & 0x0ff00fff) == 0x01b00e9f {
        // Matched: LDAEXD_a32
        let args = extract_ldrex(inst);
        return Some(Instruction::LDAEXD_a32 { args });
    }
    if (inst & 0x0e5000f0) == 0x005000d0 {
        // Matched: LDRSBT_ri
        let args = extract_ldst_ri8_p0w1(inst);
        return Some(Instruction::LDRSBT_ri { args });
    }
    if (inst & 0x0ff0f010) == 0x01500000 {
        // Matched: CMP_xrri
        let args = extract_S_xrr_shi(inst);
        return Some(Instruction::CMP_xrri { args });
    }
    if (inst & 0x0fe00000) == 0x02c00000 {
        // Matched: SBC_rri
        let args = extract_s_rri_rot(inst);
        return Some(Instruction::SBC_rri { args });
    }
    if (inst & 0x0e5000f0) == 0x004000d0 {
        // Matched: LDRD_ri_a32
        let args = extract_ldst_ri8_p1w(inst);
        return Some(Instruction::LDRD_ri_a32 { args });
    }
    if (inst & 0x0ff00ff0) == 0x06200f90 {
        // Matched: QADD8
        let args = extract_rndm(inst);
        return Some(Instruction::QADD8 { args });
    }
    if (inst & 0x0fe000f0) == 0x00e00090 {
        // Matched: SMLAL
        let args = extract_s_rdamn(inst);
        return Some(Instruction::SMLAL { args });
    }
    if (inst & 0x0e500000) == 0x04500000 {
        // Matched: LDRBT_ri
        let args = extract_ldst_ri12_p0w1(inst);
        return Some(Instruction::LDRBT_ri { args });
    }
    if (inst & 0x0fe00090) == 0x00a00010 {
        // Matched: ADC_rrrr
        let args = extract_s_rrr_shr(inst);
        return Some(Instruction::ADC_rrrr { args });
    }
    if (inst & 0x0fb00eff) == 0x01000200 {
        // Matched: MRS_bank
        let args = extract_mrs_bank(inst);
        return Some(Instruction::MRS_bank { args });
    }
    if (inst & 0x0ff000f0) == 0x07500010 {
        // Matched: SMMLA
        let args = extract_rdamn(inst);
        return Some(Instruction::SMMLA { args });
    }
    if (inst & 0x0ff00ff0) == 0x06500f90 {
        // Matched: UADD8
        let args = extract_rndm(inst);
        return Some(Instruction::UADD8 { args });
    }
    if (inst & 0x0ff00ff0) == 0x06600f90 {
        // Matched: UQADD8
        let args = extract_rndm(inst);
        return Some(Instruction::UQADD8 { args });
    }
    if (inst & 0x0ffffff0) == 0x01600070 {
        // Matched: SMC
        let args = extract_i(inst);
        return Some(Instruction::SMC { args });
    }
    if (inst & 0x0ff00ff0) == 0x01e00f90 {
        // Matched: STREXH
        let args = extract_strex(inst);
        return Some(Instruction::STREXH { args });
    }
    if (inst & 0x0fe00090) == 0x00000010 {
        // Matched: AND_rrrr
        let args = extract_s_rrr_shr(inst);
        return Some(Instruction::AND_rrrr { args });
    }
    if (inst & 0x0ffffff0) == 0x012fff30 {
        // Matched: BLX_r
        let args = extract_rm(inst);
        return Some(Instruction::BLX_r { args });
    }
    if (inst & 0x0ff000f0) == 0x01400070 {
        // Matched: HVC
        let args = extract_i16(inst);
        return Some(Instruction::HVC { args });
    }
    if (inst & 0x0ff003f0) == 0x06e00070 {
        // Matched: UXTAB
        let args = extract_rrr_rot(inst);
        return Some(Instruction::UXTAB { args });
    }
    if (inst & 0x0fff0ff0) == 0x06bf0f30 {
        // Matched: REV
        let args = extract_rdm(inst);
        return Some(Instruction::REV { args });
    }
    if (inst & 0x0ff00fff) == 0x01f00f9f {
        // Matched: LDREXH
        let args = extract_ldrex(inst);
        return Some(Instruction::LDREXH { args });
    }
    if (inst & 0x0ff00ff0) == 0x01000090 {
        // Matched: SWP
        let args = extract_swp(inst);
        return Some(Instruction::SWP { args });
    }
    if (inst & 0x0e500010) == 0x06100000 {
        // Matched: LDRT_rr
        let args = extract_ldst_rs_p0w1(inst);
        return Some(Instruction::LDRT_rr { args });
    }
    if (inst & 0x0ff00ff0) == 0x06200ff0 {
        // Matched: QSUB8
        let args = extract_rndm(inst);
        return Some(Instruction::QSUB8 { args });
    }
    if (inst & 0x0ff003f0) == 0x06f00070 {
        // Matched: UXTAH
        let args = extract_rrr_rot(inst);
        return Some(Instruction::UXTAH { args });
    }
    if (inst & 0x0ff00fff) == 0x01900c9f {
        // Matched: LDA
        let args = extract_ldrex(inst);
        return Some(Instruction::LDA { args });
    }
    if (inst & 0x0fef0010) == 0x01a00000 {
        // Matched: MOV_rxri
        let args = extract_s_rxr_shi(inst);
        return Some(Instruction::MOV_rxri { args });
    }
    if (inst & 0x0fb0fff0) == 0x0120f000 {
        // Matched: MSR_reg
        let args = extract_msr_reg(inst);
        return Some(Instruction::MSR_reg { args });
    }
    if (inst & 0x0fe00030) == 0x06e00010 {
        // Matched: USAT
        let args = extract_sat(inst);
        return Some(Instruction::USAT { args });
    }
    if (inst & 0x0ff0f090) == 0x01300010 {
        // Matched: TEQ_xrrr
        let args = extract_S_xrr_shr(inst);
        return Some(Instruction::TEQ_xrrr { args });
    }
    if (inst & 0x0ff000f0) == 0x00400090 {
        // Matched: UMAAL
        let args = extract_rdamn(inst);
        return Some(Instruction::UMAAL { args });
    }
    if (inst & 0x0ff00ff0) == 0x06a00f30 {
        // Matched: SSAT16
        let args = extract_sat16(inst);
        return Some(Instruction::SSAT16 { args });
    }
    if (inst & 0x0fe00010) == 0x00a00000 {
        // Matched: ADC_rrri
        let args = extract_s_rrr_shi(inst);
        return Some(Instruction::ADC_rrri { args });
    }
    if (inst & 0x0ff00fff) == 0x01d00f9f {
        // Matched: LDREXB
        let args = extract_ldrex(inst);
        return Some(Instruction::LDREXB { args });
    }
    None
}
