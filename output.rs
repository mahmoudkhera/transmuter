fn extract_simple(inst: u32, len: u32, pos: u32) -> u32 {
    (inst >> len) & ((1u32 << pos) - 1)
}
fn extract_mul(inst: u32, len1: u32, pos1: u32, len2: u32, pos2: u32) -> u32 {
    // mask = (1 << len) - 1
    let mask1 = (1u32 << len1) - 1;
    let mask2 = (1u32 << len2) - 1;

    let field1 = (inst >> pos1) & mask1;
    let field2 = (inst >> pos2) & mask2;

    // concatenate field1 (lower bits) and field2 (upper bits)
    field1 | (field2 << len1)
}
pub struct msr_bank {
    pub rn: u32,
    pub r: u32,
    pub sysm: u32,
}
pub struct s_rrrr {
    pub s: u32,
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}
pub struct mcrr {
    pub cp: u32,
    pub opc1: u32,
    pub crm: u32,
    pub rt: u32,
    pub rt2: u32,
}
pub struct mcr {
    pub cp: u32,
    pub opc1: u32,
    pub crn: u32,
    pub crm: u32,
    pub opc2: u32,
    pub rt: u32,
}
pub struct s_rrr_shi {
    pub s: u32,
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub shim: u32,
    pub shty: u32,
}
pub struct empty {}
pub struct rr {
    pub rd: u32,
    pub rm: u32,
}
pub struct s_rrr_shr {
    pub s: u32,
    pub rn: u32,
    pub rd: u32,
    pub rm: u32,
    pub rs: u32,
    pub shty: u32,
}
pub struct i {
    pub imm: u32,
}
pub struct ri {
    pub rd: u32,
    pub imm: u32,
}
pub struct r {
    pub rm: u32,
}
pub struct ldst_rr {
    pub p: u32,
    pub w: u32,
    pub u: u32,
    pub rn: u32,
    pub rt: u32,
    pub rm: u32,
    pub shimm: u32,
    pub shtype: u32,
}
pub struct ldst_block {
    pub rn: u32,
    pub i: u32,
    pub b: u32,
    pub u: u32,
    pub w: u32,
    pub list: u32,
}
pub struct mrs_bank {
    pub rd: u32,
    pub r: u32,
    pub sysm: u32,
}
pub struct rrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
}
pub struct rrrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}
pub struct strex {
    pub rn: u32,
    pub rd: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
}
pub struct ldrex {
    pub rn: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
}
pub struct bfx {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub widthm1: u32,
}
pub struct rrr_rot {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub rot: u32,
}
pub struct s_rri_rot {
    pub s: u32,
    pub rn: u32,
    pub rd: u32,
    pub imm: u32,
    pub rot: u32,
}
pub struct bfi {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub msb: u32,
}
pub struct sat {
    pub rd: u32,
    pub rn: u32,
    pub satimm: u32,
    pub imm: u32,
    pub sh: u32,
}
pub struct pkh {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub imm: u32,
    pub tb: u32,
}
pub struct mrs_reg {
    pub rd: u32,
    pub r: u32,
}
pub struct msr_i {
    pub r: u32,
    pub mask: u32,
    pub rot: u32,
    pub imm: u32,
}
pub struct ldst_ri {
    pub p: u32,
    pub w: u32,
    pub u: u32,
    pub rn: u32,
    pub rt: u32,
    pub imm: u32,
}
pub struct msr_reg {
    pub rn: u32,
    pub r: u32,
    pub mask: u32,
}
pub fn extract_mov16(inst: u32) -> ri {
    ri {
        imm: extract_mul(inst, 16, 4, 0, 12),
        rd: extract_simple(inst, 4, 12),
    }
}

pub fn extract_i16(inst: u32) -> i {
    i {
        imm: extract_mul(inst, 8, 12, 0, 4),
    }
}

pub fn extract_S_xri_rot(inst: u32) -> s_rri_rot {
    s_rri_rot {
        rd: 0,
        s: 1,
        rn: extract_simple(inst, 4, 16),
        imm: extract_simple(inst, 8, 0),
        rot: extract_mul(inst, 8, 4),
    }
}

pub fn extract_ldst_ri12_p1w(inst: u32) -> ldst_ri {
    ldst_ri {
        u: extract_simple(inst, 1, 23),
        imm: extract_simple(inst, 12, 0),
        rn: extract_simple(inst, 4, 16),
        w: extract_simple(inst, 1, 21),
        rt: extract_simple(inst, 4, 12),
        p: 1,
    }
}

