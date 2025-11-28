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
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_s_rrr_shi(inst:Instruction)->s_rrr_shi{
s_rrr_shi { 
shty : extract(inst,2,5),
rm : extract(inst,4,0),
rn : extract(inst,4,16),
shim : extract(inst,5,7),
rd : extract(inst,4,12),
s : extract(inst,1,20),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_s_rxr_shi(inst:Instruction)->s_rrr_shi{
s_rrr_shi { 
shim : extract(inst,5,7),
rd : extract(inst,4,12),
shty : extract(inst,2,5),
rm : extract(inst,4,0),
s : extract(inst,1,20),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_S_xrr_shi(inst:Instruction)->s_rrr_shi{
s_rrr_shi { 
rn : extract(inst,4,16),
shim : extract(inst,5,7),
rm : extract(inst,4,0),
shty : extract(inst,2,5),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_mov16(inst:Instruction)->ri{
ri { 
rd : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_s_rrr_shr(inst:Instruction)->s_rrr_shr{
s_rrr_shr { 
s : extract(inst,1,20),
rs : extract(inst,4,8),
shty : extract(inst,2,5),
rd : extract(inst,4,12),
rn : extract(inst,4,16),
rm : extract(inst,4,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_s_rxr_shr(inst:Instruction)->s_rrr_shr{
s_rrr_shr { 
shty : extract(inst,2,5),
rs : extract(inst,4,8),
s : extract(inst,1,20),
rd : extract(inst,4,12),
rm : extract(inst,4,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_S_xrr_shr(inst:Instruction)->s_rrr_shr{
s_rrr_shr { 
rn : extract(inst,4,16),
rs : extract(inst,4,8),
shty : extract(inst,2,5),
rm : extract(inst,4,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_s_rri_rot(inst:Instruction)->s_rri_rot{
s_rri_rot { 
s : extract(inst,1,20),
rn : extract(inst,4,16),
imm : extract(inst,8,0),
rd : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_s_rxi_rot(inst:Instruction)->s_rri_rot{
s_rri_rot { 
s : extract(inst,1,20),
rd : extract(inst,4,12),
imm : extract(inst,8,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_S_xri_rot(inst:Instruction)->s_rri_rot{
s_rri_rot { 
imm : extract(inst,8,0),
rn : extract(inst,4,16),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_s_rdamn(inst:Instruction)->s_rrrr{
s_rrrr { 
s : extract(inst,1,20),
rd : extract(inst,4,16),
rn : extract(inst,4,0),
ra : extract(inst,4,12),
rm : extract(inst,4,8),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_s_rd0mn(inst:Instruction)->s_rrrr{
s_rrrr { 
rd : extract(inst,4,16),
rn : extract(inst,4,0),
rm : extract(inst,4,8),
s : extract(inst,1,20),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_rdamn(inst:Instruction)->rrrr{
rrrr { 
ra : extract(inst,4,12),
rm : extract(inst,4,8),
rn : extract(inst,4,0),
rd : extract(inst,4,16),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_rd0mn(inst:Instruction)->rrrr{
rrrr { 
rd : extract(inst,4,16),
rm : extract(inst,4,8),
rn : extract(inst,4,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_rndm(inst:Instruction)->rrr{
rrr { 
rn : extract(inst,4,16),
rm : extract(inst,4,0),
rd : extract(inst,4,12),
}}
 
pub struct msr_i {
    pub r: u32,
    pub mask: u32,
    pub rot: u32,
    pub imm: u32,
}
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_msr_i(inst:Instruction)->msr_i{
msr_i { 
imm : extract(inst,8,0),
mask : extract(inst,4,16),
rot : extract(inst,4,8),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_rm(inst:Instruction)->r{
r { 
rm : extract(inst,4,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_rdm(inst:Instruction)->rr{
rr { 
rd : extract(inst,4,12),
rm : extract(inst,4,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_i16(inst:Instruction)->i{
i { 
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_rr_p1w(inst:Instruction)->ldst_rr{
ldst_rr { 
w : extract(inst,1,21),
rn : extract(inst,4,16),
rm : extract(inst,4,0),
rt : extract(inst,4,12),
u : extract(inst,1,23),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_rr_pw0(inst:Instruction)->ldst_rr{
ldst_rr { 
u : extract(inst,1,23),
rm : extract(inst,4,0),
rn : extract(inst,4,16),
rt : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_rr_p0w1(inst:Instruction)->ldst_rr{
ldst_rr { 
u : extract(inst,1,23),
rt : extract(inst,4,12),
rm : extract(inst,4,0),
rn : extract(inst,4,16),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_rs_p1w(inst:Instruction)->ldst_rr{
ldst_rr { 
shtype : extract(inst,2,5),
rt : extract(inst,4,12),
w : extract(inst,1,21),
u : extract(inst,1,23),
rn : extract(inst,4,16),
shimm : extract(inst,5,7),
rm : extract(inst,4,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_rs_pw0(inst:Instruction)->ldst_rr{
ldst_rr { 
shtype : extract(inst,2,5),
rn : extract(inst,4,16),
rm : extract(inst,4,0),
shimm : extract(inst,5,7),
u : extract(inst,1,23),
rt : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_rs_p0w1(inst:Instruction)->ldst_rr{
ldst_rr { 
u : extract(inst,1,23),
rt : extract(inst,4,12),
shimm : extract(inst,5,7),
rn : extract(inst,4,16),
rm : extract(inst,4,0),
shtype : extract(inst,2,5),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_ri8_p1w(inst:Instruction)->ldst_ri{
ldst_ri { 
w : extract(inst,1,21),
rn : extract(inst,4,16),
u : extract(inst,1,23),
rt : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_ri8_pw0(inst:Instruction)->ldst_ri{
ldst_ri { 
u : extract(inst,1,23),
rn : extract(inst,4,16),
rt : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_ri8_p0w1(inst:Instruction)->ldst_ri{
ldst_ri { 
rn : extract(inst,4,16),
u : extract(inst,1,23),
rt : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_ri12_p1w(inst:Instruction)->ldst_ri{
ldst_ri { 
u : extract(inst,1,23),
w : extract(inst,1,21),
rn : extract(inst,4,16),
rt : extract(inst,4,12),
imm : extract(inst,12,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_ri12_pw0(inst:Instruction)->ldst_ri{
ldst_ri { 
rt : extract(inst,4,12),
rn : extract(inst,4,16),
u : extract(inst,1,23),
imm : extract(inst,12,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldst_ri12_p0w1(inst:Instruction)->ldst_ri{
ldst_ri { 
imm : extract(inst,12,0),
u : extract(inst,1,23),
rn : extract(inst,4,16),
rt : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_swp(inst:Instruction)->{
 { 
rt2 : extract(inst,4,0),
rn : extract(inst,4,16),
rt : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_strex(inst:Instruction)->strex{
strex { 
rn : extract(inst,4,16),
rt : extract(inst,4,0),
rd : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_ldrex(inst:Instruction)->ldrex{
ldrex { 
rn : extract(inst,4,16),
rt : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_stl(inst:Instruction)->ldrex{
ldrex { 
rt : extract(inst,4,0),
rn : extract(inst,4,16),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_bfx(inst:Instruction)->bfx{
bfx { 
rd : extract(inst,4,12),
rn : extract(inst,4,0),
widthm1 : extract(inst,5,16),
lsb : extract(inst,5,7),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_sat(inst:Instruction)->sat{
sat { 
satimm : extract(inst,5,16),
imm : extract(inst,5,7),
sh : extract(inst,1,6),
rd : extract(inst,4,12),
rn : extract(inst,4,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_sat16(inst:Instruction)->sat{
sat { 
satimm : extract(inst,4,16),
rd : extract(inst,4,12),
rn : extract(inst,4,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_rrr_rot(inst:Instruction)->rrr_rot{
rrr_rot { 
rn : extract(inst,4,16),
rot : extract(inst,2,10),
rd : extract(inst,4,12),
rm : extract(inst,4,0),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_rdmn(inst:Instruction)->rrr{
rrr { 
rm : extract(inst,4,8),
rn : extract(inst,4,0),
rd : extract(inst,4,16),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_branch(inst:Instruction)->i{
i { 
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_mcr(inst:Instruction)->mcr{
mcr { 
crn : extract(inst,4,16),
crm : extract(inst,4,0),
opc2 : extract(inst,3,5),
cp : extract(inst,4,8),
opc1 : extract(inst,3,21),
rt : extract(inst,4,12),
}}
 
 fn extract(inst:u32,len:u32,pos:u32) -> u32 {
 (value >> len) & ((1u32 << pos) - 1)
}
pub fn extract_mcrr(inst:Instruction)->mcrr{
mcrr { 
rt2 : extract(inst,4,16),
opc1 : extract(inst,4,4),
crm : extract(inst,4,0),
cp : extract(inst,4,8),
rt : extract(inst,4,12),
}}
 
