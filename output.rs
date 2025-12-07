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
#[derive(Debug, Clone)]
pub struct arg_ldst_block {
    pub rn: u32,
    pub i: u32,
    pub b: u32,
    pub u: u32,
    pub w: u32,
    pub list: u32,
}

#[derive(Debug, Clone)]
pub struct arg_rrrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}

#[derive(Debug, Clone)]
pub struct arg_rrr_rot {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub rot: u32,
}

#[derive(Debug, Clone)]
pub struct arg_pkh {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub imm: u32,
    pub tb: u32,
}

#[derive(Debug, Clone)]
pub struct arg_rrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
}

#[derive(Debug, Clone)]
pub struct arg_sat {
    pub rd: u32,
    pub rn: u32,
    pub satimm: u32,
    pub imm: u32,
    pub sh: u32,
}

#[derive(Debug, Clone)]
pub struct arg_s_rrr_shi {
    pub s: u32,
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub shim: u32,
    pub shty: u32,
}

#[derive(Debug, Clone)]
pub struct arg_s_rri_rot {
    pub s: u32,
    pub rn: u32,
    pub rd: u32,
    pub imm: u32,
    pub rot: u32,
}

#[derive(Debug, Clone)]
pub struct arg_swp {
    pub rt: u32,
    pub rt2: u32,
    pub rn: u32,
}

#[derive(Debug, Clone)]
pub struct arg_s_rrrr {
    pub s: u32,
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}

#[derive(Debug, Clone)]
pub struct arg_i {
    pub imm: u32,
}

#[derive(Debug, Clone)]
pub struct arg_msr_reg {
    pub rn: u32,
    pub r: u32,
    pub mask: u32,
}

#[derive(Debug, Clone)]
pub struct arg_rr {
    pub rd: u32,
    pub rm: u32,
}

#[derive(Debug, Clone)]
pub struct arg_msr_bank {
    pub rn: u32,
    pub r: u32,
    pub sysm: u32,
}

#[derive(Debug, Clone)]
pub struct arg_empty {}

#[derive(Debug, Clone)]
pub struct arg_strex {
    pub rn: u32,
    pub rd: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
}

#[derive(Debug, Clone)]
pub struct arg_ldst_ri {
    pub p: u32,
    pub w: u32,
    pub u: u32,
    pub rn: u32,
    pub rt: u32,
    pub imm: u32,
}

#[derive(Debug, Clone)]
pub struct arg_mrs_bank {
    pub rd: u32,
    pub r: u32,
    pub sysm: u32,
}

#[derive(Debug, Clone)]
pub struct arg_mcr {
    pub cp: u32,
    pub opc1: u32,
    pub crn: u32,
    pub crm: u32,
    pub opc2: u32,
    pub rt: u32,
}

#[derive(Debug, Clone)]
pub struct arg_r {
    pub rm: u32,
}

#[derive(Debug, Clone)]
pub struct arg_mcrr {
    pub cp: u32,
    pub opc1: u32,
    pub crm: u32,
    pub rt: u32,
    pub rt2: u32,
}

#[derive(Debug, Clone)]
pub struct arg_bfi {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub msb: u32,
}

#[derive(Debug, Clone)]
pub struct arg_ldrex {
    pub rn: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
}

#[derive(Debug, Clone)]
pub struct arg_msr_i {
    pub r: u32,
    pub mask: u32,
    pub rot: u32,
    pub imm: u32,
}

#[derive(Debug, Clone)]
pub struct arg_mrs_reg {
    pub rd: u32,
    pub r: u32,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct arg_bfx {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub widthm1: u32,
}

#[derive(Debug, Clone)]
pub struct arg_s_rrr_shr {
    pub s: u32,
    pub rn: u32,
    pub rd: u32,
    pub rm: u32,
    pub rs: u32,
    pub shty: u32,
}

#[derive(Debug, Clone)]
pub struct arg_ri {
    pub rd: u32,
    pub imm: u32,
}

pub fn extract_branch(inst: u32) -> arg_i {
    arg_i {
        imm: times_4(extract_signed(inst, 0, 24) as u32),
    }
}

pub fn extract_i16(inst: u32) -> arg_i {
    arg_i {
        imm: extract_mul2(inst, 8, 12, 0, 4),
    }
}

pub fn extract_S_xrr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        shim: extract_simple(inst, 7, 5),
        shty: extract_simple(inst, 5, 2),
        rm: extract_simple(inst, 0, 4),
        s: 1,
        rd: 0,
        rn: extract_simple(inst, 16, 4),
    }
}

