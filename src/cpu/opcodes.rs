
// RV32I Base Instruction Set, Version 2.1

pub const OP_LUI:             u32 = 0b_0110111;
pub const OP_AUIPC:           u32 = 0b_0010111;
pub const OP_JAL:             u32 = 0b_1101111;
pub const OP_JALR:            u32 = 0b_1100111;
pub const OP_BR___:           u32 = 0b_1100011;
    pub const F3_BEQ:         u32 = 0b_000;
    pub const F3_BNE:         u32 = 0b_001;
    pub const F3_BLT:         u32 = 0b_100;
    pub const F3_BGE:         u32 = 0b_101;
    pub const F3_BLTU:        u32 = 0b_110;
    pub const F3_BGEU:        u32 = 0b_111;
pub const OP_LD___:           u32 = 0b_0000011;
    pub const F3_LB:          u32 = 0b_000;
    pub const F3_LH:          u32 = 0b_001;
    pub const F3_LW:          u32 = 0b_010;
    pub const F3_LBU:         u32 = 0b_100;
    pub const F3_LHU:         u32 = 0b_101;
pub const OP_ST___:           u32 = 0b_0100011;
    pub const F3_SB:          u32 = 0b_000;
    pub const F3_SH:          u32 = 0b_001;
    pub const F3_SW:          u32 = 0b_010;
pub const OP_MO1___:          u32 = 0b_0010011;
    pub const F3_ADDI:        u32 = 0x_000;
    pub const F3_SLTI:        u32 = 0b_010;
    pub const F3_SLTIU:       u32 = 0b_011;
    pub const F3_XORI:        u32 = 0b_100;
    pub const F3_ORI:         u32 = 0b_110;
    pub const F3_ANDI:        u32 = 0b_111;
    pub const F3_SL___:       u32 = 0b_001;
        pub const F7_SLLI:    u32 = 0b_0000000;
    pub const F3_SR___:       u32 = 0b_101;
        pub const F7_SRLI:    u32 = 0b_0000000;
        pub const F7_SRAI:    u32 = 0b_0100000;
pub const OP_MO2___:          u32 = 0b_0110011;
    pub const F3_AS___:       u32 = 0b_000;
        pub const F7_ADD:     u32 = 0b_0000000;
        pub const F7_SUB:     u32 = 0b_0100000;
    pub const F3_SLL:         u32 = 0b_001;
        pub const F7_SLL:     u32 = 0b_0000000;
    pub const F3_SLT:         u32 = 0b_010;
        pub const F7_SLT:     u32 = 0b_0000000;
    pub const F3_SLTU:        u32 = 0b_011;
        pub const F7_SLTU:    u32 = 0b_0000000;
    pub const F3_XOR:         u32 = 0b_100;
        pub const F7_XOR:     u32 = 0b_0000000;
    pub const F3_SR2___:      u32 = 0b_101;
        pub const F7_SRL:     u32 = 0b_0000000;
        pub const F7_SRA:     u32 = 0b_0100000;
    pub const F3_OR:          u32 = 0b_110;
        pub const F7_OR:      u32 = 0b_0000000;
    pub const F3_AND:         u32 = 0b_111;
        pub const F7_AND:     u32 = 0b_0000000;
pub const OP_MISC_MEM___:     u32 = 0b_0001111;
    pub const F3_FENCE:       u32 = 0b_000;
pub const OP_SYSTEM___:       u32 = 0b_1110011;
    pub const F3_SYS_PRIV___: u32 = 0b_000;
        pub const F12_ECALL:  u32 = 0b_000000000000;
        pub const F12_EBREAK: u32 = 0b_000000000001;


// "Zifencei" Instruction-Fetch Fence, Version 2.0


// RV32E Base Integer Instruction Set, Version 1.9


// RV64I Base Instruction Set (in addition to RV32I)

// OP_LD___
    pub const F3_LWU:         u32 = 0b_110;
    pub const F3_LD:          u32 = 0b_011;
