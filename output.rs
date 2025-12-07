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
 fn extract_mul(inst:u32,len1:u32,pos1:u32,len2:u32,pos2:u32) -> u32 {
    // mask = (1 << len) - 1
    let mask1 = (1u32 << len1) - 1;
    let mask2 = (1u32 << len2) - 1;

    let field1 = (inst >> pos1) & mask1;
    let field2 = (inst >> pos2) & mask2;

    // concatenate field1 (lower bits) and field2 (upper bits)
    field1 | (field2 << len1)
}
pub struct ldrex {
    pub rn: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
}
pub struct rr {
    pub rd: u32,
    pub rm: u32,
}
pub struct bfx {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub widthm1: u32,
}
pub struct mrs_bank {
    pub rd: u32,
    pub r: u32,
    pub sysm: u32,
}
pub struct msr_reg {
    pub rn: u32,
    pub r: u32,
    pub mask: u32,
}
pub struct ldst_block {
    pub rn: u32,
    pub i: u32,
    pub b: u32,
    pub u: u32,
    pub w: u32,
    pub list: u32,
}
pub struct sat {
    pub rd: u32,
    pub rn: u32,
    pub satimm: u32,
    pub imm: u32,
    pub sh: u32,
}
pub struct i {
    pub imm: u32,
}
pub struct rrr_rot {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub rot: u32,
}
pub struct empty {
}
pub struct s_rrr_shi {
    pub s: u32,
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub shim: u32,
    pub shty: u32,
}
pub struct rrrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}
pub struct s_rrrr {
    pub s: u32,
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}
pub struct ri {
    pub rd: u32,
    pub imm: u32,
}
pub struct strex {
    pub rn: u32,
    pub rd: u32,
    pub rt: u32,
    pub rt2: u32,
    pub imm: u32,
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
pub struct bfi {
    pub rd: u32,
    pub rn: u32,
    pub lsb: u32,
    pub msb: u32,
}
pub struct pkh {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub imm: u32,
    pub tb: u32,
}
pub struct s_rrr_shr {
    pub s: u32,
    pub rn: u32,
    pub rd: u32,
    pub rm: u32,
    pub rs: u32,
    pub shty: u32,
}
pub struct s_rri_rot {
    pub s: u32,
    pub rn: u32,
    pub rd: u32,
    pub imm: u32,
    pub rot: u32,
}
pub struct ldst_ri {
    pub p: u32,
    pub w: u32,
    pub u: u32,
    pub rn: u32,
    pub rt: u32,
    pub imm: u32,
}
pub struct mcr {
    pub cp: u32,
    pub opc1: u32,
    pub crn: u32,
    pub crm: u32,
    pub opc2: u32,
    pub rt: u32,
}
pub struct mcrr {
    pub cp: u32,
    pub opc1: u32,
    pub crm: u32,
    pub rt: u32,
    pub rt2: u32,
}
pub struct msr_i {
    pub r: u32,
    pub mask: u32,
    pub rot: u32,
    pub imm: u32,
}
pub struct r {
    pub rm: u32,
}
pub struct mrs_reg {
    pub rd: u32,
    pub r: u32,
}
pub struct rrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
}
pub struct msr_bank {
    pub rn: u32,
    pub r: u32,
    pub sysm: u32,
}
pub fn extract_msr_i(inst:u32)->msr_i{
msr_i { 
rot : extract_simple(inst,4,8),
imm : extract_simple(inst,8,0),
mask : extract_simple(inst,4,16),
}}
 
pub fn extract_s_rxi_rot(inst:u32)->s_rri_rot{
s_rri_rot { 
rn:0,
s : extract_simple(inst,1,20),
rd : extract_simple(inst,4,12),
imm : extract_simple(inst,8,0),
rot : extract_mul(inst,8 ,4),
}}
 