pub fn extract_swp(inst: u32) -> swp {
    swp {
        rt: extract_simple(inst, 4, 12),
        rt2: extract_simple(inst, 4, 0),
        rn: extract_simple(inst, 4, 16),
    }
}

pub fn extract_rdamn(inst: u32) -> rrrr {
    rrrr {
        ra: extract_simple(inst, 4, 12),
        rn: extract_simple(inst, 4, 0),
        rd: extract_simple(inst, 4, 16),
        rm: extract_simple(inst, 4, 8),
    }
}

pub fn extract_ldst_rs_p0w1(inst: u32) -> ldst_rr {
    ldst_rr {
        p: 0,
        shimm: extract_simple(inst, 5, 7),
        rm: extract_simple(inst, 4, 0),
        rn: extract_simple(inst, 4, 16),
        rt: extract_simple(inst, 4, 12),
        w: 0,
        u: extract_simple(inst, 1, 23),
        shtype: extract_simple(inst, 2, 5),
    }
}

pub fn extract_branch(inst: u32) -> i {
    i {
        imm: extract_mul(inst, 0, 0),
    }
}

pub fn extract_ldst_ri8_pw0(inst: u32) -> ldst_ri {
    ldst_ri {
        u: extract_simple(inst, 1, 23),
        rn: extract_simple(inst, 4, 16),
        imm: extract_mul(inst, 8, 4, 0, 4),
        w: 0,
        rt: extract_simple(inst, 4, 12),
        p: 0,
    }
}

pub fn extract_mcr(inst: u32) -> mcr {
    mcr {
        crm: extract_simple(inst, 4, 0),
        opc1: extract_simple(inst, 3, 21),
        crn: extract_simple(inst, 4, 16),
        opc2: extract_simple(inst, 3, 5),
        cp: extract_simple(inst, 4, 8),
        rt: extract_simple(inst, 4, 12),
    }
}

pub fn extract_ldst_rs_pw0(inst: u32) -> ldst_rr {
    ldst_rr {
        p: 0,
        w: 0,
        rn: extract_simple(inst, 4, 16),
        rt: extract_simple(inst, 4, 12),
        rm: extract_simple(inst, 4, 0),
        shtype: extract_simple(inst, 2, 5),
        shimm: extract_simple(inst, 5, 7),
        u: extract_simple(inst, 1, 23),
    }
}

pub fn extract_rdm(inst: u32) -> rr {
    rr {
        rm: extract_simple(inst, 4, 0),
        rd: extract_simple(inst, 4, 12),
    }
}

pub fn extract_s_rxr_shi(inst: u32) -> s_rrr_shi {
    s_rrr_shi {
        shim: extract_simple(inst, 5, 7),
        rd: extract_simple(inst, 4, 12),
        shty: extract_simple(inst, 2, 5),
        rm: extract_simple(inst, 4, 0),
        rn: 0,
        s: extract_simple(inst, 1, 20),
    }
}

pub fn extract_s_rxr_shr(inst: u32) -> s_rrr_shr {
    s_rrr_shr {
        s: extract_simple(inst, 1, 20),
        shty: extract_simple(inst, 2, 5),
        rm: extract_simple(inst, 4, 0),
        rn: 0,
        rd: extract_simple(inst, 4, 12),
        rs: extract_simple(inst, 4, 8),
    }
}

pub fn extract_s_rrr_shr(inst: u32) -> s_rrr_shr {
    s_rrr_shr {
        rm: extract_simple(inst, 4, 0),
        rs: extract_simple(inst, 4, 8),
        shty: extract_simple(inst, 2, 5),
        s: extract_simple(inst, 1, 20),
        rd: extract_simple(inst, 4, 12),
        rn: extract_simple(inst, 4, 16),
    }
}

pub fn extract_s_rd0mn(inst: u32) -> s_rrrr {
    s_rrrr {
        s: extract_simple(inst, 1, 20),
        ra: 0,
        rd: extract_simple(inst, 4, 16),
        rn: extract_simple(inst, 4, 0),
        rm: extract_simple(inst, 4, 8),
    }
}

pub fn extract_rm(inst: u32) -> r {
    r {
        rm: extract_simple(inst, 4, 0),
    }
}