pub fn extract_swp(inst: u32) -> arg_swp {
    arg_swp {
        rt: extract_simple(inst, 12, 4),
        rt2: extract_simple(inst, 0, 4),
        rn: extract_simple(inst, 16, 4),
    }
}

pub fn extract_rdamn(inst: u32) -> arg_rrrr {
    arg_rrrr {
        rd: extract_simple(inst, 16, 4),
        rn: extract_simple(inst, 0, 4),
        ra: extract_simple(inst, 12, 4),
        rm: extract_simple(inst, 8, 4),
    }
}

pub fn extract_mov16(inst: u32) -> arg_ri {
    arg_ri {
        rd: extract_simple(inst, 12, 4),
        imm: extract_mul2(inst, 16, 4, 0, 12),
    }
}

pub fn extract_s_rrr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        s: extract_simple(inst, 20, 1),
        rs: extract_simple(inst, 8, 4),
        rn: extract_simple(inst, 16, 4),
        shty: extract_simple(inst, 5, 2),
        rm: extract_simple(inst, 0, 4),
        rd: extract_simple(inst, 12, 4),
    }
}

pub fn extract_ldst_rr_p0w1(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        p: 0,
        rm: extract_simple(inst, 0, 4),
        rt: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
        w: 0,
        shimm: 0,
        u: extract_simple(inst, 23, 1),
        shtype: 0,
    }
}

pub fn extract_ldst_ri12_p0w1(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        imm: extract_simple(inst, 0, 12),
        u: extract_simple(inst, 23, 1),
        rn: extract_simple(inst, 16, 4),
        p: 0,
        rt: extract_simple(inst, 12, 4),
        w: 0,
    }
}

pub fn extract_bfx(inst: u32) -> arg_bfx {
    arg_bfx {
        lsb: extract_simple(inst, 7, 5),
        widthm1: extract_simple(inst, 16, 5),
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 0, 4),
    }
}

pub fn extract_S_xrr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        rs: extract_simple(inst, 8, 4),
        rm: extract_simple(inst, 0, 4),
        rn: extract_simple(inst, 16, 4),
        shty: extract_simple(inst, 5, 2),
        rd: 0,
        s: 1,
    }
}

pub fn extract_strex(inst: u32) -> arg_strex {
    arg_strex {
        rn: extract_simple(inst, 16, 4),
        imm: 0,
        rt: extract_simple(inst, 0, 4),
        rd: extract_simple(inst, 12, 4),
        rt2: 15,
    }
}

pub fn extract_s_rxr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        s: extract_simple(inst, 20, 1),
        shty: extract_simple(inst, 5, 2),
        rs: extract_simple(inst, 8, 4),
        rm: extract_simple(inst, 0, 4),
        rn: 0,
        rd: extract_simple(inst, 12, 4),
    }
}

pub fn extract_ldst_rs_pw0(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        w: 0,
        shimm: extract_simple(inst, 7, 5),
        rn: extract_simple(inst, 16, 4),
        p: 0,
        u: extract_simple(inst, 23, 1),
        rm: extract_simple(inst, 0, 4),
        shtype: extract_simple(inst, 5, 2),
        rt: extract_simple(inst, 12, 4),
    }
}

pub fn extract_ldst_ri12_pw0(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        w: 0,
        rn: extract_simple(inst, 16, 4),
        u: extract_simple(inst, 23, 1),
        rt: extract_simple(inst, 12, 4),
        p: 0,
        imm: extract_simple(inst, 0, 12),
    }
}