// OP_ST___
    pub const F3_SD:          u32 = 0b_011;
pub const OP_MO3___:          u32 = 0b_0011011;
    pub const F3_ADDIW:       u32 = 0b_000;
    pub const F3_SHL___:      u32 = 0b_001;
        pub const F7_SLLIW:   u32 = 0b_0000000;
    pub const F3_SHR___:      u32 = 0b_101;
        pub const F7_SRLIW:   u32 = 0b_0000000;
        pub const F7_SRAIW:   u32 = 0b_0100000;
pub const OP_MO4___:          u32 = 0b_0111011;
    pub const F3_ASW___:      u32 = 0b_000;
        pub const F7_ADDW:    u32 = 0b_0000000;
        pub const F7_SUBW:    u32 = 0b_0100000;
    pub const F3_SLLW___:     u32 = 0b_001;
        pub const F7_SLLW:    u32 = 0b_0000000;
    // F3_SHR___
        pub const F7_SRLW:    u32 = 0b_0000000;
        pub const F7_SRAW:    u32 = 0b_0100000;


// RV128I Base Integer Instruction Set, Version 1.7


// "M" Standard Extension for Integer Multiplication and Division, Version 2.0

// RV32M Standard Extension
// OP_MO2___
    pub const F3_32_MUL___:    u32 = 0b_000;
    pub const F3_32_MULH___:   u32 = 0b_001;
    pub const F3_32_MULHSU___: u32 = 0b_010;
    pub const F3_32_MULHU___:  u32 = 0b_011;
    pub const F3_32_DIV___:    u32 = 0b_100;
    pub const F3_32_DIVU___:   u32 = 0b_101;
    pub const F3_32_REM___:    u32 = 0b_110;
    pub const F3_32_REMU___:   u32 = 0b_111;
        pub const F7_M_EXT:    u32 = 0b_0000001;
// RV64M Standard Extension (in addition to RV32M)
// OP_MO4___
    pub const F3_64_MULW___:   u32 = 0b_000;
    pub const F3_64_DIVW___:   u32 = 0b_100;
    pub const F3_64_DIVUW___:  u32 = 0b_101;
    pub const F3_64_REMW___:   u32 = 0b_110;
    pub const F3_64_REMUW___:  u32 = 0b_111;
        // F7_M_EXT

// "A" Standard Extension for Atomic Instructions, Version 2.1

/*
RV32A Standard Extension

00010 aq rl  00000  rs1  010  rd  0101111  lr.w
00011 aq rl  rs2    rs1  010  rd  0101111  sc.w
00001 aq rl  rs2    rs1  010  rd  0101111  amoswap.w
00000 aq rl  rs2    rs1  010  rd  0101111  amoadd.w
00100 aq rl  rs2    rs1  010  rd  0101111  amoxor.w
01100 aq rl  rs2    rs1  010  rd  0101111  amoand.w
01000 aq rl  rs2    rs1  010  rd  0101111  amoor.w
10000 aq rl  rs2    rs1  010  rd  0101111  amomin.w
10100 aq rl  rs2    rs1  010  rd  0101111  amomax.w
11000 aq rl  rs2    rs1  010  rd  0101111  amominu.w
11100 aq rl  rs2    rs1  010  rd  0101111  amomaxu.w

RV64A Standard Extension (in addition to RV32A)

00010 aq rl  00000  rs1  011  rd  0101111  lr.d
00011 aq rl  rs2    rs1  011  rd  0101111  sc.d
00001 aq rl  rs2    rs1  011  rd  0101111  amoswap.d
00000 aq rl  rs2    rs1  011  rd  0101111  amoadd.d
00100 aq rl  rs2    rs1  011  rd  0101111  amoxor.d
01100 aq rl  rs2    rs1  011  rd  0101111  amoand.d
01000 aq rl  rs2    rs1  011  rd  0101111  amoor.d
10000 aq rl  rs2    rs1  011  rd  0101111  amomin.d
10100 aq rl  rs2    rs1  011  rd  0101111  amomax.d
11000 aq rl  rs2    rs1  011  rd  0101111  amominu.d
11100 aq rl  rs2    rs1  011  rd  0101111  amomaxu.d
*/