pub fn extract_s_rri_rot(inst: u32) -> s_rri_rot {
    s_rri_rot {
        rn: extract_simple(inst, 4, 16),
        imm: extract_simple(inst, 8, 0),
        rd: extract_simple(inst, 4, 12),
        s: extract_simple(inst, 1, 20),
        rot: extract_mul(inst, 8, 4),
    }
}

pub fn extract_S_xrr_shr(inst: u32) -> s_rrr_shr {
    s_rrr_shr {
        shty: extract_simple(inst, 2, 5),
        rs: extract_simple(inst, 4, 8),
        rm: extract_simple(inst, 4, 0),
        s: 1,
        rn: extract_simple(inst, 4, 16),
        rd: 0,
    }
}

pub fn extract_strex(inst: u32) -> strex {
    strex {
        rt2: 15,
        rd: extract_simple(inst, 4, 12),
        rn: extract_simple(inst, 4, 16),
        rt: extract_simple(inst, 4, 0),
        imm: 0,
    }
}

pub fn extract_stl(inst: u32) -> ldrex {
    ldrex {
        rn: extract_simple(inst, 4, 16),
        rt: extract_simple(inst, 4, 0),
        rt2: 15,
        imm: 0,
    }
}

pub fn extract_ldst_rr_pw0(inst: u32) -> ldst_rr {
    ldst_rr {
        w: 0,
        u: extract_simple(inst, 1, 23),
        shimm: 0,
        p: 0,
        shtype: 0,
        rn: extract_simple(inst, 4, 16),
        rt: extract_simple(inst, 4, 12),
        rm: extract_simple(inst, 4, 0),
    }
}

pub fn extract_ldst_ri12_p0w1(inst: u32) -> ldst_ri {
    ldst_ri {
        rt: extract_simple(inst, 4, 12),
        imm: extract_simple(inst, 12, 0),
        u: extract_simple(inst, 1, 23),
        p: 0,
        rn: extract_simple(inst, 4, 16),
        w: 0,
    }
}

pub fn extract_bfx(inst: u32) -> bfx {
    bfx {
        widthm1: extract_simple(inst, 5, 16),
        lsb: extract_simple(inst, 5, 7),
        rn: extract_simple(inst, 4, 0),
        rd: extract_simple(inst, 4, 12),
    }
}

pub fn extract_sat16(inst: u32) -> sat {
    sat {
        sh: 0,
        rd: extract_simple(inst, 4, 12),
        imm: 0,
        rn: extract_simple(inst, 4, 0),
        satimm: extract_simple(inst, 4, 16),
    }
}

pub fn extract_rdmn(inst: u32) -> rrr {
    rrr {
        rn: extract_simple(inst, 4, 0),
        rd: extract_simple(inst, 4, 16),
        rm: extract_simple(inst, 4, 8),
    }
}

pub fn extract_ldst_ri8_p0w1(inst: u32) -> ldst_ri {
    ldst_ri {
        u: extract_simple(inst, 1, 23),
        rn: extract_simple(inst, 4, 16),
        rt: extract_simple(inst, 4, 12),
        imm: extract_mul(inst, 8, 4, 0, 4),
        p: 0,
        w: 0,
    }
}

pub fn extract_ldst_rs_p1w(inst: u32) -> ldst_rr {
    ldst_rr {
        rt: extract_simple(inst, 4, 12),
        shtype: extract_simple(inst, 2, 5),
        w: extract_simple(inst, 1, 21),
        rm: extract_simple(inst, 4, 0),
        u: extract_simple(inst, 1, 23),
        rn: extract_simple(inst, 4, 16),
        shimm: extract_simple(inst, 5, 7),
        p: 1,
    }
}

pub fn extract_ldst_ri12_pw0(inst: u32) -> ldst_ri {
    ldst_ri {
        imm: extract_simple(inst, 12, 0),
        p: 0,
        w: 0,
        u: extract_simple(inst, 1, 23),
        rt: extract_simple(inst, 4, 12),
        rn: extract_simple(inst, 4, 16),
    }
}

pub fn extract_s_rdamn(inst: u32) -> s_rrrr {
    s_rrrr {
        ra: extract_simple(inst, 4, 12),
        rm: extract_simple(inst, 4, 8),
        rd: extract_simple(inst, 4, 16),
        rn: extract_simple(inst, 4, 0),
        s: extract_simple(inst, 1, 20),
    }
}