pub fn extract_rdamn(inst:u32)->rrrr{
rrrr { 
rd : extract_simple(inst,4,16),
ra : extract_simple(inst,4,12),
rn : extract_simple(inst,4,0),
rm : extract_simple(inst,4,8),
}}
 
pub fn extract_s_rd0mn(inst:u32)->s_rrrr{
s_rrrr { 
rd : extract_simple(inst,4,16),
rm : extract_simple(inst,4,8),
ra:0,
s : extract_simple(inst,1,20),
rn : extract_simple(inst,4,0),
}}
 
pub fn extract_s_rxr_shr(inst:u32)->s_rrr_shr{
s_rrr_shr { 
rn:0,
rs : extract_simple(inst,4,8),
s : extract_simple(inst,1,20),
shty : extract_simple(inst,2,5),
rd : extract_simple(inst,4,12),
rm : extract_simple(inst,4,0),
}}
 
pub fn extract_s_rri_rot(inst:u32)->s_rri_rot{
s_rri_rot { 
rd : extract_simple(inst,4,12),
rn : extract_simple(inst,4,16),
s : extract_simple(inst,1,20),
imm : extract_simple(inst,8,0),
rot : extract_mul(inst,8 ,4),
}}
 
pub fn extract_ldrex(inst:u32)->ldrex{
ldrex { 
rt2:15,
imm:0,
rn : extract_simple(inst,4,16),
rt : extract_simple(inst,4,12),
}}
 
pub fn extract_stl(inst:u32)->ldrex{
ldrex { 
imm:0,
rn : extract_simple(inst,4,16),
rt2:15,
rt : extract_simple(inst,4,0),
}}
 
pub fn extract_rdmn(inst:u32)->rrr{
rrr { 
rm : extract_simple(inst,4,8),
rd : extract_simple(inst,4,16),
rn : extract_simple(inst,4,0),
}}
 
pub fn extract_ldst_ri12_pw0(inst:u32)->ldst_ri{
ldst_ri { 
p:0,
w:0,
rt : extract_simple(inst,4,12),
imm : extract_simple(inst,12,0),
u : extract_simple(inst,1,23),
rn : extract_simple(inst,4,16),
}}
 
pub fn extract_ldst_rs_pw0(inst:u32)->ldst_rr{
ldst_rr { 
rn : extract_simple(inst,4,16),
p:0,
u : extract_simple(inst,1,23),
rm : extract_simple(inst,4,0),
w:0,
rt : extract_simple(inst,4,12),
shtype : extract_simple(inst,2,5),
shimm : extract_simple(inst,5,7),
}}
 
pub fn extract_ldst_rs_p0w1(inst:u32)->ldst_rr{
ldst_rr { 
u : extract_simple(inst,1,23),
shimm : extract_simple(inst,5,7),
rm : extract_simple(inst,4,0),
p:0,
w:0,
rt : extract_simple(inst,4,12),
shtype : extract_simple(inst,2,5),
rn : extract_simple(inst,4,16),
}}
 
pub fn extract_branch(inst:u32)->i{
i { 
imm : extract_mul(inst,0 ,0),
}}
 
pub fn extract_strex(inst:u32)->strex{
strex { 
rt : extract_simple(inst,4,0),
rd : extract_simple(inst,4,12),
rn : extract_simple(inst,4,16),
imm:0,
rt2:15,
}}
 
pub fn extract_S_xrr_shr(inst:u32)->s_rrr_shr{
s_rrr_shr { 
rm : extract_simple(inst,4,0),
s:1,
rd:0,
rs : extract_simple(inst,4,8),
rn : extract_simple(inst,4,16),
shty : extract_simple(inst,2,5),
}}
 
pub fn extract_i16(inst:u32)->i{
i { 
imm : extract_mul(inst,8 ,12,0 ,4),
}}
 
pub fn extract_s_rrr_shr(inst:u32)->s_rrr_shr{
s_rrr_shr { 
rd : extract_simple(inst,4,12),
rs : extract_simple(inst,4,8),
rn : extract_simple(inst,4,16),
s : extract_simple(inst,1,20),
shty : extract_simple(inst,2,5),
rm : extract_simple(inst,4,0),
}}
 