pub const EXT_A_F7_MASK: u32 = 0b_1111100;

// RV32A Standard Extension
pub const OP_EXT_A___:           u32   = 0b_0101111;
    pub const F3_EXT_A32___:     u32   = 0b_010;
        pub const F7_LR_W___:    u32   = 0b_00010_00;
            pub const RS2_LR_W:  usize = 0b_00000;
        pub const F7_SC_W:       u32   = 0b_00011_00;
        pub const F7_AMOSWAP_W:  u32   = 0b_00001_00;
        pub const F7_AMOADD_W:   u32   = 0b_00000_00;
        pub const F7_AMOXOR_W:   u32   = 0b_00100_00;
        pub const F7_AMOAND_W:   u32   = 0b_01100_00;
        pub const F7_AMOOR_W:    u32   = 0b_01000_00;
        pub const F7_AMOMIN_W:   u32   = 0b_10000_00;
        pub const F7_AMOMAX_W:   u32   = 0b_10100_00;
        pub const F7_AMOMINU_W:  u32   = 0b_11000_00;
        pub const F7_AMOMAXU_W:  u32   = 0b_11100_00;

// RV64A Standard Extension (in addition to RV32A)
// OP_EXT_A___
    pub const F3_EXT_A64___:     u32   = 0b_011;
        pub const F7_LR_D___:    u32   = 0b_00010_00;
            pub const RS2_LR_D:  usize = 0b_00000;
        pub const F7_SC_D:       u32   = 0b_00011_00;
        pub const F7_AMOSWAP_D:  u32   = 0b_00001_00;
        pub const F7_AMOADD_D:   u32   = 0b_00000_00;
        pub const F7_AMOXOR_D:   u32   = 0b_00100_00;
        pub const F7_AMOAND_D:   u32   = 0b_01100_00;
        pub const F7_AMOOR_D:    u32   = 0b_01000_00;
        pub const F7_AMOMIN_D:   u32   = 0b_10000_00;
        pub const F7_AMOMAX_D:   u32   = 0b_10100_00;
        pub const F7_AMOMINU_D:  u32   = 0b_11000_00;
        pub const F7_AMOMAXU_D:  u32   = 0b_11100_00;


// "Zicsr" Control and Status Register (CSR) Instructions, Version 2.0

// OP_SYSTEM___
    pub const F3_CSRRW:        u32 = 0b_001;
    pub const F3_CSRRS:        u32 = 0b_010;
    pub const F3_CSRRC:        u32 = 0b_011;
    pub const F3_CSRRWI:       u32 = 0b_101;
    pub const F3_CSRRSI:       u32 = 0b_110;
    pub const F3_CSRRCI:       u32 = 0b_111;


// Counters


