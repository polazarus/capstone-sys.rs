use libc::c_uint;
use placeholders;


#[derive(Copy, Clone)]
#[repr(u32)]
pub enum mips_op_type {
    MIPS_OP_INVALID = 0,
    MIPS_OP_REG = 1,
    MIPS_OP_IMM = 2,
    MIPS_OP_MEM = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mips_op_mem {
    pub base: c_uint,
    pub disp: i64,
}
impl ::std::default::Default for mips_op_mem {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_mips_op {
    pub typ: mips_op_type,
    data: placeholders::mips_op_data,
}
impl cs_mips_op {
    pub unsafe fn reg(&self) -> &c_uint {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn imm(&self) -> &i64 {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn mem(&self) -> &mips_op_mem {
        ::std::mem::transmute(&self.data)
    }
}
impl ::std::default::Default for cs_mips_op {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_mips {
    pub op_count: u8,
    pub operands: [cs_mips_op; 8usize],
}
impl ::std::default::Default for cs_mips {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub const MIPS_REG_ZERO: mips_reg = mips_reg::MIPS_REG_0;
pub const MIPS_REG_AT: mips_reg = mips_reg::MIPS_REG_1;
pub const MIPS_REG_V0: mips_reg = mips_reg::MIPS_REG_2;
pub const MIPS_REG_V1: mips_reg = mips_reg::MIPS_REG_3;
pub const MIPS_REG_A0: mips_reg = mips_reg::MIPS_REG_4;
pub const MIPS_REG_A1: mips_reg = mips_reg::MIPS_REG_5;
pub const MIPS_REG_A2: mips_reg = mips_reg::MIPS_REG_6;
pub const MIPS_REG_A3: mips_reg = mips_reg::MIPS_REG_7;
pub const MIPS_REG_T0: mips_reg = mips_reg::MIPS_REG_8;
pub const MIPS_REG_T1: mips_reg = mips_reg::MIPS_REG_9;
pub const MIPS_REG_T2: mips_reg = mips_reg::MIPS_REG_10;
pub const MIPS_REG_T3: mips_reg = mips_reg::MIPS_REG_11;
pub const MIPS_REG_T4: mips_reg = mips_reg::MIPS_REG_12;
pub const MIPS_REG_T5: mips_reg = mips_reg::MIPS_REG_13;
pub const MIPS_REG_T6: mips_reg = mips_reg::MIPS_REG_14;
pub const MIPS_REG_T7: mips_reg = mips_reg::MIPS_REG_15;
pub const MIPS_REG_S0: mips_reg = mips_reg::MIPS_REG_16;
pub const MIPS_REG_S1: mips_reg = mips_reg::MIPS_REG_17;
pub const MIPS_REG_S2: mips_reg = mips_reg::MIPS_REG_18;
pub const MIPS_REG_S3: mips_reg = mips_reg::MIPS_REG_19;
pub const MIPS_REG_S4: mips_reg = mips_reg::MIPS_REG_20;
pub const MIPS_REG_S5: mips_reg = mips_reg::MIPS_REG_21;
pub const MIPS_REG_S6: mips_reg = mips_reg::MIPS_REG_22;
pub const MIPS_REG_S7: mips_reg = mips_reg::MIPS_REG_23;
pub const MIPS_REG_T8: mips_reg = mips_reg::MIPS_REG_24;
pub const MIPS_REG_T9: mips_reg = mips_reg::MIPS_REG_25;
pub const MIPS_REG_K0: mips_reg = mips_reg::MIPS_REG_26;
pub const MIPS_REG_K1: mips_reg = mips_reg::MIPS_REG_27;
pub const MIPS_REG_GP: mips_reg = mips_reg::MIPS_REG_28;
pub const MIPS_REG_SP: mips_reg = mips_reg::MIPS_REG_29;
pub const MIPS_REG_FP: mips_reg = mips_reg::MIPS_REG_30;
pub const MIPS_REG_S8: mips_reg = mips_reg::MIPS_REG_30;
pub const MIPS_REG_RA: mips_reg = mips_reg::MIPS_REG_31;
pub const MIPS_REG_HI0: mips_reg = mips_reg::MIPS_REG_AC0;
pub const MIPS_REG_HI1: mips_reg = mips_reg::MIPS_REG_AC1;
pub const MIPS_REG_HI2: mips_reg = mips_reg::MIPS_REG_AC2;
pub const MIPS_REG_HI3: mips_reg = mips_reg::MIPS_REG_AC3;
pub const MIPS_REG_LO0: mips_reg = mips_reg::MIPS_REG_AC0;
pub const MIPS_REG_LO1: mips_reg = mips_reg::MIPS_REG_AC1;
pub const MIPS_REG_LO2: mips_reg = mips_reg::MIPS_REG_AC2;
pub const MIPS_REG_LO3: mips_reg = mips_reg::MIPS_REG_AC3;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum mips_reg {
    MIPS_REG_INVALID = 0,
    MIPS_REG_0 = 1,
    MIPS_REG_1 = 2,
    MIPS_REG_2 = 3,
    MIPS_REG_3 = 4,
    MIPS_REG_4 = 5,
    MIPS_REG_5 = 6,
    MIPS_REG_6 = 7,
    MIPS_REG_7 = 8,
    MIPS_REG_8 = 9,
    MIPS_REG_9 = 10,
    MIPS_REG_10 = 11,
    MIPS_REG_11 = 12,
    MIPS_REG_12 = 13,
    MIPS_REG_13 = 14,
    MIPS_REG_14 = 15,
    MIPS_REG_15 = 16,
    MIPS_REG_16 = 17,
    MIPS_REG_17 = 18,
    MIPS_REG_18 = 19,
    MIPS_REG_19 = 20,
    MIPS_REG_20 = 21,
    MIPS_REG_21 = 22,
    MIPS_REG_22 = 23,
    MIPS_REG_23 = 24,
    MIPS_REG_24 = 25,
    MIPS_REG_25 = 26,
    MIPS_REG_26 = 27,
    MIPS_REG_27 = 28,
    MIPS_REG_28 = 29,
    MIPS_REG_29 = 30,
    MIPS_REG_30 = 31,
    MIPS_REG_31 = 32,
    MIPS_REG_DSPCCOND = 33,
    MIPS_REG_DSPCARRY = 34,
    MIPS_REG_DSPEFI = 35,
    MIPS_REG_DSPOUTFLAG = 36,
    MIPS_REG_DSPOUTFLAG16_19 = 37,
    MIPS_REG_DSPOUTFLAG20 = 38,
    MIPS_REG_DSPOUTFLAG21 = 39,
    MIPS_REG_DSPOUTFLAG22 = 40,
    MIPS_REG_DSPOUTFLAG23 = 41,
    MIPS_REG_DSPPOS = 42,
    MIPS_REG_DSPSCOUNT = 43,
    MIPS_REG_AC0 = 44,
    MIPS_REG_AC1 = 45,
    MIPS_REG_AC2 = 46,
    MIPS_REG_AC3 = 47,
    MIPS_REG_CC0 = 48,
    MIPS_REG_CC1 = 49,
    MIPS_REG_CC2 = 50,
    MIPS_REG_CC3 = 51,
    MIPS_REG_CC4 = 52,
    MIPS_REG_CC5 = 53,
    MIPS_REG_CC6 = 54,
    MIPS_REG_CC7 = 55,
    MIPS_REG_F0 = 56,
    MIPS_REG_F1 = 57,
    MIPS_REG_F2 = 58,
    MIPS_REG_F3 = 59,
    MIPS_REG_F4 = 60,
    MIPS_REG_F5 = 61,
    MIPS_REG_F6 = 62,
    MIPS_REG_F7 = 63,
    MIPS_REG_F8 = 64,
    MIPS_REG_F9 = 65,
    MIPS_REG_F10 = 66,
    MIPS_REG_F11 = 67,
    MIPS_REG_F12 = 68,
    MIPS_REG_F13 = 69,
    MIPS_REG_F14 = 70,
    MIPS_REG_F15 = 71,
    MIPS_REG_F16 = 72,
    MIPS_REG_F17 = 73,
    MIPS_REG_F18 = 74,
    MIPS_REG_F19 = 75,
    MIPS_REG_F20 = 76,
    MIPS_REG_F21 = 77,
    MIPS_REG_F22 = 78,
    MIPS_REG_F23 = 79,
    MIPS_REG_F24 = 80,
    MIPS_REG_F25 = 81,
    MIPS_REG_F26 = 82,
    MIPS_REG_F27 = 83,
    MIPS_REG_F28 = 84,
    MIPS_REG_F29 = 85,
    MIPS_REG_F30 = 86,
    MIPS_REG_F31 = 87,
    MIPS_REG_FCC0 = 88,
    MIPS_REG_FCC1 = 89,
    MIPS_REG_FCC2 = 90,
    MIPS_REG_FCC3 = 91,
    MIPS_REG_FCC4 = 92,
    MIPS_REG_FCC5 = 93,
    MIPS_REG_FCC6 = 94,
    MIPS_REG_FCC7 = 95,
    MIPS_REG_W0 = 96,
    MIPS_REG_W1 = 97,
    MIPS_REG_W2 = 98,
    MIPS_REG_W3 = 99,
    MIPS_REG_W4 = 100,
    MIPS_REG_W5 = 101,
    MIPS_REG_W6 = 102,
    MIPS_REG_W7 = 103,
    MIPS_REG_W8 = 104,
    MIPS_REG_W9 = 105,
    MIPS_REG_W10 = 106,
    MIPS_REG_W11 = 107,
    MIPS_REG_W12 = 108,
    MIPS_REG_W13 = 109,
    MIPS_REG_W14 = 110,
    MIPS_REG_W15 = 111,
    MIPS_REG_W16 = 112,
    MIPS_REG_W17 = 113,
    MIPS_REG_W18 = 114,
    MIPS_REG_W19 = 115,
    MIPS_REG_W20 = 116,
    MIPS_REG_W21 = 117,
    MIPS_REG_W22 = 118,
    MIPS_REG_W23 = 119,
    MIPS_REG_W24 = 120,
    MIPS_REG_W25 = 121,
    MIPS_REG_W26 = 122,
    MIPS_REG_W27 = 123,
    MIPS_REG_W28 = 124,
    MIPS_REG_W29 = 125,
    MIPS_REG_W30 = 126,
    MIPS_REG_W31 = 127,
    MIPS_REG_HI = 128,
    MIPS_REG_LO = 129,
    MIPS_REG_P0 = 130,
    MIPS_REG_P1 = 131,
    MIPS_REG_P2 = 132,
    MIPS_REG_MPL0 = 133,
    MIPS_REG_MPL1 = 134,
    MIPS_REG_MPL2 = 135,
    MIPS_REG_ENDING = 136,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum mips_insn {
    MIPS_INS_INVALID = 0,
    MIPS_INS_ABSQ_S = 1,
    MIPS_INS_ADD = 2,
    MIPS_INS_ADDIUPC = 3,
    MIPS_INS_ADDQH = 4,
    MIPS_INS_ADDQH_R = 5,
    MIPS_INS_ADDQ = 6,
    MIPS_INS_ADDQ_S = 7,
    MIPS_INS_ADDSC = 8,
    MIPS_INS_ADDS_A = 9,
    MIPS_INS_ADDS_S = 10,
    MIPS_INS_ADDS_U = 11,
    MIPS_INS_ADDUH = 12,
    MIPS_INS_ADDUH_R = 13,
    MIPS_INS_ADDU = 14,
    MIPS_INS_ADDU_S = 15,
    MIPS_INS_ADDVI = 16,
    MIPS_INS_ADDV = 17,
    MIPS_INS_ADDWC = 18,
    MIPS_INS_ADD_A = 19,
    MIPS_INS_ADDI = 20,
    MIPS_INS_ADDIU = 21,
    MIPS_INS_ALIGN = 22,
    MIPS_INS_ALUIPC = 23,
    MIPS_INS_AND = 24,
    MIPS_INS_ANDI = 25,
    MIPS_INS_APPEND = 26,
    MIPS_INS_ASUB_S = 27,
    MIPS_INS_ASUB_U = 28,
    MIPS_INS_AUI = 29,
    MIPS_INS_AUIPC = 30,
    MIPS_INS_AVER_S = 31,
    MIPS_INS_AVER_U = 32,
    MIPS_INS_AVE_S = 33,
    MIPS_INS_AVE_U = 34,
    MIPS_INS_BADDU = 35,
    MIPS_INS_BAL = 36,
    MIPS_INS_BALC = 37,
    MIPS_INS_BALIGN = 38,
    MIPS_INS_BC = 39,
    MIPS_INS_BC0F = 40,
    MIPS_INS_BC0FL = 41,
    MIPS_INS_BC0T = 42,
    MIPS_INS_BC0TL = 43,
    MIPS_INS_BC1EQZ = 44,
    MIPS_INS_BC1F = 45,
    MIPS_INS_BC1FL = 46,
    MIPS_INS_BC1NEZ = 47,
    MIPS_INS_BC1T = 48,
    MIPS_INS_BC1TL = 49,
    MIPS_INS_BC2EQZ = 50,
    MIPS_INS_BC2F = 51,
    MIPS_INS_BC2FL = 52,
    MIPS_INS_BC2NEZ = 53,
    MIPS_INS_BC2T = 54,
    MIPS_INS_BC2TL = 55,
    MIPS_INS_BC3F = 56,
    MIPS_INS_BC3FL = 57,
    MIPS_INS_BC3T = 58,
    MIPS_INS_BC3TL = 59,
    MIPS_INS_BCLRI = 60,
    MIPS_INS_BCLR = 61,
    MIPS_INS_BEQ = 62,
    MIPS_INS_BEQC = 63,
    MIPS_INS_BEQL = 64,
    MIPS_INS_BEQZALC = 65,
    MIPS_INS_BEQZC = 66,
    MIPS_INS_BGEC = 67,
    MIPS_INS_BGEUC = 68,
    MIPS_INS_BGEZ = 69,
    MIPS_INS_BGEZAL = 70,
    MIPS_INS_BGEZALC = 71,
    MIPS_INS_BGEZALL = 72,
    MIPS_INS_BGEZALS = 73,
    MIPS_INS_BGEZC = 74,
    MIPS_INS_BGEZL = 75,
    MIPS_INS_BGTZ = 76,
    MIPS_INS_BGTZALC = 77,
    MIPS_INS_BGTZC = 78,
    MIPS_INS_BGTZL = 79,
    MIPS_INS_BINSLI = 80,
    MIPS_INS_BINSL = 81,
    MIPS_INS_BINSRI = 82,
    MIPS_INS_BINSR = 83,
    MIPS_INS_BITREV = 84,
    MIPS_INS_BITSWAP = 85,
    MIPS_INS_BLEZ = 86,
    MIPS_INS_BLEZALC = 87,
    MIPS_INS_BLEZC = 88,
    MIPS_INS_BLEZL = 89,
    MIPS_INS_BLTC = 90,
    MIPS_INS_BLTUC = 91,
    MIPS_INS_BLTZ = 92,
    MIPS_INS_BLTZAL = 93,
    MIPS_INS_BLTZALC = 94,
    MIPS_INS_BLTZALL = 95,
    MIPS_INS_BLTZALS = 96,
    MIPS_INS_BLTZC = 97,
    MIPS_INS_BLTZL = 98,
    MIPS_INS_BMNZI = 99,
    MIPS_INS_BMNZ = 100,
    MIPS_INS_BMZI = 101,
    MIPS_INS_BMZ = 102,
    MIPS_INS_BNE = 103,
    MIPS_INS_BNEC = 104,
    MIPS_INS_BNEGI = 105,
    MIPS_INS_BNEG = 106,
    MIPS_INS_BNEL = 107,
    MIPS_INS_BNEZALC = 108,
    MIPS_INS_BNEZC = 109,
    MIPS_INS_BNVC = 110,
    MIPS_INS_BNZ = 111,
    MIPS_INS_BOVC = 112,
    MIPS_INS_BPOSGE32 = 113,
    MIPS_INS_BREAK = 114,
    MIPS_INS_BSELI = 115,
    MIPS_INS_BSEL = 116,
    MIPS_INS_BSETI = 117,
    MIPS_INS_BSET = 118,
    MIPS_INS_BZ = 119,
    MIPS_INS_BEQZ = 120,
    MIPS_INS_B = 121,
    MIPS_INS_BNEZ = 122,
    MIPS_INS_BTEQZ = 123,
    MIPS_INS_BTNEZ = 124,
    MIPS_INS_CACHE = 125,
    MIPS_INS_CEIL = 126,
    MIPS_INS_CEQI = 127,
    MIPS_INS_CEQ = 128,
    MIPS_INS_CFC1 = 129,
    MIPS_INS_CFCMSA = 130,
    MIPS_INS_CINS = 131,
    MIPS_INS_CINS32 = 132,
    MIPS_INS_CLASS = 133,
    MIPS_INS_CLEI_S = 134,
    MIPS_INS_CLEI_U = 135,
    MIPS_INS_CLE_S = 136,
    MIPS_INS_CLE_U = 137,
    MIPS_INS_CLO = 138,
    MIPS_INS_CLTI_S = 139,
    MIPS_INS_CLTI_U = 140,
    MIPS_INS_CLT_S = 141,
    MIPS_INS_CLT_U = 142,
    MIPS_INS_CLZ = 143,
    MIPS_INS_CMPGDU = 144,
    MIPS_INS_CMPGU = 145,
    MIPS_INS_CMPU = 146,
    MIPS_INS_CMP = 147,
    MIPS_INS_COPY_S = 148,
    MIPS_INS_COPY_U = 149,
    MIPS_INS_CTC1 = 150,
    MIPS_INS_CTCMSA = 151,
    MIPS_INS_CVT = 152,
    MIPS_INS_C = 153,
    MIPS_INS_CMPI = 154,
    MIPS_INS_DADD = 155,
    MIPS_INS_DADDI = 156,
    MIPS_INS_DADDIU = 157,
    MIPS_INS_DADDU = 158,
    MIPS_INS_DAHI = 159,
    MIPS_INS_DALIGN = 160,
    MIPS_INS_DATI = 161,
    MIPS_INS_DAUI = 162,
    MIPS_INS_DBITSWAP = 163,
    MIPS_INS_DCLO = 164,
    MIPS_INS_DCLZ = 165,
    MIPS_INS_DDIV = 166,
    MIPS_INS_DDIVU = 167,
    MIPS_INS_DERET = 168,
    MIPS_INS_DEXT = 169,
    MIPS_INS_DEXTM = 170,
    MIPS_INS_DEXTU = 171,
    MIPS_INS_DI = 172,
    MIPS_INS_DINS = 173,
    MIPS_INS_DINSM = 174,
    MIPS_INS_DINSU = 175,
    MIPS_INS_DIV = 176,
    MIPS_INS_DIVU = 177,
    MIPS_INS_DIV_S = 178,
    MIPS_INS_DIV_U = 179,
    MIPS_INS_DLSA = 180,
    MIPS_INS_DMFC0 = 181,
    MIPS_INS_DMFC1 = 182,
    MIPS_INS_DMFC2 = 183,
    MIPS_INS_DMOD = 184,
    MIPS_INS_DMODU = 185,
    MIPS_INS_DMTC0 = 186,
    MIPS_INS_DMTC1 = 187,
    MIPS_INS_DMTC2 = 188,
    MIPS_INS_DMUH = 189,
    MIPS_INS_DMUHU = 190,
    MIPS_INS_DMUL = 191,
    MIPS_INS_DMULT = 192,
    MIPS_INS_DMULTU = 193,
    MIPS_INS_DMULU = 194,
    MIPS_INS_DOTP_S = 195,
    MIPS_INS_DOTP_U = 196,
    MIPS_INS_DPADD_S = 197,
    MIPS_INS_DPADD_U = 198,
    MIPS_INS_DPAQX_SA = 199,
    MIPS_INS_DPAQX_S = 200,
    MIPS_INS_DPAQ_SA = 201,
    MIPS_INS_DPAQ_S = 202,
    MIPS_INS_DPAU = 203,
    MIPS_INS_DPAX = 204,
    MIPS_INS_DPA = 205,
    MIPS_INS_DPOP = 206,
    MIPS_INS_DPSQX_SA = 207,
    MIPS_INS_DPSQX_S = 208,
    MIPS_INS_DPSQ_SA = 209,
    MIPS_INS_DPSQ_S = 210,
    MIPS_INS_DPSUB_S = 211,
    MIPS_INS_DPSUB_U = 212,
    MIPS_INS_DPSU = 213,
    MIPS_INS_DPSX = 214,
    MIPS_INS_DPS = 215,
    MIPS_INS_DROTR = 216,
    MIPS_INS_DROTR32 = 217,
    MIPS_INS_DROTRV = 218,
    MIPS_INS_DSBH = 219,
    MIPS_INS_DSHD = 220,
    MIPS_INS_DSLL = 221,
    MIPS_INS_DSLL32 = 222,
    MIPS_INS_DSLLV = 223,
    MIPS_INS_DSRA = 224,
    MIPS_INS_DSRA32 = 225,
    MIPS_INS_DSRAV = 226,
    MIPS_INS_DSRL = 227,
    MIPS_INS_DSRL32 = 228,
    MIPS_INS_DSRLV = 229,
    MIPS_INS_DSUB = 230,
    MIPS_INS_DSUBU = 231,
    MIPS_INS_EHB = 232,
    MIPS_INS_EI = 233,
    MIPS_INS_ERET = 234,
    MIPS_INS_EXT = 235,
    MIPS_INS_EXTP = 236,
    MIPS_INS_EXTPDP = 237,
    MIPS_INS_EXTPDPV = 238,
    MIPS_INS_EXTPV = 239,
    MIPS_INS_EXTRV_RS = 240,
    MIPS_INS_EXTRV_R = 241,
    MIPS_INS_EXTRV_S = 242,
    MIPS_INS_EXTRV = 243,
    MIPS_INS_EXTR_RS = 244,
    MIPS_INS_EXTR_R = 245,
    MIPS_INS_EXTR_S = 246,
    MIPS_INS_EXTR = 247,
    MIPS_INS_EXTS = 248,
    MIPS_INS_EXTS32 = 249,
    MIPS_INS_ABS = 250,
    MIPS_INS_FADD = 251,
    MIPS_INS_FCAF = 252,
    MIPS_INS_FCEQ = 253,
    MIPS_INS_FCLASS = 254,
    MIPS_INS_FCLE = 255,
    MIPS_INS_FCLT = 256,
    MIPS_INS_FCNE = 257,
    MIPS_INS_FCOR = 258,
    MIPS_INS_FCUEQ = 259,
    MIPS_INS_FCULE = 260,
    MIPS_INS_FCULT = 261,
    MIPS_INS_FCUNE = 262,
    MIPS_INS_FCUN = 263,
    MIPS_INS_FDIV = 264,
    MIPS_INS_FEXDO = 265,
    MIPS_INS_FEXP2 = 266,
    MIPS_INS_FEXUPL = 267,
    MIPS_INS_FEXUPR = 268,
    MIPS_INS_FFINT_S = 269,
    MIPS_INS_FFINT_U = 270,
    MIPS_INS_FFQL = 271,
    MIPS_INS_FFQR = 272,
    MIPS_INS_FILL = 273,
    MIPS_INS_FLOG2 = 274,
    MIPS_INS_FLOOR = 275,
    MIPS_INS_FMADD = 276,
    MIPS_INS_FMAX_A = 277,
    MIPS_INS_FMAX = 278,
    MIPS_INS_FMIN_A = 279,
    MIPS_INS_FMIN = 280,
    MIPS_INS_MOV = 281,
    MIPS_INS_FMSUB = 282,
    MIPS_INS_FMUL = 283,
    MIPS_INS_MUL = 284,
    MIPS_INS_NEG = 285,
    MIPS_INS_FRCP = 286,
    MIPS_INS_FRINT = 287,
    MIPS_INS_FRSQRT = 288,
    MIPS_INS_FSAF = 289,
    MIPS_INS_FSEQ = 290,
    MIPS_INS_FSLE = 291,
    MIPS_INS_FSLT = 292,
    MIPS_INS_FSNE = 293,
    MIPS_INS_FSOR = 294,
    MIPS_INS_FSQRT = 295,
    MIPS_INS_SQRT = 296,
    MIPS_INS_FSUB = 297,
    MIPS_INS_SUB = 298,
    MIPS_INS_FSUEQ = 299,
    MIPS_INS_FSULE = 300,
    MIPS_INS_FSULT = 301,
    MIPS_INS_FSUNE = 302,
    MIPS_INS_FSUN = 303,
    MIPS_INS_FTINT_S = 304,
    MIPS_INS_FTINT_U = 305,
    MIPS_INS_FTQ = 306,
    MIPS_INS_FTRUNC_S = 307,
    MIPS_INS_FTRUNC_U = 308,
    MIPS_INS_HADD_S = 309,
    MIPS_INS_HADD_U = 310,
    MIPS_INS_HSUB_S = 311,
    MIPS_INS_HSUB_U = 312,
    MIPS_INS_ILVEV = 313,
    MIPS_INS_ILVL = 314,
    MIPS_INS_ILVOD = 315,
    MIPS_INS_ILVR = 316,
    MIPS_INS_INS = 317,
    MIPS_INS_INSERT = 318,
    MIPS_INS_INSV = 319,
    MIPS_INS_INSVE = 320,
    MIPS_INS_J = 321,
    MIPS_INS_JAL = 322,
    MIPS_INS_JALR = 323,
    MIPS_INS_JALRS = 324,
    MIPS_INS_JALS = 325,
    MIPS_INS_JALX = 326,
    MIPS_INS_JIALC = 327,
    MIPS_INS_JIC = 328,
    MIPS_INS_JR = 329,
    MIPS_INS_JRADDIUSP = 330,
    MIPS_INS_JRC = 331,
    MIPS_INS_JALRC = 332,
    MIPS_INS_LB = 333,
    MIPS_INS_LBUX = 334,
    MIPS_INS_LBU = 335,
    MIPS_INS_LD = 336,
    MIPS_INS_LDC1 = 337,
    MIPS_INS_LDC2 = 338,
    MIPS_INS_LDC3 = 339,
    MIPS_INS_LDI = 340,
    MIPS_INS_LDL = 341,
    MIPS_INS_LDPC = 342,
    MIPS_INS_LDR = 343,
    MIPS_INS_LDXC1 = 344,
    MIPS_INS_LH = 345,
    MIPS_INS_LHX = 346,
    MIPS_INS_LHU = 347,
    MIPS_INS_LL = 348,
    MIPS_INS_LLD = 349,
    MIPS_INS_LSA = 350,
    MIPS_INS_LUXC1 = 351,
    MIPS_INS_LUI = 352,
    MIPS_INS_LW = 353,
    MIPS_INS_LWC1 = 354,
    MIPS_INS_LWC2 = 355,
    MIPS_INS_LWC3 = 356,
    MIPS_INS_LWL = 357,
    MIPS_INS_LWPC = 358,
    MIPS_INS_LWR = 359,
    MIPS_INS_LWUPC = 360,
    MIPS_INS_LWU = 361,
    MIPS_INS_LWX = 362,
    MIPS_INS_LWXC1 = 363,
    MIPS_INS_LI = 364,
    MIPS_INS_MADD = 365,
    MIPS_INS_MADDF = 366,
    MIPS_INS_MADDR_Q = 367,
    MIPS_INS_MADDU = 368,
    MIPS_INS_MADDV = 369,
    MIPS_INS_MADD_Q = 370,
    MIPS_INS_MAQ_SA = 371,
    MIPS_INS_MAQ_S = 372,
    MIPS_INS_MAXA = 373,
    MIPS_INS_MAXI_S = 374,
    MIPS_INS_MAXI_U = 375,
    MIPS_INS_MAX_A = 376,
    MIPS_INS_MAX = 377,
    MIPS_INS_MAX_S = 378,
    MIPS_INS_MAX_U = 379,
    MIPS_INS_MFC0 = 380,
    MIPS_INS_MFC1 = 381,
    MIPS_INS_MFC2 = 382,
    MIPS_INS_MFHC1 = 383,
    MIPS_INS_MFHI = 384,
    MIPS_INS_MFLO = 385,
    MIPS_INS_MINA = 386,
    MIPS_INS_MINI_S = 387,
    MIPS_INS_MINI_U = 388,
    MIPS_INS_MIN_A = 389,
    MIPS_INS_MIN = 390,
    MIPS_INS_MIN_S = 391,
    MIPS_INS_MIN_U = 392,
    MIPS_INS_MOD = 393,
    MIPS_INS_MODSUB = 394,
    MIPS_INS_MODU = 395,
    MIPS_INS_MOD_S = 396,
    MIPS_INS_MOD_U = 397,
    MIPS_INS_MOVE = 398,
    MIPS_INS_MOVF = 399,
    MIPS_INS_MOVN = 400,
    MIPS_INS_MOVT = 401,
    MIPS_INS_MOVZ = 402,
    MIPS_INS_MSUB = 403,
    MIPS_INS_MSUBF = 404,
    MIPS_INS_MSUBR_Q = 405,
    MIPS_INS_MSUBU = 406,
    MIPS_INS_MSUBV = 407,
    MIPS_INS_MSUB_Q = 408,
    MIPS_INS_MTC0 = 409,
    MIPS_INS_MTC1 = 410,
    MIPS_INS_MTC2 = 411,
    MIPS_INS_MTHC1 = 412,
    MIPS_INS_MTHI = 413,
    MIPS_INS_MTHLIP = 414,
    MIPS_INS_MTLO = 415,
    MIPS_INS_MTM0 = 416,
    MIPS_INS_MTM1 = 417,
    MIPS_INS_MTM2 = 418,
    MIPS_INS_MTP0 = 419,
    MIPS_INS_MTP1 = 420,
    MIPS_INS_MTP2 = 421,
    MIPS_INS_MUH = 422,
    MIPS_INS_MUHU = 423,
    MIPS_INS_MULEQ_S = 424,
    MIPS_INS_MULEU_S = 425,
    MIPS_INS_MULQ_RS = 426,
    MIPS_INS_MULQ_S = 427,
    MIPS_INS_MULR_Q = 428,
    MIPS_INS_MULSAQ_S = 429,
    MIPS_INS_MULSA = 430,
    MIPS_INS_MULT = 431,
    MIPS_INS_MULTU = 432,
    MIPS_INS_MULU = 433,
    MIPS_INS_MULV = 434,
    MIPS_INS_MUL_Q = 435,
    MIPS_INS_MUL_S = 436,
    MIPS_INS_NLOC = 437,
    MIPS_INS_NLZC = 438,
    MIPS_INS_NMADD = 439,
    MIPS_INS_NMSUB = 440,
    MIPS_INS_NOR = 441,
    MIPS_INS_NORI = 442,
    MIPS_INS_NOT = 443,
    MIPS_INS_OR = 444,
    MIPS_INS_ORI = 445,
    MIPS_INS_PACKRL = 446,
    MIPS_INS_PAUSE = 447,
    MIPS_INS_PCKEV = 448,
    MIPS_INS_PCKOD = 449,
    MIPS_INS_PCNT = 450,
    MIPS_INS_PICK = 451,
    MIPS_INS_POP = 452,
    MIPS_INS_PRECEQU = 453,
    MIPS_INS_PRECEQ = 454,
    MIPS_INS_PRECEU = 455,
    MIPS_INS_PRECRQU_S = 456,
    MIPS_INS_PRECRQ = 457,
    MIPS_INS_PRECRQ_RS = 458,
    MIPS_INS_PRECR = 459,
    MIPS_INS_PRECR_SRA = 460,
    MIPS_INS_PRECR_SRA_R = 461,
    MIPS_INS_PREF = 462,
    MIPS_INS_PREPEND = 463,
    MIPS_INS_RADDU = 464,
    MIPS_INS_RDDSP = 465,
    MIPS_INS_RDHWR = 466,
    MIPS_INS_REPLV = 467,
    MIPS_INS_REPL = 468,
    MIPS_INS_RINT = 469,
    MIPS_INS_ROTR = 470,
    MIPS_INS_ROTRV = 471,
    MIPS_INS_ROUND = 472,
    MIPS_INS_SAT_S = 473,
    MIPS_INS_SAT_U = 474,
    MIPS_INS_SB = 475,
    MIPS_INS_SC = 476,
    MIPS_INS_SCD = 477,
    MIPS_INS_SD = 478,
    MIPS_INS_SDBBP = 479,
    MIPS_INS_SDC1 = 480,
    MIPS_INS_SDC2 = 481,
    MIPS_INS_SDC3 = 482,
    MIPS_INS_SDL = 483,
    MIPS_INS_SDR = 484,
    MIPS_INS_SDXC1 = 485,
    MIPS_INS_SEB = 486,
    MIPS_INS_SEH = 487,
    MIPS_INS_SELEQZ = 488,
    MIPS_INS_SELNEZ = 489,
    MIPS_INS_SEL = 490,
    MIPS_INS_SEQ = 491,
    MIPS_INS_SEQI = 492,
    MIPS_INS_SH = 493,
    MIPS_INS_SHF = 494,
    MIPS_INS_SHILO = 495,
    MIPS_INS_SHILOV = 496,
    MIPS_INS_SHLLV = 497,
    MIPS_INS_SHLLV_S = 498,
    MIPS_INS_SHLL = 499,
    MIPS_INS_SHLL_S = 500,
    MIPS_INS_SHRAV = 501,
    MIPS_INS_SHRAV_R = 502,
    MIPS_INS_SHRA = 503,
    MIPS_INS_SHRA_R = 504,
    MIPS_INS_SHRLV = 505,
    MIPS_INS_SHRL = 506,
    MIPS_INS_SLDI = 507,
    MIPS_INS_SLD = 508,
    MIPS_INS_SLL = 509,
    MIPS_INS_SLLI = 510,
    MIPS_INS_SLLV = 511,
    MIPS_INS_SLT = 512,
    MIPS_INS_SLTI = 513,
    MIPS_INS_SLTIU = 514,
    MIPS_INS_SLTU = 515,
    MIPS_INS_SNE = 516,
    MIPS_INS_SNEI = 517,
    MIPS_INS_SPLATI = 518,
    MIPS_INS_SPLAT = 519,
    MIPS_INS_SRA = 520,
    MIPS_INS_SRAI = 521,
    MIPS_INS_SRARI = 522,
    MIPS_INS_SRAR = 523,
    MIPS_INS_SRAV = 524,
    MIPS_INS_SRL = 525,
    MIPS_INS_SRLI = 526,
    MIPS_INS_SRLRI = 527,
    MIPS_INS_SRLR = 528,
    MIPS_INS_SRLV = 529,
    MIPS_INS_SSNOP = 530,
    MIPS_INS_ST = 531,
    MIPS_INS_SUBQH = 532,
    MIPS_INS_SUBQH_R = 533,
    MIPS_INS_SUBQ = 534,
    MIPS_INS_SUBQ_S = 535,
    MIPS_INS_SUBSUS_U = 536,
    MIPS_INS_SUBSUU_S = 537,
    MIPS_INS_SUBS_S = 538,
    MIPS_INS_SUBS_U = 539,
    MIPS_INS_SUBUH = 540,
    MIPS_INS_SUBUH_R = 541,
    MIPS_INS_SUBU = 542,
    MIPS_INS_SUBU_S = 543,
    MIPS_INS_SUBVI = 544,
    MIPS_INS_SUBV = 545,
    MIPS_INS_SUXC1 = 546,
    MIPS_INS_SW = 547,
    MIPS_INS_SWC1 = 548,
    MIPS_INS_SWC2 = 549,
    MIPS_INS_SWC3 = 550,
    MIPS_INS_SWL = 551,
    MIPS_INS_SWR = 552,
    MIPS_INS_SWXC1 = 553,
    MIPS_INS_SYNC = 554,
    MIPS_INS_SYSCALL = 555,
    MIPS_INS_TEQ = 556,
    MIPS_INS_TEQI = 557,
    MIPS_INS_TGE = 558,
    MIPS_INS_TGEI = 559,
    MIPS_INS_TGEIU = 560,
    MIPS_INS_TGEU = 561,
    MIPS_INS_TLBP = 562,
    MIPS_INS_TLBR = 563,
    MIPS_INS_TLBWI = 564,
    MIPS_INS_TLBWR = 565,
    MIPS_INS_TLT = 566,
    MIPS_INS_TLTI = 567,
    MIPS_INS_TLTIU = 568,
    MIPS_INS_TLTU = 569,
    MIPS_INS_TNE = 570,
    MIPS_INS_TNEI = 571,
    MIPS_INS_TRUNC = 572,
    MIPS_INS_V3MULU = 573,
    MIPS_INS_VMM0 = 574,
    MIPS_INS_VMULU = 575,
    MIPS_INS_VSHF = 576,
    MIPS_INS_WAIT = 577,
    MIPS_INS_WRDSP = 578,
    MIPS_INS_WSBH = 579,
    MIPS_INS_XOR = 580,
    MIPS_INS_XORI = 581,
    MIPS_INS_NOP = 582,
    MIPS_INS_NEGU = 583,
    MIPS_INS_JALR_HB = 584,
    MIPS_INS_JR_HB = 585,
    MIPS_INS_ENDING = 586,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum mips_insn_group {
    MIPS_GRP_INVALID = 0,
    MIPS_GRP_JUMP = 1,
    MIPS_GRP_BITCOUNT = 128,
    MIPS_GRP_DSP = 129,
    MIPS_GRP_DSPR2 = 130,
    MIPS_GRP_FPIDX = 131,
    MIPS_GRP_MSA = 132,
    MIPS_GRP_MIPS32R2 = 133,
    MIPS_GRP_MIPS64 = 134,
    MIPS_GRP_MIPS64R2 = 135,
    MIPS_GRP_SEINREG = 136,
    MIPS_GRP_STDENC = 137,
    MIPS_GRP_SWAP = 138,
    MIPS_GRP_MICROMIPS = 139,
    MIPS_GRP_MIPS16MODE = 140,
    MIPS_GRP_FP64BIT = 141,
    MIPS_GRP_NONANSFPMATH = 142,
    MIPS_GRP_NOTFP64BIT = 143,
    MIPS_GRP_NOTINMICROMIPS = 144,
    MIPS_GRP_NOTNACL = 145,
    MIPS_GRP_NOTMIPS32R6 = 146,
    MIPS_GRP_NOTMIPS64R6 = 147,
    MIPS_GRP_CNMIPS = 148,
    MIPS_GRP_MIPS32 = 149,
    MIPS_GRP_MIPS32R6 = 150,
    MIPS_GRP_MIPS64R6 = 151,
    MIPS_GRP_MIPS2 = 152,
    MIPS_GRP_MIPS3 = 153,
    MIPS_GRP_MIPS3_32 = 154,
    MIPS_GRP_MIPS3_32R2 = 155,
    MIPS_GRP_MIPS4_32 = 156,
    MIPS_GRP_MIPS4_32R2 = 157,
    MIPS_GRP_MIPS5_32R2 = 158,
    MIPS_GRP_GP32BIT = 159,
    MIPS_GRP_GP64BIT = 160,
    MIPS_GRP_ENDING = 161,
}