pub fn extract_bfx(inst:u32)->bfx{
bfx { 
rd : extract_simple(inst,4,12),
lsb : extract_simple(inst,5,7),
rn : extract_simple(inst,4,0),
widthm1 : extract_simple(inst,5,16),
}}
 
pub fn extract_rdm(inst:u32)->rr{
rr { 
rd : extract_simple(inst,4,12),
rm : extract_simple(inst,4,0),
}}
 
pub fn extract_mcr(inst:u32)->mcr{
mcr { 
crn : extract_simple(inst,4,16),
rt : extract_simple(inst,4,12),
opc2 : extract_simple(inst,3,5),
cp : extract_simple(inst,4,8),
opc1 : extract_simple(inst,3,21),
crm : extract_simple(inst,4,0),
}}
 
pub fn extract_s_rdamn(inst:u32)->s_rrrr{
s_rrrr { 
s : extract_simple(inst,1,20),
rd : extract_simple(inst,4,16),
rm : extract_simple(inst,4,8),
rn : extract_simple(inst,4,0),
ra : extract_simple(inst,4,12),
}}
 
pub fn extract_S_xri_rot(inst:u32)->s_rri_rot{
s_rri_rot { 
rn : extract_simple(inst,4,16),
rot : extract_mul(inst,8 ,4),
rd:0,
imm : extract_simple(inst,8,0),
s:1,
}}
 
pub fn extract_ldst_ri12_p1w(inst:u32)->ldst_ri{
ldst_ri { 
w : extract_simple(inst,1,21),
imm : extract_simple(inst,12,0),
p:1,
rt : extract_simple(inst,4,12),
rn : extract_simple(inst,4,16),
u : extract_simple(inst,1,23),
}}
 
pub fn extract_s_rrr_shi(inst:u32)->s_rrr_shi{
s_rrr_shi { 
shty : extract_simple(inst,2,5),
s : extract_simple(inst,1,20),
rn : extract_simple(inst,4,16),
rd : extract_simple(inst,4,12),
shim : extract_simple(inst,5,7),
rm : extract_simple(inst,4,0),
}}
 
pub fn extract_ldst_rr_p1w(inst:u32)->ldst_rr{
ldst_rr { 
rt : extract_simple(inst,4,12),
rm : extract_simple(inst,4,0),
w : extract_simple(inst,1,21),
p:1,
u : extract_simple(inst,1,23),
shtype:0,
shimm:0,
rn : extract_simple(inst,4,16),
}}
 
pub fn extract_mov16(inst:u32)->ri{
ri { 
imm : extract_mul(inst,16 ,4,0 ,12),
rd : extract_simple(inst,4,12),
}}
 
pub fn extract_ldst_rr_pw0(inst:u32)->ldst_rr{
ldst_rr { 
p:0,
w:0,
rt : extract_simple(inst,4,12),
rn : extract_simple(inst,4,16),
rm : extract_simple(inst,4,0),
shimm:0,
shtype:0,
u : extract_simple(inst,1,23),
}}
 
pub fn extract_swp(inst:u32)->swp{
swp { 
rn : extract_simple(inst,4,16),
rt : extract_simple(inst,4,12),
rt2 : extract_simple(inst,4,0),
}}
 
pub fn extract_rrr_rot(inst:u32)->rrr_rot{
rrr_rot { 
rd : extract_simple(inst,4,12),
rn : extract_simple(inst,4,16),
rot : extract_simple(inst,2,10),
rm : extract_simple(inst,4,0),
}}
 
pub fn extract_ldst_rr_p0w1(inst:u32)->ldst_rr{
ldst_rr { 
p:0,
rt : extract_simple(inst,4,12),
w:0,
rn : extract_simple(inst,4,16),
shtype:0,
rm : extract_simple(inst,4,0),
u : extract_simple(inst,1,23),
shimm:0,
}}
 