// "F" Standard Extension for Single-Precision Floating-Point, Version 2.2
/*
imm[11:0]          rs1   010   rd        0000111  flw
imm[11:5]   rs2    rs1   010   imm[4:0]  0100111  fsw
rs3 00      rs2    rs1   rm    rd        1000011  fmadd.s
rs3 00      rs2    rs1   rm    rd        1000111  fmsub.s
rs3 00      rs2    rs1   rm    rd        1001011  fnmsub.s
rs3 00      rs2    rs1   rm    rd        1001111  fnmadd.s
0000000     rs2    rs1   rm    rd        1010011  fadd.s
0000100     rs2    rs1   rm    rd        1010011  fsub.s
0001000     rs2    rs1   rm    rd        1010011  fmul.s
0001100     rs2    rs1   rm    rd        1010011  fdiv.s
0101100     00000  rs1   rm    rd        1010011  fsqrt.s
0010000     rs2    rs1   000   rd        1010011  fsgnj.s
0010000     rs2    rs1   001   rd        1010011  fsgnjn.s
0010000     rs2    rs1   010   rd        1010011  fsgnjx.s
0010100     rs2    rs1   000   rd        1010011  fmin.s
0010100     rs2    rs1   001   rd        1010011  fmax.s
1100000     00000  rs1   rm    rd        1010011  fcvt.w.s
1100000     00001  rs1   rm    rd        1010011  fcvt.wu.s
1110000     00000  rs1   000   rd        1010011  fmv.x.w
1010000     rs2    rs1   010   rd        1010011  feq.s
1010000     rs2    rs1   001   rd        1010011  flt.s
1010000     rs2    rs1   000   rd        1010011  fle.s
1110000     00000  rs1   001   rd        1010011  fclass.s
1101000     00000  rs1   rm    rd        1010011  fcvt.s.w
1101000     00001  rs1   rm    rd        1010011  fcvt.s.wu
1111000     00000  rs1   000   rd        1010011  fmv.w.x

RV64F Standard Extension (in addition to RV32F)
1100000     00010  rs1   rm    rd        1010011  FCVT.L.S
1100000     00011  rs1   rm    rd        1010011  FCVT.LU.S
1101000     00010  rs1   rm    rd        1010011  FCVT.S.L
1101000     00011  rs1   rm    rd        1010011  FCVT.S.LU
*/

pub const RS3_MASK: u32 = 0b_0000011;

// RV32F Standard Extension
pub const OP_FLW:                     u32   = 0b_0000111;
pub const OP_FSW:                     u32   = 0b_0100111;
pub const OP_FMADD_S___:              u32   = 0b_1000011;
    pub const F7_FMADD_S:             u32   = 0b_00;
pub const OP_FMSUB_S___:              u32   = 0b_1000111;
    pub const F7_FMSUB_S:             u32   = 0b_00;
pub const OP_FNMSUB_S___:             u32   = 0b_1001011;
    pub const F7_FNMSUB_S:            u32   = 0b_00;
pub const OP_FNMADD_S___:             u32   = 0b_1001111;
    pub const F7_FNMADD_S:            u32   = 0b_00;
pub const OP_RV_F___:                 u32   = 0b_1010011;
        pub const F7_FADD_S:          u32   = 0b_0000000;
        pub const F7_FSUB_S:          u32   = 0b_0000100;
        pub const F7_FMUL_S:          u32   = 0b_0001000;
        pub const F7_FDIV_S:          u32   = 0b_0001100;
        pub const F7_FSQRT_S:         u32   = 0b_0101100;
            pub const RS2_F7_FSQRT_S: usize = 0b_00000;
        pub const F7_FSGNJ___:        u32   = 0b_0010000;
            pub const F3_FSGNJ_S:     u32   = 0b_000;
            pub const F3_FSGNJN_S:    u32   = 0b_001;
            pub const F3_FSGNJX_S:    u32   = 0b_010;
        pub const F7_FSMM___:         u32   = 0b_0010100;
            pub const F3_FMIN_S:      u32   = 0b_000;
            pub const F3_FMAX_S:      u32   = 0b_001;
        pub const F7_FCVT_W___:       u32   = 0b_1100000;
            pub const F3_FCVT_W_S:    u32   = 0b_000;
            pub const F3_FCVT_WU_S:   u32   = 0b_001;
    pub const F3_32_FMV___:           u32   = 0b_000;
        pub const F7_FMV_X_W:         u32   = 0b_1110000;
            pub const RS2_FMV_X_W:    usize = 0b_00000;
        pub const F7_FLE_S:           u32   = 0b_1010000;
        pub const F7_FMV_W_X:         u32   = 0b_1111000;
            pub const RS2_FMV_W_X:    usize = 0b_00000;
    pub const F3_32_FE___:            u32   = 0b_010;
        pub const F7_FEQ_S:           u32   = 0b_1010000;
    pub const F3_32_FLT___:           u32   = 0b_001;
        pub const F7_FLT_S:           u32   = 0b_1010000;
    pub const F3_32_FC___:            u32   = 0b_001;
        pub const F7_FCLASS_S:        u32   = 0b_1110000;
            pub const RS2_FCLASS_S:   usize = 0b_00000;
        pub const F7_FCVT_S___:       u32   = 0b_1101000;  // rs2 = 00000
            pub const F3_FCVT_S_W:    u32   = 0b_000;
            pub const F3_FCVT_S_WU:   u32   = 0b_001;