pub fn extract_ldst_ri8_p1w(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        rt: extract_simple(inst, 12, 4),
        w: extract_simple(inst, 21, 1),
        u: extract_simple(inst, 23, 1),
        imm: extract_mul2(inst, 8, 4, 0, 4),
        p: 1,
        rn: extract_simple(inst, 16, 4),
    }
}

pub fn extract_s_rd0mn(inst: u32) -> arg_s_rrrr {
    arg_s_rrrr {
        s: extract_simple(inst, 20, 1),
        rd: extract_simple(inst, 16, 4),
        rm: extract_simple(inst, 8, 4),
        rn: extract_simple(inst, 0, 4),
        ra: 0,
    }
}

pub fn extract_sat(inst: u32) -> arg_sat {
    arg_sat {
        satimm: extract_simple(inst, 16, 5),
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 0, 4),
        sh: extract_simple(inst, 6, 1),
        imm: extract_simple(inst, 7, 5),
    }
}

pub fn extract_rrr_rot(inst: u32) -> arg_rrr_rot {
    arg_rrr_rot {
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
        rm: extract_simple(inst, 0, 4),
        rot: extract_simple(inst, 10, 2),
    }
}

pub fn extract_ldst_rr_pw0(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        u: extract_simple(inst, 23, 1),
        rt: extract_simple(inst, 12, 4),
        shimm: 0,
        rm: extract_simple(inst, 0, 4),
        w: 0,
        rn: extract_simple(inst, 16, 4),
        shtype: 0,
        p: 0,
    }
}

pub fn extract_ldst_ri8_p0w1(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        p: 0,
        w: 0,
        imm: extract_mul2(inst, 8, 4, 0, 4),
        u: extract_simple(inst, 23, 1),
        rt: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
    }
}

pub fn extract_ldst_rs_p0w1(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        rm: extract_simple(inst, 0, 4),
        rt: extract_simple(inst, 12, 4),
        w: 0,
        shimm: extract_simple(inst, 7, 5),
        rn: extract_simple(inst, 16, 4),
        p: 0,
        shtype: extract_simple(inst, 5, 2),
        u: extract_simple(inst, 23, 1),
    }
}

pub fn extract_s_rxr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        s: extract_simple(inst, 20, 1),
        rd: extract_simple(inst, 12, 4),
        shim: extract_simple(inst, 7, 5),
        shty: extract_simple(inst, 5, 2),
        rm: extract_simple(inst, 0, 4),
        rn: 0,
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
        rn: extract_simple(inst, 16, 4),
        s: extract_simple(inst, 20, 1),
        rd: extract_simple(inst, 12, 4),
        imm: extract_simple(inst, 0, 8),
    }
}

pub fn extract_S_xri_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        s: 1,
        rot: times_2(extract_simple(inst, 8, 4)),
        imm: extract_simple(inst, 0, 8),
        rn: extract_simple(inst, 16, 4),
        rd: 0,
    }
}

pub fn extract_stl(inst: u32) -> arg_ldrex {
    arg_ldrex {
        rt2: 15,
        rn: extract_simple(inst, 16, 4),
        rt: extract_simple(inst, 0, 4),
        imm: 0,
    }
}

pub fn extract_ldst_rs_p1w(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        w: extract_simple(inst, 21, 1),
        p: 1,
        u: extract_simple(inst, 23, 1),
        shtype: extract_simple(inst, 5, 2),
        rt: extract_simple(inst, 12, 4),
        rm: extract_simple(inst, 0, 4),
        rn: extract_simple(inst, 16, 4),
        shimm: extract_simple(inst, 7, 5),
    }
}

pub fn extract_s_rxi_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        rot: times_2(extract_simple(inst, 8, 4)),
        rn: 0,
        s: extract_simple(inst, 20, 1),
        rd: extract_simple(inst, 12, 4),
        imm: extract_simple(inst, 0, 8),
    }
}

