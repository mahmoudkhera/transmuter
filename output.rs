// Auto-generated from a32.decode
// Do not edit manually

#![allow(non_camel_case_types)]
#![allow(clippy::all)]

// ===== Extraction Helper Functions =====

 fn extract_simple(inst:u32,pos:u32,len:u32) -> u32 {
    (inst >> pos) & ((1u32 << len) - 1)
 }

 fn extract_signed(inst:u32,pos:u32,len:u32) -> i32 {
    let val = (inst >> pos) & ((1u32 << len) - 1);
    // Sign extend
    if (val & (1u32 << (len - 1))) != 0 {
        (val | (!((1u32 << len) - 1))) as i32
    } else {
        val as i32
    }
 }

 fn extract_mul2(inst:u32,pos1:u32,len1:u32,pos2:u32,len2:u32) -> u32 {
    let mask1 = (1u32 << len1) - 1;
    let mask2 = (1u32 << len2) - 1;
    let field1 = (inst >> pos1) & mask1;
    let field2 = (inst >> pos2) & mask2;
    // concatenate field1 (lower bits) and field2 (upper bits)
    field1 | (field2 << len1)
 }

 fn extract_mul3(inst:u32,pos1:u32,len1:u32,pos2:u32,len2:u32,pos3:u32,len3:u32) -> u32 {
    let mask1 = (1u32 << len1) - 1;
    let mask2 = (1u32 << len2) - 1;
    let mask3 = (1u32 << len3) - 1;
    let field1 = (inst >> pos1) & mask1;
    let field2 = (inst >> pos2) & mask2;
    let field3 = (inst >> pos3) & mask3;
    field1 | (field2 << len1) | (field3 << (len1 + len2))
 }

 fn extract_mul4(inst:u32,pos1:u32,len1:u32,pos2:u32,len2:u32,pos3:u32,len3:u32,pos4:u32,len4:u32) -> u32 {
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

 fn times_2(val:u32) -> u32 {
    val << 1
 }

 fn times_4(val:u32) -> u32 {
    val << 2
 }

 fn times_8(val:u32) -> u32 {
    val << 3
 }

 fn expand_imm(val:u32) -> u32 {
    // ARM immediate expansion logic
    let rotate = (val >> 8) & 0xF;
    let imm = val & 0xFF;
    imm.rotate_right(rotate * 2)
 }

 fn negate(val:u32) -> u32 {
    (!val).wrapping_add(1)
 }

// ===== Argument Structures =====

#[derive(Debug, Clone, PartialEq)]
pub struct arg_empty {
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
pub struct arg_USADA8 {
    pub rd: u32,
    pub rn: u32,
    pub ra: u32,
    pub rm: u32,
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
pub struct arg_r {
    pub rm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_mrs_reg {
    pub rd: u32,
    pub r: u32,
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
pub struct arg_i {
    pub imm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_mrs_bank {
    pub rd: u32,
    pub r: u32,
    pub sysm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_rr {
    pub rd: u32,
    pub rm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_ldrex {
    pub rn: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
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
pub struct arg_msr_reg {
    pub rn: u32,
    pub r: u32,
    pub mask: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_bfx {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub widthm1: u32,
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
pub struct arg_pkh {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub imm: u32,
    pub tb: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_rrrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_msr_bank {
    pub rn: u32,
    pub r: u32,
    pub sysm: u32,
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
pub struct arg_ldst_ri {
    pub p: u32,
    pub w: u32,
    pub u: u32,
    pub rn: u32,
    pub rt: u32,
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
pub struct arg_s_rrrr {
    pub s: u32,
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
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
pub struct arg_ri {
    pub rd: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct arg_bfi {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub msb: u32,
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
pub struct arg_swp {
    pub rt2: u32,
    pub rn: u32,
    pub rt: u32,
}

// ===== Format Extraction Functions =====

pub fn extract_ADD_rrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MVN_rxi(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_USAT16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLATB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STRB_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SADD16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_QSUB16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRB_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_s_rri_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        s : extract_simple(inst,20,1),
        imm : extract_simple(inst,0,8),
        rot : times_2(extract_simple(inst,8,4)),
        rd : extract_simple(inst,12,4),
        rn : extract_simple(inst,16,4),
    }
}

pub fn extract_UHADD16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_mcrr(inst: u32) -> arg_mcrr {
    arg_mcrr {
        crm : extract_simple(inst,0,4),
        cp : extract_simple(inst,8,4),
        rt2 : extract_simple(inst,16,4),
        rt : extract_simple(inst,12,4),
        opc1 : extract_simple(inst,4,4),
    }
}

pub fn extract_LDREX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLSLDX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMMLS(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLAL(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SBFX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_RSB_rrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STR_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_QADD16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDREXH(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UHSUB16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_TST_xri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDR_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ADC_rrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMULL(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_BIC_rrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_AND_rri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STRD_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_sat16(inst: u32) -> arg_sat {
    arg_sat {
        satimm : extract_simple(inst,16,4),
        imm:0,
        sh:0,
        rd : extract_simple(inst,12,4),
        rn : extract_simple(inst,0,4),
    }
}

pub fn extract_ADC_rrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SUB_rri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMMLSR(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRB_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_QDSUB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STREXH(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UQSAX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMMLAR(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_BIC_rrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLALTB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ORR_rri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ldst_rs_p0w1(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        rt : extract_simple(inst,16,4),
        w:0,
        p:0,
        shimm : extract_simple(inst,11,5),
        rn : extract_simple(inst,20,4),
        shtype : extract_simple(inst,9,2),
        rm : extract_simple(inst,4,4),
        u : extract_simple(inst,27,1),
    }
}

pub fn extract_LDA(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_s_rdamn(inst: u32) -> arg_s_rrrr {
    arg_s_rrrr {
        rn : extract_simple(inst,0,4),
        s : extract_simple(inst,20,1),
        rm : extract_simple(inst,8,4),
        ra : extract_simple(inst,12,4),
        rd : extract_simple(inst,16,4),
    }
}

pub fn extract_QADD8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMULTB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SSUB16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_REV16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLAWB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SHASX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_s_rrr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        shty : extract_simple(inst,5,2),
        s : extract_simple(inst,20,1),
        rd : extract_simple(inst,12,4),
        rn : extract_simple(inst,16,4),
        rm : extract_simple(inst,0,4),
        shim : extract_simple(inst,7,5),
    }
}

pub fn extract_ADD_rri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STRHT_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UASX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ldst_rr_p1w(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        shtype:0,
        p:1,
        u : extract_simple(inst,27,1),
        rn : extract_simple(inst,20,4),
        shimm:0,
        w : extract_simple(inst,25,1),
        rt : extract_simple(inst,16,4),
        rm : extract_simple(inst,4,4),
    }
}

pub fn extract_ERET(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_branch(inst: u32) -> arg_i {
    arg_i {
        imm : times_4(extract_signed(inst,0,24) as u32),
    }
}

pub fn extract_RSC_rrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STLB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SBC_rrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ldst_ri8_p0w1(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        rt : extract_simple(inst,16,4),
        p:0,
        w:0,
        u : extract_simple(inst,27,1),
        imm : extract_mul2(inst,8,4,0,4),
        rn : extract_simple(inst,20,4),
    }
}

pub fn extract_SMLALTT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_rd0mn(inst: u32) -> arg_rrrr {
    arg_rrrr {
        rd : extract_simple(inst,16,4),
        rm : extract_simple(inst,8,4),
        rn : extract_simple(inst,0,4),
        ra:0,
    }
}

pub fn extract_TEQ_xrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_CMN_xrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_rm(inst: u32) -> arg_r {
    arg_r {
        rm : extract_simple(inst,0,4),
    }
}

pub fn extract_LDRSH_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ldst_rr_p0w1(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        p:0,
        rn : extract_simple(inst,20,4),
        w:0,
        rm : extract_simple(inst,4,4),
        shimm:0,
        shtype:0,
        u : extract_simple(inst,27,1),
        rt : extract_simple(inst,16,4),
    }
}

pub fn extract_MVN_rxri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_TEQ_xrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDAB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_QSAX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_CRC32CH(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRD_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ESB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_QADD(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SEL(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_AND_rrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MVN_rxrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_BXJ(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MLA(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_HLT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRSHT_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_WFI(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_USAX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SWP(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STREXD_a32(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_REVSH(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UHASX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLABT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ADD_rrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_S_xri_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        rot : times_2(extract_simple(inst,8,4)),
        imm : extract_simple(inst,0,8),
        s:1,
        rn : extract_simple(inst,16,4),
        rd:0,
    }
}

pub fn extract_SASX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_TST_xrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SHSAX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UQASX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_swp(inst: u32) -> arg_swp {
    arg_swp {
        rt2 : extract_simple(inst,0,4),
        rn : extract_simple(inst,16,4),
        rt : extract_simple(inst,12,4),
    }
}

pub fn extract_strex(inst: u32) -> arg_strex {
    arg_strex {
        rn : extract_simple(inst,16,4),
        rd : extract_simple(inst,12,4),
        rt : extract_simple(inst,0,4),
        rt2:15,
        imm:0,
    }
}

pub fn extract_SMLABB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRH_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STRT_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMULWT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRD_ri_a32(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_WFE(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDREXD_a32(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UHSAX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_stl(inst: u32) -> arg_ldrex {
    arg_ldrex {
        rt : extract_simple(inst,0,4),
        rn : extract_simple(inst,16,4),
        rt2:15,
        imm:0,
    }
}

pub fn extract_SBC_rrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_USUB8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_RSB_rrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UQADD8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STRBT_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_NOP(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_sat(inst: u32) -> arg_sat {
    arg_sat {
        rn : extract_simple(inst,0,4),
        rd : extract_simple(inst,12,4),
        imm : extract_simple(inst,7,5),
        sh : extract_simple(inst,6,1),
        satimm : extract_simple(inst,16,5),
    }
}

pub fn extract_ldst_ri12_p1w(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        imm : extract_simple(inst,4,12),
        p:1,
        rt : extract_simple(inst,16,4),
        u : extract_simple(inst,27,1),
        rn : extract_simple(inst,20,4),
        w : extract_simple(inst,25,1),
    }
}

pub fn extract_ldst_ri12_p0w1(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        imm : extract_simple(inst,4,12),
        u : extract_simple(inst,27,1),
        p:0,
        rn : extract_simple(inst,20,4),
        w:0,
        rt : extract_simple(inst,16,4),
    }
}

pub fn extract_SMLATT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SXTAH(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMULBT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDR_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRT_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ldst_ri8_pw0(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        rt : extract_simple(inst,17,4),
        imm : extract_mul2(inst,8,4,0,4),
        u : extract_simple(inst,27,1),
        rn : extract_simple(inst,21,4),
        p:0,
        w:0,
    }
}

pub fn extract_SMLAWT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_TST_xrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SHSUB16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_CMN_xrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_rrr_rot(inst: u32) -> arg_rrr_rot {
    arg_rrr_rot {
        rn : extract_simple(inst,16,4),
        rd : extract_simple(inst,12,4),
        rot : extract_simple(inst,10,2),
        rm : extract_simple(inst,0,4),
    }
}

pub fn extract_LDAEXB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SUB_rrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STLH(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDAEXH(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STREX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_CRC32W(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLSD(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRBT_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STREXB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MRRC(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRSB_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_rndm(inst: u32) -> arg_rrr {
    arg_rrr {
        rm : extract_simple(inst,0,4),
        rn : extract_simple(inst,16,4),
        rd : extract_simple(inst,12,4),
    }
}

pub fn extract_BX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMULTT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_YIELD(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRHT_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_RBIT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLAD(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_RSB_rri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UMULL(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLALBB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_QDADD(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRSBT_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRSBT_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SHADD8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UXTAB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_BKPT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UQSUB16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLADX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRSH_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SXTAB16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_BL(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SUB_rrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MLS(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STLEXB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_s_rrr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        rd : extract_simple(inst,12,4),
        shty : extract_simple(inst,5,2),
        rm : extract_simple(inst,0,4),
        rn : extract_simple(inst,16,4),
        rs : extract_simple(inst,8,4),
        s : extract_simple(inst,20,1),
    }
}

pub fn extract_s_rd0mn(inst: u32) -> arg_s_rrrr {
    arg_s_rrrr {
        s : extract_simple(inst,20,1),
        rm : extract_simple(inst,8,4),
        rn : extract_simple(inst,0,4),
        rd : extract_simple(inst,16,4),
        ra:0,
    }
}

pub fn extract_ldst_ri12_pw0(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        w:0,
        rn : extract_simple(inst,21,4),
        imm : extract_simple(inst,5,12),
        p:0,
        u : extract_simple(inst,27,1),
        rt : extract_simple(inst,17,4),
    }
}

pub fn extract_SMLSDX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_CMP_xrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ldst_ri8_p1w(inst: u32) -> arg_ldst_ri {
    arg_ldst_ri {
        u : extract_simple(inst,27,1),
        rt : extract_simple(inst,16,4),
        rn : extract_simple(inst,20,4),
        imm : extract_mul2(inst,8,4,0,4),
        p:1,
        w : extract_simple(inst,25,1),
    }
}

pub fn extract_SMULWB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRT_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_msr_i(inst: u32) -> arg_msr_i {
    arg_msr_i {
        rot : extract_simple(inst,8,4),
        mask : extract_simple(inst,16,4),
        imm : extract_simple(inst,0,8),
        r:1,
    }
}

pub fn extract_S_xrr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        shty : extract_simple(inst,5,2),
        rm : extract_simple(inst,0,4),
        s:1,
        shim : extract_simple(inst,7,5),
        rd:0,
        rn : extract_simple(inst,16,4),
    }
}

pub fn extract_CRC32CB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_i16(inst: u32) -> arg_i {
    arg_i {
        imm : extract_mul2(inst,8,12,0,4),
    }
}

pub fn extract_bfx(inst: u32) -> arg_bfx {
    arg_bfx {
        rd : extract_simple(inst,12,4),
        widthm1 : extract_simple(inst,16,5),
        lsb : extract_simple(inst,7,5),
        rn : extract_simple(inst,0,4),
    }
}

pub fn extract_CLZ(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRSHT_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MCRR(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UXTAB16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ldst_rs_p1w(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        rn : extract_simple(inst,20,4),
        u : extract_simple(inst,27,1),
        p:1,
        shimm : extract_simple(inst,11,5),
        shtype : extract_simple(inst,9,2),
        rm : extract_simple(inst,4,4),
        rt : extract_simple(inst,16,4),
        w : extract_simple(inst,25,1),
    }
}

pub fn extract_QASX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MOV_rxri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UDF(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_REV(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_EOR_rrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MCR(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_QSUB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLALBT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MRC(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ldrex(inst: u32) -> arg_ldrex {
    arg_ldrex {
        rt : extract_simple(inst,12,4),
        rn : extract_simple(inst,16,4),
        imm:0,
        rt2:15,
    }
}

pub fn extract_SMLALDX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_BLX_r(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_CMN_xri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STL(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_USAT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRBT_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMMLA(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_rdm(inst: u32) -> arg_rr {
    arg_rr {
        rd : extract_simple(inst,12,4),
        rm : extract_simple(inst,0,4),
    }
}

pub fn extract_CRC32CW(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_rdmn(inst: u32) -> arg_rrr {
    arg_rrr {
        rn : extract_simple(inst,0,4),
        rd : extract_simple(inst,16,4),
        rm : extract_simple(inst,8,4),
    }
}

pub fn extract_UMLAL(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRSB_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UQSUB8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UHSUB8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_TEQ_xri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_EOR_rri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STRB_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRHT_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ldst_rr_pw0(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        u : extract_simple(inst,27,1),
        rm : extract_simple(inst,5,4),
        rn : extract_simple(inst,21,4),
        rt : extract_simple(inst,17,4),
        shimm:0,
        shtype:0,
        w:0,
        p:0,
    }
}

pub fn extract_SWPB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_s_rxi_rot(inst: u32) -> arg_s_rri_rot {
    arg_s_rri_rot {
        rn:0,
        rd : extract_simple(inst,12,4),
        rot : times_2(extract_simple(inst,8,4)),
        s : extract_simple(inst,20,1),
        imm : extract_simple(inst,0,8),
    }
}

pub fn extract_UHADD8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_RSC_rri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_rdamn(inst: u32) -> arg_rrrr {
    arg_rrrr {
        rd : extract_simple(inst,16,4),
        ra : extract_simple(inst,12,4),
        rn : extract_simple(inst,0,4),
        rm : extract_simple(inst,8,4),
    }
}

pub fn extract_MOVT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMULBB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SSAX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UXTAH(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLSLD(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDRH_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_CRC32B(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SSAT16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_AND_rrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_CMP_xri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MOV_rxrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SSAT(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SMLALD(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ORR_rrri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_BIC_rri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UMAAL(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STLEX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SSUB8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_QSUB8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STLEXH(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDAEXD_a32(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UQADD16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SHADD16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_s_rxr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        s : extract_simple(inst,20,1),
        rd : extract_simple(inst,12,4),
        rs : extract_simple(inst,8,4),
        shty : extract_simple(inst,5,2),
        rn:0,
        rm : extract_simple(inst,0,4),
    }
}

pub fn extract_ldst_rs_pw0(inst: u32) -> arg_ldst_rr {
    arg_ldst_rr {
        rt : extract_simple(inst,17,4),
        u : extract_simple(inst,27,1),
        shtype : extract_simple(inst,10,2),
        shimm : extract_simple(inst,12,5),
        p:0,
        rm : extract_simple(inst,5,4),
        rn : extract_simple(inst,21,4),
        w:0,
    }
}

pub fn extract_STLEXD_a32(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_S_xrr_shr(inst: u32) -> arg_s_rrr_shr {
    arg_s_rrr_shr {
        rn : extract_simple(inst,16,4),
        rm : extract_simple(inst,0,4),
        s:1,
        shty : extract_simple(inst,5,2),
        rd:0,
        rs : extract_simple(inst,8,4),
    }
}

pub fn extract_STRD_ri_a32(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STRBT_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SHSUB8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ADC_rri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_USUB16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_RSC_rrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UADD8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SXTAB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_ORR_rrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_B(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_s_rxr_shi(inst: u32) -> arg_s_rrr_shi {
    arg_s_rrr_shi {
        rn:0,
        shim : extract_simple(inst,7,5),
        shty : extract_simple(inst,5,2),
        s : extract_simple(inst,20,1),
        rm : extract_simple(inst,0,4),
        rd : extract_simple(inst,12,4),
    }
}

pub fn extract_STRH_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_mcr(inst: u32) -> arg_mcr {
    arg_mcr {
        opc2 : extract_simple(inst,5,3),
        rt : extract_simple(inst,12,4),
        opc1 : extract_simple(inst,21,3),
        cp : extract_simple(inst,8,4),
        crm : extract_simple(inst,0,4),
        crn : extract_simple(inst,16,4),
    }
}

pub fn extract_SBC_rri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MUL(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_HVC(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UBFX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STRH_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UDIV(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_mov16(inst: u32) -> arg_ri {
    arg_ri {
        imm : extract_mul2(inst,16,4,0,12),
        rd : extract_simple(inst,12,4),
    }
}

pub fn extract_STRT_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MOV_rxi(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDAH(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDREXB(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STR_rr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_LDAEX(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SADD8(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_EOR_rrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_SDIV(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_CMP_xrrr(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_CRC32H(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_MOVW(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_UADD16(_: u32) -> arg_empty  {
            arg_empty{}
            
}

pub fn extract_STRHT_ri(_: u32) -> arg_empty  {
            arg_empty{}
            
}

// ===== Pattern Group Structure =====

pub enum Instruction {
SMLAWT {args : arg_rrrr },
USUB16 {args : arg_rrr },
ADD_rrrr {args : arg_s_rrr_shr },
TST_xrri {args : arg_s_rrr_shi },
LDRT_ri {args : arg_ldst_ri },
UASX {args : arg_rrr },
SMULBT {args : arg_rrrr },
UDF {args : arg_UDF },
SSAT {args : arg_sat },
QADD16 {args : arg_rrr },
CMP_xrri {args : arg_s_rrr_shi },
QDSUB {args : arg_rrr },
SMLSD {args : arg_rrrr },
MOV_rxri {args : arg_s_rrr_shi },
BKPT {args : arg_i },
UMAAL {args : arg_rrrr },
AND_rri {args : arg_s_rri_rot },
SMMLAR {args : arg_rrrr },
STRH_rr {args : arg_ldst_rr },
SMLALBT {args : arg_rrrr },
STREXH {args : arg_strex },
LDRD_rr {args : arg_ldst_rr },
SMLALD {args : arg_rrrr },
SMMLS {args : arg_rrrr },
MLA {args : arg_s_rrrr },
LDRSHT_rr {args : arg_ldst_rr },
REV {args : arg_rr },
USAT {args : arg_sat },
REV16 {args : arg_rr },
SMLAL {args : arg_s_rrrr },
LDRH_ri {args : arg_ldst_ri },
MUL {args : arg_s_rrrr },
BL {args : arg_i },
RSB_rri {args : arg_s_rri_rot },
LDRSHT_ri {args : arg_ldst_ri },
MRC {args : arg_mcr },
REVSH {args : arg_rr },
SUB_rrrr {args : arg_s_rrr_shr },
BIC_rri {args : arg_s_rri_rot },
STR_ri {args : arg_ldst_ri },
STRHT_ri {args : arg_ldst_ri },
RSC_rrri {args : arg_s_rrr_shi },
MVN_rxrr {args : arg_s_rrr_shr },
STLEXD_a32 {args : arg_strex },
UHASX {args : arg_rrr },
QASX {args : arg_rrr },
LDRBT_rr {args : arg_ldst_rr },
ADC_rrrr {args : arg_s_rrr_shr },
SEL {args : arg_rrr },
MOVT {args : arg_ri },
SMULWT {args : arg_rrrr },
LDRB_rr {args : arg_ldst_rr },
UHADD16 {args : arg_rrr },
LDRHT_ri {args : arg_ldst_ri },
MSR_imm {args : arg_msr_i },
LDAH {args : arg_ldrex },
SADD8 {args : arg_rrr },
SMMLA {args : arg_rrrr },
LDRT_rr {args : arg_ldst_rr },
QDADD {args : arg_rrr },
UQSUB8 {args : arg_rrr },
LDRHT_rr {args : arg_ldst_rr },
EOR_rrrr {args : arg_s_rrr_shr },
SMLAWB {args : arg_rrrr },
SMLSDX {args : arg_rrrr },
SWP {args : arg_swp },
SXTAB16 {args : arg_rrr_rot },
SBC_rrrr {args : arg_s_rrr_shr },
TST_xri {args : arg_s_rri_rot },
SSAX {args : arg_rrr },
ADC_rrri {args : arg_s_rrr_shi },
MOV_rxi {args : arg_s_rri_rot },
STRH_ri {args : arg_ldst_ri },
SWPB {args : arg_swp },
RSB_rrri {args : arg_s_rrr_shi },
STLH {args : arg_ldrex },
SHSAX {args : arg_rrr },
MCR {args : arg_mcr },
SXTAH {args : arg_rrr_rot },
ADC_rri {args : arg_s_rri_rot },
CMN_xrrr {args : arg_s_rrr_shr },
MOVW {args : arg_ri },
LDRD_ri_a32 {args : arg_ldst_ri },
SMLATB {args : arg_rrrr },
CMN_xri {args : arg_s_rri_rot },
BLX_r {args : arg_r },
CRC32CB {args : arg_rrr },
SHASX {args : arg_rrr },
SMLABB {args : arg_rrrr },
STR_rr {args : arg_ldst_rr },
SBFX {args : arg_bfx },
LDREXD_a32 {args : arg_ldrex },
STRD_ri_a32 {args : arg_ldst_ri },
MVN_rxri {args : arg_s_rrr_shi },
TEQ_xrri {args : arg_s_rrr_shi },
LDRB_ri {args : arg_ldst_ri },
HLT {args : arg_i },
LDRSH_ri {args : arg_ldst_ri },
UDIV {args : arg_rrr },
MOV_rxrr {args : arg_s_rrr_shr },
STRBT_rr {args : arg_ldst_rr },
ORR_rrrr {args : arg_s_rrr_shr },
SMLALDX {args : arg_rrrr },
B {args : arg_i },
SMLADX {args : arg_rrrr },
SHADD8 {args : arg_rrr },
MVN_rxi {args : arg_s_rri_rot },
BX {args : arg_r },
SASX {args : arg_rrr },
STRHT_rr {args : arg_ldst_rr },
SHSUB8 {args : arg_rrr },
SADD16 {args : arg_rrr },
LDAEXH {args : arg_ldrex },
UADD8 {args : arg_rrr },
SHADD16 {args : arg_rrr },
QADD8 {args : arg_rrr },
LDAEXD_a32 {args : arg_ldrex },
SBC_rri {args : arg_s_rri_rot },
UQADD16 {args : arg_rrr },
SMLATT {args : arg_rrrr },
MLS {args : arg_rrrr },
SUB_rrri {args : arg_s_rrr_shi },
LDR_rr {args : arg_ldst_rr },
STRT_rr {args : arg_ldst_rr },
SSUB16 {args : arg_rrr },
UHSUB8 {args : arg_rrr },
SSAT16 {args : arg_sat },
CRC32CH {args : arg_rrr },
LDRSB_rr {args : arg_ldst_rr },
STRB_ri {args : arg_ldst_ri },
SMLAD {args : arg_rrrr },
STREX {args : arg_strex },
UHSUB16 {args : arg_rrr },
UQADD8 {args : arg_rrr },
MRRC {args : arg_mcrr },
STRB_rr {args : arg_ldst_rr },
CMP_xri {args : arg_s_rri_rot },
LDRBT_ri {args : arg_ldst_ri },
LDAEXB {args : arg_ldrex },
STLB {args : arg_ldrex },
ADD_rrri {args : arg_s_rrr_shi },
AND_rrrr {args : arg_s_rrr_shr },
UQSAX {args : arg_rrr },
STL {args : arg_ldrex },
QSAX {args : arg_rrr },
USAT16 {args : arg_sat },
USUB8 {args : arg_rrr },
EOR_rri {args : arg_s_rri_rot },
SMULL {args : arg_s_rrrr },
QSUB8 {args : arg_rrr },
STLEXH {args : arg_strex },
SHSUB16 {args : arg_rrr },
LDREXB {args : arg_ldrex },
RSB_rrrr {args : arg_s_rrr_shr },
QADD {args : arg_rrr },
CRC32W {args : arg_rrr },
BIC_rrrr {args : arg_s_rrr_shr },
QSUB {args : arg_rrr },
STRD_rr {args : arg_ldst_rr },
SUB_rri {args : arg_s_rri_rot },
RSC_rri {args : arg_s_rri_rot },
ORR_rri {args : arg_s_rri_rot },
UBFX {args : arg_bfx },
TEQ_xrrr {args : arg_s_rrr_shr },
LDRSH_rr {args : arg_ldst_rr },
LDA {args : arg_ldrex },
SMMLSR {args : arg_rrrr },
LDAB {args : arg_ldrex },
SMULTT {args : arg_rrrr },
UQASX {args : arg_rrr },
SMLSLDX {args : arg_rrrr },
UXTAB {args : arg_rrr_rot },
UMULL {args : arg_s_rrrr },
LDREX {args : arg_ldrex },
SMLALTB {args : arg_rrrr },
CMN_xrri {args : arg_s_rrr_shi },
BIC_rrri {args : arg_s_rrr_shi },
ERET {args : arg_ERET },
UMLAL {args : arg_s_rrrr },
SMLALTT {args : arg_rrrr },
EOR_rrri {args : arg_s_rrr_shi },
ADD_rri {args : arg_s_rri_rot },
CMP_xrrr {args : arg_s_rrr_shr },
SBC_rrri {args : arg_s_rrr_shi },
LDRSBT_rr {args : arg_ldst_rr },
SMLSLD {args : arg_rrrr },
RSC_rrrr {args : arg_s_rrr_shr },
SXTAB {args : arg_rrr_rot },
TEQ_xri {args : arg_s_rri_rot },
USAX {args : arg_rrr },
MCRR {args : arg_mcrr },
SMULTB {args : arg_rrrr },
QSUB16 {args : arg_rrr },
LDRH_rr {args : arg_ldst_rr },
LDR_ri {args : arg_ldst_ri },
CRC32CW {args : arg_rrr },
LDAEX {args : arg_ldrex },
STRT_ri {args : arg_ldst_ri },
CRC32H {args : arg_rrr },
HVC {args : arg_i },
UADD16 {args : arg_rrr },
UHADD8 {args : arg_rrr },
CLZ {args : arg_rr },
ORR_rrri {args : arg_s_rrr_shi },
STREXB {args : arg_strex },
SMLALBB {args : arg_rrrr },
STRBT_ri {args : arg_ldst_ri },
LDRSBT_ri {args : arg_ldst_ri },
AND_rrri {args : arg_s_rrr_shi },
SMULBB {args : arg_rrrr },
STREXD_a32 {args : arg_strex },
LDRSB_ri {args : arg_ldst_ri },
LDREXH {args : arg_ldrex },
STLEX {args : arg_strex },
BXJ {args : arg_r },
UHSAX {args : arg_rrr },
SMLABT {args : arg_rrrr },
STLEXB {args : arg_strex },
UXTAB16 {args : arg_rrr_rot },
UQSUB16 {args : arg_rrr },
CRC32B {args : arg_rrr },
TST_xrrr {args : arg_s_rrr_shr },
RBIT {args : arg_rr },
SDIV {args : arg_rrr },
SSUB8 {args : arg_rrr },
UXTAH {args : arg_rrr_rot },
SMULWB {args : arg_rrrr },
MSR_imm {args : arg_msr_i },
NOP {args : arg_NOP },
ESB {args : arg_ESB },
WFI {args : arg_WFI },
YIELD {args : arg_YIELD },
WFE {args : arg_WFE },
}
// ===== Decoder Function (skeleton) =====

#[derive(Debug, Clone)]

pub fn decode_instruction(inst: u32) -> Option<Instruction> {
// Overlap group - check in order
    // Overlap group - check in order
        // Overlap group - check in order
            // No-overlap group - mutually exclusive patterns
            match inst & 0x0f000000 {
                0x0f000000 => {  // ESB
                    let args = extract_ESB(inst);
                    return Some(Instruction::ESB { args });
                }
                0x0f000000 => {  // WFI
                    let args = extract_WFI(inst);
                    return Some(Instruction::WFI { args });
                }
                0x0f000000 => {  // YIELD
                    let args = extract_YIELD(inst);
                    return Some(Instruction::YIELD { args });
                }
                0x0f000000 => {  // WFE
                    let args = extract_WFE(inst);
                    return Some(Instruction::WFE { args });
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
        let args = extract_msr_i(inst);
        return Some(Instruction::MSR_imm { args });
    }
if (inst & 0x0000c000) == 0x0000c000 {
    // Matched: SMLAWT
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLAWT { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: USUB16
    let args = extract_rndm(inst);
    return Some(Instruction::USUB16 { args });
}
if (inst & 0x00001000) == 0x00001000 {
    // Matched: ADD_rrrr
    let args = extract_s_rrr_shr(inst);
    return Some(Instruction::ADD_rrrr { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: TST_xrri
    let args = extract_S_xrr_shi(inst);
    return Some(Instruction::TST_xrri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: LDRT_ri
    let args = extract_ldst_ri12_p0w1(inst);
    return Some(Instruction::LDRT_ri { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: UASX
    let args = extract_rndm(inst);
    return Some(Instruction::UASX { args });
}
if (inst & 0x000c0000) == 0x000c0000 {
    // Matched: SMULBT
    let args = extract_rd0mn(inst);
    return Some(Instruction::SMULBT { args });
}
if (inst & 0xfe001e00) == 0xfe001e00 {
    // Matched: UDF
    let args = extract_UDF(inst);
    return Some(Instruction::UDF { args });
}
if (inst & 0x08000000) == 0x08000000 {
    // Matched: SSAT
    let args = extract_sat(inst);
    return Some(Instruction::SSAT { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: QADD16
    let args = extract_rndm(inst);
    return Some(Instruction::QADD16 { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: CMP_xrri
    let args = extract_S_xrr_shi(inst);
    return Some(Instruction::CMP_xrri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: QDSUB
    let args = extract_rndm(inst);
    return Some(Instruction::QDSUB { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMLSD
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLSD { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: MOV_rxri
    let args = extract_s_rxr_shi(inst);
    return Some(Instruction::MOV_rxri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: BKPT
    let args = extract_i16(inst);
    return Some(Instruction::BKPT { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: UMAAL
    let args = extract_rdamn(inst);
    return Some(Instruction::UMAAL { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: AND_rri
    let args = extract_s_rri_rot(inst);
    return Some(Instruction::AND_rri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMMLAR
    let args = extract_rdamn(inst);
    return Some(Instruction::SMMLAR { args });
}
if (inst & 0x00080000) == 0x00080000 {
    // Matched: STRH_rr
    let args = extract_ldst_rr_p1w(inst);
    return Some(Instruction::STRH_rr { args });
}
if (inst & 0x0000c000) == 0x0000c000 {
    // Matched: SMLALBT
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLALBT { args });
}
if (inst & 0x0e01f000) == 0x0e01f000 {
    // Matched: STREXH
    let args = extract_strex(inst);
    return Some(Instruction::STREXH { args });
}
if (inst & 0x000c0000) == 0x000c0000 {
    // Matched: LDRD_rr
    let args = extract_ldst_rr_p1w(inst);
    return Some(Instruction::LDRD_rr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMLALD
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLALD { args });
}
if (inst & 0x0000c000) == 0x0000c000 {
    // Matched: SMMLS
    let args = extract_rdamn(inst);
    return Some(Instruction::SMMLS { args });
}
if (inst & 0x00004000) == 0x00004000 {
    // Matched: MLA
    let args = extract_s_rdamn(inst);
    return Some(Instruction::MLA { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: LDRSHT_rr
    let args = extract_ldst_rr_p0w1(inst);
    return Some(Instruction::LDRSHT_rr { args });
}
if (inst & 0x0f878000) == 0x0f878000 {
    // Matched: REV
    let args = extract_rdm(inst);
    return Some(Instruction::REV { args });
}
if (inst & 0x0e000000) == 0x0e000000 {
    // Matched: USAT
    let args = extract_sat(inst);
    return Some(Instruction::USAT { args });
}
if (inst & 0x0f87c000) == 0x0f87c000 {
    // Matched: REV16
    let args = extract_rdm(inst);
    return Some(Instruction::REV16 { args });
}
if (inst & 0x0e000800) == 0x0e000800 {
    // Matched: SMLAL
    let args = extract_s_rdamn(inst);
    return Some(Instruction::SMLAL { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: LDRH_ri
    let args = extract_ldst_ri8_p1w(inst);
    return Some(Instruction::LDRH_ri { args });
}
if (inst & 0x00040000) == 0x00040000 {
    // Matched: MUL
    let args = extract_s_rd0mn(inst);
    return Some(Instruction::MUL { args });
}
if (inst & 0x08000000) == 0x08000000 {
    // Matched: BL
    let args = extract_branch(inst);
    return Some(Instruction::BL { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: RSB_rri
    let args = extract_s_rri_rot(inst);
    return Some(Instruction::RSB_rri { args });
}
if (inst & 0x0000f000) == 0x0000f000 {
    // Matched: LDRSHT_ri
    let args = extract_ldst_ri8_p0w1(inst);
    return Some(Instruction::LDRSHT_ri { args });
}
if (inst & 0x0e200020) == 0x0e200020 {
    // Matched: MRC
    let args = extract_mcr(inst);
    return Some(Instruction::MRC { args });
}
if (inst & 0x0ff0f800) == 0x0ff0f800 {
    // Matched: REVSH
    let args = extract_rdm(inst);
    return Some(Instruction::REVSH { args });
}
if (inst & 0x00001000) == 0x00001000 {
    // Matched: SUB_rrrr
    let args = extract_s_rrr_shr(inst);
    return Some(Instruction::SUB_rrrr { args });
}
if (inst & 0x0e000000) == 0x0e000000 {
    // Matched: BIC_rri
    let args = extract_s_rri_rot(inst);
    return Some(Instruction::BIC_rri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: STR_ri
    let args = extract_ldst_ri12_pw0(inst);
    return Some(Instruction::STR_ri { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: STRHT_ri
    let args = extract_ldst_ri8_p0w1(inst);
    return Some(Instruction::STRHT_ri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: RSC_rrri
    let args = extract_s_rrr_shi(inst);
    return Some(Instruction::RSC_rrri { args });
}
if (inst & 0x0f001000) == 0x0f001000 {
    // Matched: MVN_rxrr
    let args = extract_s_rxr_shr(inst);
    return Some(Instruction::MVN_rxrr { args });
}
if (inst & 0x08078000) == 0x08078000 {
    // Matched: STLEXD_a32
    let args = extract_strex(inst);
    return Some(Instruction::STLEXD_a32 { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: UHASX
    let args = extract_rndm(inst);
    return Some(Instruction::UHASX { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: QASX
    let args = extract_rndm(inst);
    return Some(Instruction::QASX { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: LDRBT_rr
    let args = extract_ldst_rs_p0w1(inst);
    return Some(Instruction::LDRBT_rr { args });
}
if (inst & 0x00001000) == 0x00001000 {
    // Matched: ADC_rrrr
    let args = extract_s_rrr_shr(inst);
    return Some(Instruction::ADC_rrrr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMC
    let args = extract_i(inst);
    return Some(Instruction::SMC { args });
}
if (inst & 0x0807c000) == 0x0807c000 {
    // Matched: SEL
    let args = extract_rndm(inst);
    return Some(Instruction::SEL { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: MOVT
    let args = extract_mov16(inst);
    return Some(Instruction::MOVT { args });
}
if (inst & 0x000e0000) == 0x000e0000 {
    // Matched: SMULWT
    let args = extract_rd0mn(inst);
    return Some(Instruction::SMULWT { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: LDRB_rr
    let args = extract_ldst_rs_p1w(inst);
    return Some(Instruction::LDRB_rr { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: UHADD16
    let args = extract_rndm(inst);
    return Some(Instruction::UHADD16 { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: LDRHT_ri
    let args = extract_ldst_ri8_p0w1(inst);
    return Some(Instruction::LDRHT_ri { args });
}
if (inst & 0x00f00000) == 0x00f00000 {
    // Matched: MSR_imm
    let args = extract_msr_i(inst);
    return Some(Instruction::MSR_imm { args });
}
if (inst & 0x0f00fe00) == 0x0f00fe00 {
    // Matched: LDAH
    let args = extract_ldrex(inst);
    return Some(Instruction::LDAH { args });
}
if (inst & 0x000f8000) == 0x000f8000 {
    // Matched: SADD8
    let args = extract_rndm(inst);
    return Some(Instruction::SADD8 { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMMLA
    let args = extract_rdamn(inst);
    return Some(Instruction::SMMLA { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: BFCI
    let args = extract_bfi(inst);
    return Some(Instruction::BFCI { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: LDRT_rr
    let args = extract_ldst_rs_p0w1(inst);
    return Some(Instruction::LDRT_rr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: QDADD
    let args = extract_rndm(inst);
    return Some(Instruction::QDADD { args });
}
if (inst & 0x000ff000) == 0x000ff000 {
    // Matched: UQSUB8
    let args = extract_rndm(inst);
    return Some(Instruction::UQSUB8 { args });
}
if (inst & 0x00080000) == 0x00080000 {
    // Matched: LDRHT_rr
    let args = extract_ldst_rr_p0w1(inst);
    return Some(Instruction::LDRHT_rr { args });
}
if (inst & 0x00001000) == 0x00001000 {
    // Matched: EOR_rrrr
    let args = extract_s_rrr_shr(inst);
    return Some(Instruction::EOR_rrrr { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: SMLAWB
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLAWB { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMLSDX
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLSDX { args });
}
if (inst & 0x00080000) == 0x00080000 {
    // Matched: SWP
    let args = extract_swp(inst);
    return Some(Instruction::SWP { args });
}
if (inst & 0x08000000) == 0x08000000 {
    // Matched: SXTAB16
    let args = extract_rrr_rot(inst);
    return Some(Instruction::SXTAB16 { args });
}
if (inst & 0x00001000) == 0x00001000 {
    // Matched: SBC_rrrr
    let args = extract_s_rrr_shr(inst);
    return Some(Instruction::SBC_rrrr { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: TST_xri
    let args = extract_S_xri_rot(inst);
    return Some(Instruction::TST_xri { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: SSAX
    let args = extract_rndm(inst);
    return Some(Instruction::SSAX { args });
}
if (inst & 0x08000000) == 0x08000000 {
    // Matched: PKH
    let args = extract_pkh(inst);
    return Some(Instruction::PKH { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: ADC_rrri
    let args = extract_s_rrr_shi(inst);
    return Some(Instruction::ADC_rrri { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: MOV_rxi
    let args = extract_s_rxi_rot(inst);
    return Some(Instruction::MOV_rxi { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: STRH_ri
    let args = extract_ldst_ri8_p1w(inst);
    return Some(Instruction::STRH_ri { args });
}
if (inst & 0x00080000) == 0x00080000 {
    // Matched: SWPB
    let args = extract_swp(inst);
    return Some(Instruction::SWPB { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: RSB_rrri
    let args = extract_s_rrr_shi(inst);
    return Some(Instruction::RSB_rrri { args });
}
if (inst & 0x0e1fc000) == 0x0e1fc000 {
    // Matched: STLH
    let args = extract_stl(inst);
    return Some(Instruction::STLH { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: SHSAX
    let args = extract_rndm(inst);
    return Some(Instruction::SHSAX { args });
}
if (inst & 0x0e000040) == 0x0e000040 {
    // Matched: MCR
    let args = extract_mcr(inst);
    return Some(Instruction::MCR { args });
}
if (inst & 0x08000000) == 0x08000000 {
    // Matched: SXTAH
    let args = extract_rrr_rot(inst);
    return Some(Instruction::SXTAH { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: ADC_rri
    let args = extract_s_rri_rot(inst);
    return Some(Instruction::ADC_rri { args });
}
if (inst & 0x0c008000) == 0x0c008000 {
    // Matched: CMN_xrrr
    let args = extract_S_xrr_shr(inst);
    return Some(Instruction::CMN_xrrr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: MOVW
    let args = extract_mov16(inst);
    return Some(Instruction::MOVW { args });
}
if (inst & 0x0000c000) == 0x0000c000 {
    // Matched: LDRD_ri_a32
    let args = extract_ldst_ri8_p1w(inst);
    return Some(Instruction::LDRD_ri_a32 { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: SMLATB
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLATB { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: CMN_xri
    let args = extract_S_xri_rot(inst);
    return Some(Instruction::CMN_xri { args });
}
if (inst & 0x0fff0000) == 0x0fff0000 {
    // Matched: BLX_r
    let args = extract_rm(inst);
    return Some(Instruction::BLX_r { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: CRC32CB
    let args = extract_rndm(inst);
    return Some(Instruction::CRC32CB { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: SHASX
    let args = extract_rndm(inst);
    return Some(Instruction::SHASX { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: SMLABB
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLABB { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: STR_rr
    let args = extract_ldst_rs_p1w(inst);
    return Some(Instruction::STR_rr { args });
}
if (inst & 0x08001000) == 0x08001000 {
    // Matched: SBFX
    let args = extract_bfx(inst);
    return Some(Instruction::SBFX { args });
}
if (inst & 0x07800000) == 0x07800000 {
    // Matched: MRS_reg
    let args = extract_mrs_reg(inst);
    return Some(Instruction::MRS_reg { args });
}
if (inst & 0x0807fc00) == 0x0807fc00 {
    // Matched: LDREXD_a32
    let args = extract_ldrex(inst);
    return Some(Instruction::LDREXD_a32 { args });
}
if (inst & 0x0000f000) == 0x0000f000 {
    // Matched: STRD_ri_a32
    let args = extract_ldst_ri8_p1w(inst);
    return Some(Instruction::STRD_ri_a32 { args });
}
if (inst & 0x0f000000) == 0x0f000000 {
    // Matched: MVN_rxri
    let args = extract_s_rxr_shi(inst);
    return Some(Instruction::MVN_rxri { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: TEQ_xrri
    let args = extract_S_xrr_shi(inst);
    return Some(Instruction::TEQ_xrri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: LDRB_ri
    let args = extract_ldst_ri12_pw0(inst);
    return Some(Instruction::LDRB_ri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: HLT
    let args = extract_i16(inst);
    return Some(Instruction::HLT { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: MRS_bank
    let args = extract_mrs_bank(inst);
    return Some(Instruction::MRS_bank { args });
}
if (inst & 0x08000000) == 0x08000000 {
    // Matched: USADA8
    let args = extract_USADA8(inst);
    return Some(Instruction::USADA8 { args });
}
if (inst & 0x0000f000) == 0x0000f000 {
    // Matched: LDRSH_ri
    let args = extract_ldst_ri8_p1w(inst);
    return Some(Instruction::LDRSH_ri { args });
}
if (inst & 0x00f00000) == 0x00f00000 {
    // Matched: UDIV
    let args = extract_rdmn(inst);
    return Some(Instruction::UDIV { args });
}
if (inst & 0x0c004000) == 0x0c004000 {
    // Matched: MOV_rxrr
    let args = extract_s_rxr_shr(inst);
    return Some(Instruction::MOV_rxrr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: STRBT_rr
    let args = extract_ldst_rs_p0w1(inst);
    return Some(Instruction::STRBT_rr { args });
}
if (inst & 0x0c000400) == 0x0c000400 {
    // Matched: ORR_rrrr
    let args = extract_s_rrr_shr(inst);
    return Some(Instruction::ORR_rrrr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMLALDX
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLALDX { args });
}
if (inst & 0x08000000) == 0x08000000 {
    // Matched: B
    let args = extract_branch(inst);
    return Some(Instruction::B { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMLADX
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLADX { args });
}
if (inst & 0x000f8000) == 0x000f8000 {
    // Matched: SHADD8
    let args = extract_rndm(inst);
    return Some(Instruction::SHADD8 { args });
}
if (inst & 0x08000000) == 0x08000000 {
    // Matched: STM
    let args = extract_ldst_block(inst);
    return Some(Instruction::STM { args });
}
if (inst & 0x043c0000) == 0x043c0000 {
    // Matched: MSR_reg
    let args = extract_msr_reg(inst);
    return Some(Instruction::MSR_reg { args });
}
if (inst & 0x0f000000) == 0x0f000000 {
    // Matched: MVN_rxi
    let args = extract_s_rxi_rot(inst);
    return Some(Instruction::MVN_rxi { args });
}
if (inst & 0x0fff0000) == 0x0fff0000 {
    // Matched: BX
    let args = extract_rm(inst);
    return Some(Instruction::BX { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: SASX
    let args = extract_rndm(inst);
    return Some(Instruction::SASX { args });
}
if (inst & 0x00080000) == 0x00080000 {
    // Matched: STRHT_rr
    let args = extract_ldst_rr_p0w1(inst);
    return Some(Instruction::STRHT_rr { args });
}
if (inst & 0x000ff000) == 0x000ff000 {
    // Matched: SHSUB8
    let args = extract_rndm(inst);
    return Some(Instruction::SHSUB8 { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: SADD16
    let args = extract_rndm(inst);
    return Some(Instruction::SADD16 { args });
}
if (inst & 0x0f00ff00) == 0x0f00ff00 {
    // Matched: LDAEXH
    let args = extract_ldrex(inst);
    return Some(Instruction::LDAEXH { args });
}
if (inst & 0x000f8000) == 0x000f8000 {
    // Matched: UADD8
    let args = extract_rndm(inst);
    return Some(Instruction::UADD8 { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: SHADD16
    let args = extract_rndm(inst);
    return Some(Instruction::SHADD16 { args });
}
if (inst & 0x000f8000) == 0x000f8000 {
    // Matched: QADD8
    let args = extract_rndm(inst);
    return Some(Instruction::QADD8 { args });
}
if (inst & 0x0807f800) == 0x0807f800 {
    // Matched: LDAEXD_a32
    let args = extract_ldrex(inst);
    return Some(Instruction::LDAEXD_a32 { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SBC_rri
    let args = extract_s_rri_rot(inst);
    return Some(Instruction::SBC_rri { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: UQADD16
    let args = extract_rndm(inst);
    return Some(Instruction::UQADD16 { args });
}
if (inst & 0x0000e000) == 0x0000e000 {
    // Matched: SMLATT
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLATT { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: MLS
    let args = extract_rdamn(inst);
    return Some(Instruction::MLS { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SUB_rrri
    let args = extract_s_rrr_shi(inst);
    return Some(Instruction::SUB_rrri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: LDR_rr
    let args = extract_ldst_rs_p1w(inst);
    return Some(Instruction::LDR_rr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: STRT_rr
    let args = extract_ldst_rs_p0w1(inst);
    return Some(Instruction::STRT_rr { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: SSUB16
    let args = extract_rndm(inst);
    return Some(Instruction::SSUB16 { args });
}
if (inst & 0x000ff000) == 0x000ff000 {
    // Matched: UHSUB8
    let args = extract_rndm(inst);
    return Some(Instruction::UHSUB8 { args });
}
if (inst & 0x08078000) == 0x08078000 {
    // Matched: SSAT16
    let args = extract_sat16(inst);
    return Some(Instruction::SSAT16 { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: CRC32CH
    let args = extract_rndm(inst);
    return Some(Instruction::CRC32CH { args });
}
if (inst & 0x000c0000) == 0x000c0000 {
    // Matched: LDRSB_rr
    let args = extract_ldst_rr_p1w(inst);
    return Some(Instruction::LDRSB_rr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: STRB_ri
    let args = extract_ldst_ri12_pw0(inst);
    return Some(Instruction::STRB_ri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMLAD
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLAD { args });
}
if (inst & 0x0807c000) == 0x0807c000 {
    // Matched: STREX
    let args = extract_strex(inst);
    return Some(Instruction::STREX { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: UHSUB16
    let args = extract_rndm(inst);
    return Some(Instruction::UHSUB16 { args });
}
if (inst & 0x000f8000) == 0x000f8000 {
    // Matched: UQADD8
    let args = extract_rndm(inst);
    return Some(Instruction::UQADD8 { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: MRRC
    let args = extract_mcrr(inst);
    return Some(Instruction::MRRC { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: STRB_rr
    let args = extract_ldst_rs_p1w(inst);
    return Some(Instruction::STRB_rr { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: CMP_xri
    let args = extract_S_xri_rot(inst);
    return Some(Instruction::CMP_xri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: LDRBT_ri
    let args = extract_ldst_ri12_p0w1(inst);
    return Some(Instruction::LDRBT_ri { args });
}
if (inst & 0x0c03fc00) == 0x0c03fc00 {
    // Matched: LDAEXB
    let args = extract_ldrex(inst);
    return Some(Instruction::LDAEXB { args });
}
if (inst & 0x0c3f8000) == 0x0c3f8000 {
    // Matched: STLB
    let args = extract_stl(inst);
    return Some(Instruction::STLB { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: ADD_rrri
    let args = extract_s_rrr_shi(inst);
    return Some(Instruction::ADD_rrri { args });
}
if (inst & 0x00001000) == 0x00001000 {
    // Matched: AND_rrrr
    let args = extract_s_rrr_shr(inst);
    return Some(Instruction::AND_rrrr { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: UQSAX
    let args = extract_rndm(inst);
    return Some(Instruction::UQSAX { args });
}
if (inst & 0x087f0000) == 0x087f0000 {
    // Matched: STL
    let args = extract_stl(inst);
    return Some(Instruction::STL { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: QSAX
    let args = extract_rndm(inst);
    return Some(Instruction::QSAX { args });
}
if (inst & 0x0e01e000) == 0x0e01e000 {
    // Matched: USAT16
    let args = extract_sat16(inst);
    return Some(Instruction::USAT16 { args });
}
if (inst & 0x000ff000) == 0x000ff000 {
    // Matched: USUB8
    let args = extract_rndm(inst);
    return Some(Instruction::USUB8 { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: EOR_rri
    let args = extract_s_rri_rot(inst);
    return Some(Instruction::EOR_rri { args });
}
if (inst & 0x0c001000) == 0x0c001000 {
    // Matched: SMULL
    let args = extract_s_rdamn(inst);
    return Some(Instruction::SMULL { args });
}
if (inst & 0x000ff000) == 0x000ff000 {
    // Matched: QSUB8
    let args = extract_rndm(inst);
    return Some(Instruction::QSUB8 { args });
}
if (inst & 0x0e01e000) == 0x0e01e000 {
    // Matched: STLEXH
    let args = extract_strex(inst);
    return Some(Instruction::STLEXH { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: SHSUB16
    let args = extract_rndm(inst);
    return Some(Instruction::SHSUB16 { args });
}
if (inst & 0x0c03fe00) == 0x0c03fe00 {
    // Matched: LDREXB
    let args = extract_ldrex(inst);
    return Some(Instruction::LDREXB { args });
}
if (inst & 0x00001000) == 0x00001000 {
    // Matched: RSB_rrrr
    let args = extract_s_rrr_shr(inst);
    return Some(Instruction::RSB_rrrr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: QADD
    let args = extract_rndm(inst);
    return Some(Instruction::QADD { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: CRC32W
    let args = extract_rndm(inst);
    return Some(Instruction::CRC32W { args });
}
if (inst & 0x0e000200) == 0x0e000200 {
    // Matched: BIC_rrrr
    let args = extract_s_rrr_shr(inst);
    return Some(Instruction::BIC_rrrr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: QSUB
    let args = extract_rndm(inst);
    return Some(Instruction::QSUB { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: STRD_rr
    let args = extract_ldst_rr_p1w(inst);
    return Some(Instruction::STRD_rr { args });
}
if (inst & 0x0f000000) == 0x0f000000 {
    // Matched: SVC
    let args = extract_i(inst);
    return Some(Instruction::SVC { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SUB_rri
    let args = extract_s_rri_rot(inst);
    return Some(Instruction::SUB_rri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: RSC_rri
    let args = extract_s_rri_rot(inst);
    return Some(Instruction::RSC_rri { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: ORR_rri
    let args = extract_s_rri_rot(inst);
    return Some(Instruction::ORR_rri { args });
}
if (inst & 0x0e000400) == 0x0e000400 {
    // Matched: UBFX
    let args = extract_bfx(inst);
    return Some(Instruction::UBFX { args });
}
if (inst & 0x0c008000) == 0x0c008000 {
    // Matched: TEQ_xrrr
    let args = extract_S_xrr_shr(inst);
    return Some(Instruction::TEQ_xrrr { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: LDRSH_rr
    let args = extract_ldst_rr_p1w(inst);
    return Some(Instruction::LDRSH_rr { args });
}
if (inst & 0x0807f000) == 0x0807f000 {
    // Matched: LDA
    let args = extract_ldrex(inst);
    return Some(Instruction::LDA { args });
}
if (inst & 0x0000f000) == 0x0000f000 {
    // Matched: SMMLSR
    let args = extract_rdamn(inst);
    return Some(Instruction::SMMLSR { args });
}
if (inst & 0x0c03f800) == 0x0c03f800 {
    // Matched: LDAB
    let args = extract_ldrex(inst);
    return Some(Instruction::LDAB { args });
}
if (inst & 0x000e0000) == 0x000e0000 {
    // Matched: SMULTT
    let args = extract_rd0mn(inst);
    return Some(Instruction::SMULTT { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: UQASX
    let args = extract_rndm(inst);
    return Some(Instruction::UQASX { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMLSLDX
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLSLDX { args });
}
if (inst & 0x0e000000) == 0x0e000000 {
    // Matched: UXTAB
    let args = extract_rrr_rot(inst);
    return Some(Instruction::UXTAB { args });
}
if (inst & 0x08002000) == 0x08002000 {
    // Matched: UMULL
    let args = extract_s_rdamn(inst);
    return Some(Instruction::UMULL { args });
}
if (inst & 0x0807fc00) == 0x0807fc00 {
    // Matched: LDREX
    let args = extract_ldrex(inst);
    return Some(Instruction::LDREX { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: SMLALTB
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLALTB { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: CMN_xrri
    let args = extract_S_xrr_shi(inst);
    return Some(Instruction::CMN_xrri { args });
}
if (inst & 0x0e000000) == 0x0e000000 {
    // Matched: BIC_rrri
    let args = extract_s_rrr_shi(inst);
    return Some(Instruction::BIC_rrri { args });
}
if (inst & 0x0e000000) == 0x0e000000 {
    // Matched: ERET
    let args = extract_ERET(inst);
    return Some(Instruction::ERET { args });
}
if (inst & 0x08002000) == 0x08002000 {
    // Matched: UMLAL
    let args = extract_s_rdamn(inst);
    return Some(Instruction::UMLAL { args });
}
if (inst & 0x0000e000) == 0x0000e000 {
    // Matched: SMLALTT
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLALTT { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: EOR_rrri
    let args = extract_s_rrr_shi(inst);
    return Some(Instruction::EOR_rrri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: ADD_rri
    let args = extract_s_rri_rot(inst);
    return Some(Instruction::ADD_rri { args });
}
if (inst & 0x0c008000) == 0x0c008000 {
    // Matched: CMP_xrrr
    let args = extract_S_xrr_shr(inst);
    return Some(Instruction::CMP_xrrr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SBC_rrri
    let args = extract_s_rrr_shi(inst);
    return Some(Instruction::SBC_rrri { args });
}
if (inst & 0x000c0000) == 0x000c0000 {
    // Matched: LDRSBT_rr
    let args = extract_ldst_rr_p0w1(inst);
    return Some(Instruction::LDRSBT_rr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: SMLSLD
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLSLD { args });
}
if (inst & 0x00001000) == 0x00001000 {
    // Matched: RSC_rrrr
    let args = extract_s_rrr_shr(inst);
    return Some(Instruction::RSC_rrrr { args });
}
if (inst & 0x08000000) == 0x08000000 {
    // Matched: SXTAB
    let args = extract_rrr_rot(inst);
    return Some(Instruction::SXTAB { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: TEQ_xri
    let args = extract_S_xri_rot(inst);
    return Some(Instruction::TEQ_xri { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: USAX
    let args = extract_rndm(inst);
    return Some(Instruction::USAX { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: MCRR
    let args = extract_mcrr(inst);
    return Some(Instruction::MCRR { args });
}
if (inst & 0x00080000) == 0x00080000 {
    // Matched: SMULTB
    let args = extract_rd0mn(inst);
    return Some(Instruction::SMULTB { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: QSUB16
    let args = extract_rndm(inst);
    return Some(Instruction::QSUB16 { args });
}
if (inst & 0x00080000) == 0x00080000 {
    // Matched: LDRH_rr
    let args = extract_ldst_rr_p1w(inst);
    return Some(Instruction::LDRH_rr { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: LDR_ri
    let args = extract_ldst_ri12_pw0(inst);
    return Some(Instruction::LDR_ri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: CRC32CW
    let args = extract_rndm(inst);
    return Some(Instruction::CRC32CW { args });
}
if (inst & 0x0807f800) == 0x0807f800 {
    // Matched: LDAEX
    let args = extract_ldrex(inst);
    return Some(Instruction::LDAEX { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: STRT_ri
    let args = extract_ldst_ri12_p0w1(inst);
    return Some(Instruction::STRT_ri { args });
}
if (inst & 0x043c0000) == 0x043c0000 {
    // Matched: MSR_bank
    let args = extract_msr_bank(inst);
    return Some(Instruction::MSR_bank { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: CRC32H
    let args = extract_rndm(inst);
    return Some(Instruction::CRC32H { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: HVC
    let args = extract_i16(inst);
    return Some(Instruction::HVC { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: UADD16
    let args = extract_rndm(inst);
    return Some(Instruction::UADD16 { args });
}
if (inst & 0x000f8000) == 0x000f8000 {
    // Matched: UHADD8
    let args = extract_rndm(inst);
    return Some(Instruction::UHADD8 { args });
}
if (inst & 0x0f0f0000) == 0x0f0f0000 {
    // Matched: CLZ
    let args = extract_rdm(inst);
    return Some(Instruction::CLZ { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: ORR_rrri
    let args = extract_s_rrr_shi(inst);
    return Some(Instruction::ORR_rrri { args });
}
if (inst & 0x0c03e000) == 0x0c03e000 {
    // Matched: STREXB
    let args = extract_strex(inst);
    return Some(Instruction::STREXB { args });
}
if (inst & 0x00008000) == 0x00008000 {
    // Matched: SMLALBB
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLALBB { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: STRBT_ri
    let args = extract_ldst_ri12_p0w1(inst);
    return Some(Instruction::STRBT_ri { args });
}
if (inst & 0x0000c000) == 0x0000c000 {
    // Matched: LDRSBT_ri
    let args = extract_ldst_ri8_p0w1(inst);
    return Some(Instruction::LDRSBT_ri { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: AND_rrri
    let args = extract_s_rrr_shi(inst);
    return Some(Instruction::AND_rrri { args });
}
if (inst & 0x00080000) == 0x00080000 {
    // Matched: SMULBB
    let args = extract_rd0mn(inst);
    return Some(Instruction::SMULBB { args });
}
if (inst & 0x0807c000) == 0x0807c000 {
    // Matched: STREXD_a32
    let args = extract_strex(inst);
    return Some(Instruction::STREXD_a32 { args });
}
if (inst & 0x0000c000) == 0x0000c000 {
    // Matched: LDRSB_ri
    let args = extract_ldst_ri8_p1w(inst);
    return Some(Instruction::LDRSB_ri { args });
}
if (inst & 0x0f00ff80) == 0x0f00ff80 {
    // Matched: LDREXH
    let args = extract_ldrex(inst);
    return Some(Instruction::LDREXH { args });
}
if (inst & 0x08078000) == 0x08078000 {
    // Matched: STLEX
    let args = extract_strex(inst);
    return Some(Instruction::STLEX { args });
}
if (inst & 0x0fff0000) == 0x0fff0000 {
    // Matched: BXJ
    let args = extract_rm(inst);
    return Some(Instruction::BXJ { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: UHSAX
    let args = extract_rndm(inst);
    return Some(Instruction::UHSAX { args });
}
if (inst & 0x08400000) == 0x08400000 {
    // Matched: LDM_a32
    let args = extract_ldst_block(inst);
    return Some(Instruction::LDM_a32 { args });
}
if (inst & 0x0000c000) == 0x0000c000 {
    // Matched: SMLABT
    let args = extract_rdamn(inst);
    return Some(Instruction::SMLABT { args });
}
if (inst & 0x0c03c000) == 0x0c03c000 {
    // Matched: STLEXB
    let args = extract_strex(inst);
    return Some(Instruction::STLEXB { args });
}
if (inst & 0x0c000000) == 0x0c000000 {
    // Matched: UXTAB16
    let args = extract_rrr_rot(inst);
    return Some(Instruction::UXTAB16 { args });
}
if (inst & 0x000f0000) == 0x000f0000 {
    // Matched: UQSUB16
    let args = extract_rndm(inst);
    return Some(Instruction::UQSUB16 { args });
}
if (inst & 0x00000000) == 0x00000000 {
    // Matched: CRC32B
    let args = extract_rndm(inst);
    return Some(Instruction::CRC32B { args });
}
if (inst & 0x0c008000) == 0x0c008000 {
    // Matched: TST_xrrr
    let args = extract_S_xrr_shr(inst);
    return Some(Instruction::TST_xrrr { args });
}
if (inst & 0x0ff0f000) == 0x0ff0f000 {
    // Matched: RBIT
    let args = extract_rdm(inst);
    return Some(Instruction::RBIT { args });
}
if (inst & 0x00f00000) == 0x00f00000 {
    // Matched: SDIV
    let args = extract_rdmn(inst);
    return Some(Instruction::SDIV { args });
}
if (inst & 0x000ff000) == 0x000ff000 {
    // Matched: SSUB8
    let args = extract_rndm(inst);
    return Some(Instruction::SSUB8 { args });
}
if (inst & 0x0f000000) == 0x0f000000 {
    // Matched: UXTAH
    let args = extract_rrr_rot(inst);
    return Some(Instruction::UXTAH { args });
}
if (inst & 0x00080000) == 0x00080000 {
    // Matched: SMULWB
    let args = extract_rd0mn(inst);
    return Some(Instruction::SMULWB { args });
}
    None
}
