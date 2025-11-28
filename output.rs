 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (inst >> len) & ((1u32 << pos) - 1)
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
pub struct s_rrrr {
    pub s: u32,
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}
pub struct rrrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub ra: u32,
}
pub struct rrr_rot {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
    pub rot: u32,
}
pub struct rrr {
    pub rd: u32,
    pub rn: u32,
    pub rm: u32,
}
pub struct rr {
    pub rd: u32,
    pub rm: u32,
}
pub struct ri {
    pub rd: u32,
    pub imm: u32,
}
pub struct r {
    pub rm: u32,
}
pub struct i {
    pub imm: u32,
}
pub struct msr_reg {
    pub rn: u32,
    pub r: u32,
    pub mask: u32,
}
pub struct mrs_reg {
    pub rd: u32,
    pub r: u32,
}
pub struct msr_bank {
    pub rn: u32,
    pub r: u32,
    pub sysm: u32,
}
pub struct mrs_bank {
    pub rd: u32,
    pub r: u32,
    pub sysm: u32,
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
pub struct ldst_ri {
    pub p: u32,
    pub w: u32,
    pub u: u32,
    pub rn: u32,
    pub rt: u32,
    pub imm: u32,
}
pub struct ldst_block {
    pub rn: u32,
    pub i: u32,
    pub b: u32,
    pub u: u32,
    pub w: u32,
    pub list: u32,
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
pub fn extract_s_rrr_shi(inst:u32)->s_rrr_shi{
s_rrr_shi { 
s : extract(inst,1,20),
rd : extract(inst,4,12),
rm : extract(inst,4,0),
shim : extract(inst,5,7),
rn : extract(inst,4,16),
shty : extract(inst,2,5),
}}
 
pub fn extract_s_rxr_shi(inst:u32)->s_rrr_shi{
s_rrr_shi { 
rd : extract(inst,4,12),
rm : extract(inst,4,0),
shty : extract(inst,2,5),
shim : extract(inst,5,7),
s : extract(inst,1,20),
rn:0,
}}
 
pub fn extract_S_xrr_shi(inst:u32)->s_rrr_shi{
s_rrr_shi { 
shim : extract(inst,5,7),
rn : extract(inst,4,16),
rm : extract(inst,4,0),
shty : extract(inst,2,5),
s:1,
rd:0,
}}
 
pub fn extract_mov16(inst:u32)->ri{
ri { 
rd : extract(inst,4,12),
imm:0,
}}
 
pub fn extract_s_rrr_shr(inst:u32)->s_rrr_shr{
s_rrr_shr { 
shty : extract(inst,2,5),
rs : extract(inst,4,8),
rm : extract(inst,4,0),
s : extract(inst,1,20),
rd : extract(inst,4,12),
rn : extract(inst,4,16),
}}
 
pub fn extract_s_rxr_shr(inst:u32)->s_rrr_shr{
s_rrr_shr { 
rs : extract(inst,4,8),
rd : extract(inst,4,12),
s : extract(inst,1,20),
shty : extract(inst,2,5),
rm : extract(inst,4,0),
rn:0,
}}
 
pub fn extract_S_xrr_shr(inst:u32)->s_rrr_shr{
s_rrr_shr { 
rs : extract(inst,4,8),
rn : extract(inst,4,16),
shty : extract(inst,2,5),
rm : extract(inst,4,0),
rd:0,
s:1,
}}
 
pub fn extract_s_rri_rot(inst:u32)->s_rri_rot{
s_rri_rot { 
rn : extract(inst,4,16),
s : extract(inst,1,20),
imm : extract(inst,8,0),
rd : extract(inst,4,12),
rot:0,
}}
 
pub fn extract_s_rxi_rot(inst:u32)->s_rri_rot{
s_rri_rot { 
imm : extract(inst,8,0),
rd : extract(inst,4,12),
s : extract(inst,1,20),
rot:0,
rn:0,
}}
 
pub fn extract_S_xri_rot(inst:u32)->s_rri_rot{
s_rri_rot { 
rn : extract(inst,4,16),
imm : extract(inst,8,0),
rot:0,
rd:0,
s:1,
}}
 
pub fn extract_s_rdamn(inst:u32)->s_rrrr{
s_rrrr { 
rd : extract(inst,4,16),
rm : extract(inst,4,8),
s : extract(inst,1,20),
rn : extract(inst,4,0),
ra : extract(inst,4,12),
}}
 
pub fn extract_s_rd0mn(inst:u32)->s_rrrr{
s_rrrr { 
s : extract(inst,1,20),
rn : extract(inst,4,0),
rd : extract(inst,4,16),
rm : extract(inst,4,8),
ra:0,
}}
 
pub fn extract_rdamn(inst:u32)->rrrr{
rrrr { 
rm : extract(inst,4,8),
rd : extract(inst,4,16),
rn : extract(inst,4,0),
ra : extract(inst,4,12),
}}
 
pub fn extract_rd0mn(inst:u32)->rrrr{
rrrr { 
rd : extract(inst,4,16),
rn : extract(inst,4,0),
rm : extract(inst,4,8),
ra:0,
}}
 
pub fn extract_rndm(inst:u32)->rrr{
rrr { 
rn : extract(inst,4,16),
rd : extract(inst,4,12),
rm : extract(inst,4,0),
}}
 
pub struct msr_i {
    pub r: u32,
    pub mask: u32,
    pub rot: u32,
    pub imm: u32,
}
pub fn extract_msr_i(inst:u32)->msr_i{
msr_i { 
rot : extract(inst,4,8),
mask : extract(inst,4,16),
imm : extract(inst,8,0),
}}
 
pub fn extract_rm(inst:u32)->r{
r { 
rm : extract(inst,4,0),
}}
 
pub fn extract_rdm(inst:u32)->rr{
rr { 
rd : extract(inst,4,12),
rm : extract(inst,4,0),
}}
 
pub fn extract_i16(inst:u32)->i{
i { 
imm:0,
}}
 
pub fn extract_ldst_rr_p1w(inst:u32)->ldst_rr{
ldst_rr { 
rt : extract(inst,4,12),
w : extract(inst,1,21),
rm : extract(inst,4,0),
rn : extract(inst,4,16),
u : extract(inst,1,23),
p:1,
shimm:0,
shtype:0,
}}
 
pub fn extract_ldst_rr_pw0(inst:u32)->ldst_rr{
ldst_rr { 
rm : extract(inst,4,0),
rt : extract(inst,4,12),
u : extract(inst,1,23),
rn : extract(inst,4,16),
p:0,
w:0,
shimm:0,
shtype:0,
}}
 
pub fn extract_ldst_rr_p0w1(inst:u32)->ldst_rr{
ldst_rr { 
rm : extract(inst,4,0),
rt : extract(inst,4,12),
rn : extract(inst,4,16),
u : extract(inst,1,23),
p:0,
w:0,
shimm:0,
shtype:0,
}}
 
pub fn extract_ldst_rs_p1w(inst:u32)->ldst_rr{
ldst_rr { 
rm : extract(inst,4,0),
w : extract(inst,1,21),
rt : extract(inst,4,12),
rn : extract(inst,4,16),
shimm : extract(inst,5,7),
u : extract(inst,1,23),
shtype : extract(inst,2,5),
p:1,
}}
 
pub fn extract_ldst_rs_pw0(inst:u32)->ldst_rr{
ldst_rr { 
shtype : extract(inst,2,5),
rn : extract(inst,4,16),
u : extract(inst,1,23),
rt : extract(inst,4,12),
shimm : extract(inst,5,7),
rm : extract(inst,4,0),
p:0,
w:0,
}}
 
pub fn extract_ldst_rs_p0w1(inst:u32)->ldst_rr{
ldst_rr { 
u : extract(inst,1,23),
rn : extract(inst,4,16),
shtype : extract(inst,2,5),
rm : extract(inst,4,0),
rt : extract(inst,4,12),
shimm : extract(inst,5,7),
p:0,
w:0,
}}
 
pub fn extract_ldst_ri8_p1w(inst:u32)->ldst_ri{
ldst_ri { 
u : extract(inst,1,23),
w : extract(inst,1,21),
rt : extract(inst,4,12),
rn : extract(inst,4,16),
imm:0,
p:1,
}}
 
pub fn extract_ldst_ri8_pw0(inst:u32)->ldst_ri{
ldst_ri { 
rn : extract(inst,4,16),
rt : extract(inst,4,12),
u : extract(inst,1,23),
imm:0,
p:0,
w:0,
}}
 
pub fn extract_ldst_ri8_p0w1(inst:u32)->ldst_ri{
ldst_ri { 
u : extract(inst,1,23),
rn : extract(inst,4,16),
rt : extract(inst,4,12),
imm:0,
p:0,
w:0,
}}
 
pub fn extract_ldst_ri12_p1w(inst:u32)->ldst_ri{
ldst_ri { 
imm : extract(inst,12,0),
rn : extract(inst,4,16),
rt : extract(inst,4,12),
w : extract(inst,1,21),
u : extract(inst,1,23),
p:1,
}}
 
pub fn extract_ldst_ri12_pw0(inst:u32)->ldst_ri{
ldst_ri { 
u : extract(inst,1,23),
rn : extract(inst,4,16),
rt : extract(inst,4,12),
imm : extract(inst,12,0),
p:0,
w:0,
}}
 
pub fn extract_ldst_ri12_p0w1(inst:u32)->ldst_ri{
ldst_ri { 
rt : extract(inst,4,12),
imm : extract(inst,12,0),
u : extract(inst,1,23),
rn : extract(inst,4,16),
p:0,
w:0,
}}
 
pub fn extract_swp(inst:u32)->swp{
swp { 
rn : extract(inst,4,16),
rt : extract(inst,4,12),
rt2 : extract(inst,4,0),
}}
 
pub fn extract_strex(inst:u32)->strex{
strex { 
rd : extract(inst,4,12),
rn : extract(inst,4,16),
rt : extract(inst,4,0),
imm:0,
rt2:15,
}}
 
pub fn extract_ldrex(inst:u32)->ldrex{
ldrex { 
rt : extract(inst,4,12),
rn : extract(inst,4,16),
imm:0,
rt2:15,
}}
 
pub fn extract_stl(inst:u32)->ldrex{
ldrex { 
rn : extract(inst,4,16),
rt : extract(inst,4,0),
imm:0,
rt2:15,
}}
 
pub fn extract_bfx(inst:u32)->bfx{
bfx { 
rd : extract(inst,4,12),
lsb : extract(inst,5,7),
rn : extract(inst,4,0),
widthm1 : extract(inst,5,16),
}}
 
pub fn extract_sat(inst:u32)->sat{
sat { 
sh : extract(inst,1,6),
satimm : extract(inst,5,16),
rd : extract(inst,4,12),
imm : extract(inst,5,7),
rn : extract(inst,4,0),
}}
 
pub fn extract_sat16(inst:u32)->sat{
sat { 
rd : extract(inst,4,12),
satimm : extract(inst,4,16),
rn : extract(inst,4,0),
imm:0,
sh:0,
}}
 
pub fn extract_rrr_rot(inst:u32)->rrr_rot{
rrr_rot { 
rot : extract(inst,2,10),
rm : extract(inst,4,0),
rd : extract(inst,4,12),
rn : extract(inst,4,16),
}}
 
pub fn extract_rdmn(inst:u32)->rrr{
rrr { 
rd : extract(inst,4,16),
rm : extract(inst,4,8),
rn : extract(inst,4,0),
}}
 
pub fn extract_branch(inst:u32)->i{
i { 
imm:0,
}}
 
pub fn extract_mcr(inst:u32)->mcr{
mcr { 
opc2 : extract(inst,3,5),
crm : extract(inst,4,0),
cp : extract(inst,4,8),
opc1 : extract(inst,3,21),
crn : extract(inst,4,16),
rt : extract(inst,4,12),
}}
 
pub fn extract_mcrr(inst:u32)->mcrr{
mcrr { 
crm : extract(inst,4,0),
rt : extract(inst,4,12),
opc1 : extract(inst,4,4),
cp : extract(inst,4,8),
rt2 : extract(inst,4,16),
}}
 