pub fn extract_sat16(inst: u32) -> arg_sat {
    arg_sat {
        satimm: extract_simple(inst, 16, 4),
        rn: extract_simple(inst, 0, 4),
        imm: 0,
        sh: 0,
        rd: extract_simple(inst, 12, 4),
    }
}

pub fn extract_msr_i(inst: u32) -> arg_msr_i {
    arg_msr_i {
        mask: extract_simple(inst, 16, 4),
        imm: extract_simple(inst, 0, 8),
        rot: extract_simple(inst, 8, 4),
    }
}

pub fn extract_rdmn(inst: u32) -> arg_rrr {
    arg_rrr {
        rn: extract_simple(inst, 0, 4),
        rm: extract_simple(inst, 8, 4),
        rd: extract_simple(inst, 16, 4),
    }
}

pub fn extract_mcrr(inst: u32) -> arg_mcrr {
    arg_mcrr {
        opc1: extract_simple(inst, 4, 4),
        crm: extract_simple(inst, 0, 4),
        rt2: extract_simple(inst, 16, 4),
        rt: extract_simple(inst, 12, 4),
        cp: extract_simple(inst, 8, 4),
    }
}

pub fn extract_ldst_ri8_pw0(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        u: extract_simple(inst, 23, 1),
        p: 0,
        imm: extract_mul2(inst, 8, 4, 0, 4),
        rn: extract_simple(inst, 16, 4),
        w: 0,
        rt: extract_simple(inst, 12, 4),
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

pub fn extract_rndm(inst: u32) -> arg_rrr {
    arg_rrr {
        rd: extract_simple(inst, 12, 4),
        rn: extract_simple(inst, 16, 4),
        rm: extract_simple(inst, 0, 4),
    }
}

pub fn extract_ldst_rr_p1w(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        rm: extract_simple(inst, 0, 4),
        u: extract_simple(inst, 23, 1),
        w: extract_simple(inst, 21, 1),
        shtype: 0,
        rn: extract_simple(inst, 16, 4),
        shimm: 0,
        p: 1,
        rt: extract_simple(inst, 12, 4),
    }
}

pub fn extract_rd0mn(inst: u32) -> arg_rrrr {
    arg_rrrr {
        ra: 0,
        rn: extract_simple(inst, 0, 4),
        rm: extract_simple(inst, 8, 4),
        rd: extract_simple(inst, 16, 4),
    }
}

pub fn extract_mcr(inst: u32) -> arg_mcr {
    arg_mcr {
        crn: extract_simple(inst, 16, 4),
        crm: extract_simple(inst, 0, 4),
        rt: extract_simple(inst, 12, 4),
        opc1: extract_simple(inst, 21, 3),
        opc2: extract_simple(inst, 5, 3),
        cp: extract_simple(inst, 8, 4),
    }
}

pub fn extract_s_rrr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        s: extract_simple(inst, 20, 1),
        rd: extract_simple(inst, 12, 4),
        rm: extract_simple(inst, 0, 4),
        shty: extract_simple(inst, 5, 2),
        rn: extract_simple(inst, 16, 4),
        shim: extract_simple(inst, 7, 5),
    }
}

pub fn extract_ldst_ri12_p1w(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        rt: extract_simple(inst, 12, 4),
        imm: extract_simple(inst, 0, 12),
        w: extract_simple(inst, 21, 1),
        u: extract_simple(inst, 23, 1),
        p: 1,
        rn: extract_simple(inst, 16, 4),
    }
}

pub fn extract_s_rdamn(inst: u32) -> arg_s_rrrr {
    arg_s_rrrr {
        rm: extract_simple(inst, 8, 4),
        rn: extract_simple(inst, 0, 4),
        s: extract_simple(inst, 20, 1),
        rd: extract_simple(inst, 16, 4),
        ra: extract_simple(inst, 12, 4),
    }
}

pub fn extract_rdm(inst: u32) -> arg_rr {
    arg_rr {
        rm: extract_simple(inst, 0, 4),
        rd: extract_simple(inst, 12, 4),
    }
}