// RV64F Standard Extension (in addition to RV32F)
// OP_RV_F___
        // F7_FCVT_W___
            pub const RS2_FCVT_L_S:   usize = 0b_00010;
            pub const RS2_FCVT_LU_S:  usize = 0b_00011;
        // F7_FCVT_S___
            pub const RS2_FCVT_S_L:   usize = 0b_00010;
            pub const RS2_FCVT_S_LU:  usize = 0b_00011;


// "D" Standard Extension for Double-Precision Floating-Point, Version 2.2
/*
RV32D Standard Extension
imm[11:0]         rs1  011  rd        0000111  fld
imm[11:5]  rs2    rs1  011  imm[4:0]  0100111  fsd
rs3 01     rs2    rs1  rm   rd        1000011  fmadd.d
rs3 01     rs2    rs1  rm   rd        1000111  fmsub.d
rs3 01     rs2    rs1  rm   rd        1001011  fnmsub.d
rs3 01     rs2    rs1  rm   rd        1001111  fnmadd.d
0000001    rs2    rs1  rm   rd        1010011  fadd.d
0000101    rs2    rs1  rm   rd        1010011  fsub.d
0001001    rs2    rs1  rm   rd        1010011  fmul.d
0001101    rs2    rs1  rm   rd        1010011  fdiv.d
0101101    00000  rs1  rm   rd        1010011  fsqrt.d
0010001    rs2    rs1  000  rd        1010011  fsgnj.d
0010001    rs2    rs1  001  rd        1010011  fsgnjn.d
0010001    rs2    rs1  010  rd        1010011  fsgnjx.d
0010101    rs2    rs1  000  rd        1010011  fmin.d
0010101    rs2    rs1  001  rd        1010011  fmax.d
0100000    00001  rs1  rm   rd        1010011  fcvt.s.d
0100001    00000  rs1  rm   rd        1010011  fcvt.d.s
1010001    rs2    rs1  010  rd        1010011  feq.d
1010001    rs2    rs1  001  rd        1010011  flt.d
1010001    rs2    rs1  000  rd        1010011  fle.d
1110001    00000  rs1  001  rd        1010011  fclass.d
1100001    00000  rs1  rm   rd        1010011  fcvt.w.d
1100001    00001  rs1  rm   rd        1010011  fcvt.wu.d
1101001    00000  rs1  rm   rd        1010011  fcvt.d.w
1101001    00001  rs1  rm   rd        1010011  fcvt.d.wu

RV64D Standard Extension (in addition to RV32D)
1100001    00010  rs1  rm   rd        1010011  fcvt.l.d
1100001    00011  rs1  rm   rd        1010011  fcvt.lu.d
1110001    00000  rs1  000  rd        1010011  fmv.x.d
1101001    00010  rs1  rm   rd        1010011  fcvt.d.l
1101001    00011  rs1  rm   rd        1010011  fcvt.d.lu
1111001    00000  rs1  000  rd        1010011  fmv.d.x
*/

// RV32D Standard Extension

// OP_FLW
    pub const F3_FLD:                 u32   = 0b_011;
// OP_FSW
    pub const F3_FSD:                 u32   = 0b_011;
// OP_FMADD_S
    pub const F7_FMADD_D:             u32   = 0b_01;
// OP_FMSUB_S
    pub const F7_FMSUB_D:             u32   = 0b_01;
// OP_FNMSUB_S
    pub const F7_FNMSUB_D:            u32   = 0b_01;
// OP_FNMADD_S
    pub const F7_FNMADD_D:            u32   = 0b_01;