pub fn extract_mcrr(inst:u32)->mcrr{
mcrr { 
rt2 : extract_simple(inst,4,16),
rt : extract_simple(inst,4,12),
opc1 : extract_simple(inst,4,4),
crm : extract_simple(inst,4,0),
cp : extract_simple(inst,4,8),
}}
 
pub fn extract_rm(inst:u32)->r{
r { 
rm : extract_simple(inst,4,0),
}}
 
pub fn extract_sat16(inst:u32)->sat{
sat { 
satimm : extract_simple(inst,4,16),
sh:0,
imm:0,
rn : extract_simple(inst,4,0),
rd : extract_simple(inst,4,12),
}}
 
pub fn extract_rd0mn(inst:u32)->rrrr{
rrrr { 
rd : extract_simple(inst,4,16),
ra:0,
rm : extract_simple(inst,4,8),
rn : extract_simple(inst,4,0),
}}
 
pub fn extract_ldst_ri8_p1w(inst:u32)->ldst_ri{
ldst_ri { 
p:1,
u : extract_simple(inst,1,23),
imm : extract_mul(inst,8 ,4,0 ,4),
rn : extract_simple(inst,4,16),
w : extract_simple(inst,1,21),
rt : extract_simple(inst,4,12),
}}
 
pub fn extract_sat(inst:u32)->sat{
sat { 
rn : extract_simple(inst,4,0),
sh : extract_simple(inst,1,6),
rd : extract_simple(inst,4,12),
satimm : extract_simple(inst,5,16),
imm : extract_simple(inst,5,7),
}}
 
pub fn extract_S_xrr_shi(inst:u32)->s_rrr_shi{
s_rrr_shi { 
rn : extract_simple(inst,4,16),
rm : extract_simple(inst,4,0),
shim : extract_simple(inst,5,7),
rd:0,
s:1,
shty : extract_simple(inst,2,5),
}}
 
pub fn extract_ldst_ri12_p0w1(inst:u32)->ldst_ri{
ldst_ri { 
p:0,
rn : extract_simple(inst,4,16),
imm : extract_simple(inst,12,0),
w:0,
u : extract_simple(inst,1,23),
rt : extract_simple(inst,4,12),
}}
 
pub fn extract_ldst_rs_p1w(inst:u32)->ldst_rr{
ldst_rr { 
w : extract_simple(inst,1,21),
shtype : extract_simple(inst,2,5),
rm : extract_simple(inst,4,0),
rt : extract_simple(inst,4,12),
shimm : extract_simple(inst,5,7),
u : extract_simple(inst,1,23),
p:1,
rn : extract_simple(inst,4,16),
}}
 
pub fn extract_ldst_ri8_pw0(inst:u32)->ldst_ri{
ldst_ri { 
w:0,
u : extract_simple(inst,1,23),
p:0,
rn : extract_simple(inst,4,16),
rt : extract_simple(inst,4,12),
imm : extract_mul(inst,8 ,4,0 ,4),
}}
 
pub fn extract_rndm(inst:u32)->rrr{
rrr { 
rn : extract_simple(inst,4,16),
rd : extract_simple(inst,4,12),
rm : extract_simple(inst,4,0),
}}
 
pub fn extract_s_rxr_shi(inst:u32)->s_rrr_shi{
s_rrr_shi { 
shim : extract_simple(inst,5,7),
shty : extract_simple(inst,2,5),
s : extract_simple(inst,1,20),
rm : extract_simple(inst,4,0),
rn:0,
rd : extract_simple(inst,4,12),
}}
 
pub fn extract_ldst_ri8_p0w1(inst:u32)->ldst_ri{
ldst_ri { 
rt : extract_simple(inst,4,12),
u : extract_simple(inst,1,23),
p:0,
w:0,
rn : extract_simple(inst,4,16),
imm : extract_mul(inst,8 ,4,0 ,4),
}}
 