pub fn extract_rd0mn(inst: u32) -> rrrr {
    rrrr {
        rm: extract_simple(inst, 4, 8),
        rn: extract_simple(inst, 4, 0),
        ra: 0,
        rd: extract_simple(inst, 4, 16),
    }
}

pub fn extract_S_xrr_shi(inst: u32) -> s_rrr_shi {
    s_rrr_shi {
        s: 1,
        rd: 0,
        rn: extract_simple(inst, 4, 16),
        shim: extract_simple(inst, 5, 7),
        shty: extract_simple(inst, 2, 5),
        rm: extract_simple(inst, 4, 0),
    }
}

pub fn extract_ldst_rr_p1w(inst: u32) -> ldst_rr {
    ldst_rr {
        rn: extract_simple(inst, 4, 16),
        shimm: 0,
        shtype: 0,
        rt: extract_simple(inst, 4, 12),
        w: extract_simple(inst, 1, 21),
        p: 1,
        rm: extract_simple(inst, 4, 0),
        u: extract_simple(inst, 1, 23),
    }
}

pub fn extract_sat(inst: u32) -> sat {
    sat {
        sh: extract_simple(inst, 1, 6),
        rn: extract_simple(inst, 4, 0),
        satimm: extract_simple(inst, 5, 16),
        rd: extract_simple(inst, 4, 12),
        imm: extract_simple(inst, 5, 7),
    }
}

pub fn extract_s_rrr_shi(inst: u32) -> s_rrr_shi {
    s_rrr_shi {
        shim: extract_simple(inst, 5, 7),
        rd: extract_simple(inst, 4, 12),
        rm: extract_simple(inst, 4, 0),
        s: extract_simple(inst, 1, 20),
        shty: extract_simple(inst, 2, 5),
        rn: extract_simple(inst, 4, 16),
    }
}

pub fn extract_s_rxi_rot(inst: u32) -> s_rri_rot {
    s_rri_rot {
        s: extract_simple(inst, 1, 20),
        imm: extract_simple(inst, 8, 0),
        rot: extract_mul(inst, 8, 4),
        rd: extract_simple(inst, 4, 12),
        rn: 0,
    }
}

pub fn extract_mcrr(inst: u32) -> mcrr {
    mcrr {
        rt2: extract_simple(inst, 4, 16),
        opc1: extract_simple(inst, 4, 4),
        crm: extract_simple(inst, 4, 0),
        cp: extract_simple(inst, 4, 8),
        rt: extract_simple(inst, 4, 12),
    }
}

pub fn extract_msr_i(inst: u32) -> msr_i {
    msr_i {
        imm: extract_simple(inst, 8, 0),
        mask: extract_simple(inst, 4, 16),
        rot: extract_simple(inst, 4, 8),
    }
}

pub fn extract_ldst_ri8_p1w(inst: u32) -> ldst_ri {
    ldst_ri {
        rt: extract_simple(inst, 4, 12),
        rn: extract_simple(inst, 4, 16),
        u: extract_simple(inst, 1, 23),
        imm: extract_mul(inst, 8, 4, 0, 4),
        w: extract_simple(inst, 1, 21),
        p: 1,
    }
}

pub fn extract_rndm(inst: u32) -> rrr {
    rrr {
        rd: extract_simple(inst, 4, 12),
        rn: extract_simple(inst, 4, 16),
        rm: extract_simple(inst, 4, 0),
    }
}

pub fn extract_rrr_rot(inst: u32) -> rrr_rot {
    rrr_rot {
        rot: extract_simple(inst, 2, 10),
        rn: extract_simple(inst, 4, 16),
        rm: extract_simple(inst, 4, 0),
        rd: extract_simple(inst, 4, 12),
    }
}

pub fn extract_ldst_rr_p0w1(inst: u32) -> ldst_rr {
    ldst_rr {
        u: extract_simple(inst, 1, 23),
        p: 0,
        w: 0,
        rm: extract_simple(inst, 4, 0),
        shimm: 0,
        rn: extract_simple(inst, 4, 16),
        rt: extract_simple(inst, 4, 12),
        shtype: 0,
    }
}

pub fn extract_ldrex(inst: u32) -> ldrex {
    ldrex {
        imm: 0,
        rn: extract_simple(inst, 4, 16),
        rt: extract_simple(inst, 4, 12),
        rt2: 15,
    }
}