// OP_RV_F___
    pub const F7_FADD_D:              u32   = 0b_0000001;
    pub const F7_FSUB_D:              u32   = 0b_0000101;
    pub const F7_FMUL_D:              u32   = 0b_0001001;
    pub const F7_FDIV_D:              u32   = 0b_0001101;
    pub const F7_FSQRT_D:             u32   = 0b_0101101;
        pub const RS2_F7_FSQRT_D:     usize = 0b_00000;
    pub const F7_FSGNJ_D___:          u32   = 0b_0010001;
        pub const F3_FSGNJ_D:         u32   = 0b_000;
        pub const F3_FSGNJN_D:        u32   = 0b_001;
        pub const F3_FSGNJX_D:        u32   = 0b_010;
    pub const F7_FSMM_D___:           u32   = 0b_0010101;
        pub const F3_FMIN_D:          u32   = 0b_000;
        pub const F3_FMAX_D:          u32   = 0b_001;
    pub const F7_FCVT_S_D___:         u32   = 0b_0100000;
        pub const RS2_FCVT_S_D:       usize = 0b_00001;
    pub const F7_FCVT_D_S___:         u32   = 0b_0100001;
        pub const RS2_FCVT_D_S:       usize = 0b_00000;
    pub const F7_F_QLT_D___:          u32   = 0b_1010001;
        pub const F3_FEQ_D:           u32   = 0b_010;
        pub const F3_FLT_D:           u32   = 0b_001;
        pub const F3_FLE_D:           u32   = 0b_000;
    pub const F3_FCLASS_D___:         u32   = 0b_001;
        pub const F7_FCLASS_D___:     u32   = 0b_1110001;
            pub const RS2_FCLASS_D:   usize = 0b_00000;
    pub const F7_FCVT1___:            u32   = 0b_1100001;
        pub const RS2_FCVT_W_D:       usize = 0b_00000;
        pub const RS2_FCVT_WU_D:      usize = 0b_00001;
    pub const F7_FCVT2___:            u32   = 0b_1101001;
        pub const RS2_FCVT_D_W:       usize = 0b_00000;
        pub const RS2_FCVT_D_WU:      usize = 0b_00001;


// RV64D Standard Extension (in addition to RV32D)
// OP_RV_F___
    // F7_FCVT1___
        pub const RS2_FCVT_L_D:       usize = 0b_00010;
        pub const RS2_FCVT_LU_D:      usize = 0b_00011;
    pub const F3_FMV_X_D___:          u32   = 0b_010;
        pub const F7_FMV_X_D___:      u32   = 0b_1110001;
            pub const RS2_FMV_X_D:    usize = 0b_00000;
    // F7_FCVT2___
        pub const RS2_FCVT_D_L:       usize = 0b_00010;
        pub const RS2_FCVT_D_LU:      usize = 0b_00011;
    pub const F7_FMV_D_X___:          u32   = 0b_1111001;
        pub const RS2_FMV_D_X:        usize = 0b_00000;


// "Q" Standard Extension for Quad-Precision Floating-Point, Version 2.2


// RVWMO Memory Consistency Model, Version 2.0


// "L" Standard Extension for Decimal Floating-Point, Version 0.0


// "C" Standard Extension for Compressed Instructions, Version 2.0


// "B" Standard Extension for Bit Manipulation, Version 0.0


// "J" Standard Extension for Dynamically Translated Languages, Version 0.0


// "T" Standard Extension for Transactional Memory, Version 0.0


// "P" Standard Extension for Packed-SIMD Instructions, Version 0.2


// "V" Standard Extension for Vector Operations, Version 0.7


// "Zam" Standard Extension for Misaligned Atomics, v0.1


// "Ztso" Standard Extension for Total Store Ordering, v0.1




// Privileged Architecture ISAs




// Machine-Level ISA, Version 1.12


// Supervisor-Level ISA, Version 1.12


// Hypervisor Extension, Version 0.6.1
