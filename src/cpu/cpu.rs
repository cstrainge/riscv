
use std::convert::TryInto;
use super::{ opcodes::*, instruction::Instruction };


pub const IALIGN: u32 = 32;
pub const ILEN: u32 = 32;
pub const XLEN: u32 = 64;


pub enum PrivilegeLvel
{
    User       = 0b_00,
    Supervisor = 0b_01,
    Machine    = 0b_11
}


// TODO: Look at implementing memory as u32s and possibly disabling misaligned reads/writes.


pub struct Cpu
{
    pub regs: [u64; 31],
    pub csrs: [u64; 4096],
    pub pc: usize,
    pub memory: Vec<u8>
}


impl Cpu
{
    pub fn new(binary: Vec<u8>) -> Self
    {
        Self
        {
            regs: [0; 31],
            csrs: [0; 4096],
            pc: 0,
            memory: binary
        }
    }


    fn read_u8(&self, address: usize) -> u8
    {
        self.memory[address]
    }


    fn read_u16(&self, address: usize) -> u16
    {
        let slice: &[u8; 2] = &self.memory[address..2].try_into().unwrap_or([0; 2]);
        u16::from_le_bytes(*slice)
    }


    fn read_u32(&self, address: usize) -> u32
    {
        let slice: &[u8; 4] = &self.memory[address..4].try_into().unwrap_or([0; 4]);
        u32::from_le_bytes(*slice)
    }


    fn read_u64(&self, address: usize) -> u64
    {
        let slice: &[u8; 8] = &self.memory[address..8].try_into().unwrap_or([0; 8]);
        u64::from_le_bytes(*slice)
    }


    fn write_u8(&mut self, address: usize, value: u8)
    {
        self.memory[address] = value;
    }


    fn write_u16(&mut self, address: usize, value: u16)
    {
        let bytes = value.to_le_bytes();

        self.memory[address    ] = bytes[0];
        self.memory[address + 1] = bytes[1];
    }


    fn write_u32(&mut self, address: usize, value: u32)
    {
        let bytes = value.to_le_bytes();

        self.memory[address    ] = bytes[0];
        self.memory[address + 1] = bytes[1];
        self.memory[address + 2] = bytes[2];
        self.memory[address + 3] = bytes[3];
    }


    fn write_u64(&mut self, address: usize, value: u64)
    {
        let bytes = value.to_le_bytes();

        self.memory[address    ] = bytes[0];
        self.memory[address + 1] = bytes[1];
        self.memory[address + 2] = bytes[2];
        self.memory[address + 3] = bytes[3];
        self.memory[address + 4] = bytes[4];
        self.memory[address + 5] = bytes[5];
        self.memory[address + 6] = bytes[6];
        self.memory[address + 7] = bytes[7];
    }



    fn read_gp_reg(&self, index: usize) -> u64
    {
        if index == 0
        {
            0
        }
        else
        {
            self.regs[index - 1]
        }
    }


    fn write_gp_reg(&mut self, index: usize, value: u64)
    {
        if index != 0
        {
            self.regs[index - 1] = value;
        }
    }


    fn values_from_registers(&self, instruction: &Instruction) -> ( u64, u64 )
    {
        ( self.read_gp_reg(instruction.rs1), self.read_gp_reg(instruction.rs2) )
    }


    fn ivalues_from_registers(&self, instruction: &Instruction) -> ( i64, i64 )
    {
        ( self.read_gp_reg(instruction.rs1) as i64, self.read_gp_reg(instruction.rs2) as i64 )
    }


    fn u32_values_from_registers(&self, instruction: &Instruction) -> ( u32, u32 )
    {
        ( self.read_gp_reg(instruction.rs1) as u32, self.read_gp_reg(instruction.rs2) as u32 )
    }


    fn i32_values_from_registers(&self, instruction: &Instruction) -> ( i32, i32 )
    {
        ( self.read_gp_reg(instruction.rs1) as i32, self.read_gp_reg(instruction.rs2) as i32 )
    }


    fn address_from_bt(&self, base: usize, instruction: &Instruction) -> usize
    {
        let base = base as u64;
        let offset = instruction.bt_immediate() as i64;
        let address = if offset >= 0
            {
                base + offset as u64
            }
            else
            {
                base - (offset.abs() as u64)
            };

        address as usize
    }


    fn address_from_it(&self, instruction: &Instruction) -> usize
    {
        let base = self.read_gp_reg(instruction.rs1);
        let offset = instruction.it_immediate() as i64;
        let address = if offset >= 0
            {
                base + offset as u64
            }
            else
            {
                base - (offset.abs() as u64)
            };

        address as usize
    }


    fn address_from_st(&self, instruction: &Instruction) -> usize
    {
        let base = self.read_gp_reg(instruction.rs1);
        let offset = instruction.st_immediate() as i64;
        let address = if offset >= 0
            {
                base + offset as u64
            }
            else
            {
                base - (offset.abs() as u64)
            };

        address as usize
    }


    fn address_from_jt(&self, instruction: &Instruction) -> usize
    {
        let base = self.pc as u64 - 4;
        let offset = instruction.jt_immediate() as i64;
        let address = if offset >= 0
            {
                base + offset as u64
            }
            else
            {
                base - (offset.abs() as u64)
            };

        address as usize
    }


    fn address_from_ut(&self, instruction: &Instruction) -> usize
    {
        let base = self.pc as u64 - 4;
        let offset = instruction.ut_immediate() as i64;
        let address = if offset >= 0
            {
                base + offset as u64
            }
            else
            {
                base - offset.abs() as u64
            };

        address as usize
    }


    pub fn fetch(&self) -> Instruction
    {
        // TODO: self.pc must be 32-bit aligned addresses, instruction-address-misaligned exception.
        Instruction::new(self.read_u32(self.pc))
    }


    pub fn execute(&mut self, instruction: &Instruction)
    {
        match ( instruction.func7, instruction.func3, instruction.opcode )
        {
            // RV32D Standard Extension

            // fld  i-type
            ( F3_FLD, _, OP_FLW ) =>
                {
                },

            // fsd  s-type
            ( F3_FSD, _, OP_FSW ) =>
                {
                },

            // fmadd.d  r4-type
            ( func7, _, OP_FMADD_S___ ) if (func7 & RS3_MASK) == F7_FMADD_D =>
                {
                },

            // fmsub.d  r4-type
            ( func7, _, OP_FMSUB_S___ ) if (func7 & RS3_MASK) == F7_FMSUB_D =>
                {
                },

            // fnmsub.d  r4-type
            ( func7, _, OP_FNMSUB_S___ ) if (func7 & RS3_MASK) == F7_FNMSUB_D =>
                {
                },

            // fnmadd.d  r4-type
            ( func7, _, OP_FNMADD_S___ ) if (func7 & RS3_MASK) == F7_FNMADD_D =>
                {
                },

            // fadd.d  r-type
            ( F7_FADD_D, _, OP_RV_F___ ) =>
            {
            },

            // fsub.d  r-type
            ( F7_FSUB_D, _, OP_RV_F___ ) =>
            {
            },

            // fmul.d  r-type
            ( F7_FMUL_D, _, OP_RV_F___ ) =>
            {
            },

            // fdiv.d  r-type
            ( F7_FDIV_D, _, OP_RV_F___ ) =>
            {
            },

            // fsqrt.d  r-type
            ( F7_FSQRT_D, _, OP_RV_F___ ) if instruction.rs2 == RS2_F7_FSQRT_D =>
            {
            },

            // fsgnj.d  r-type
            ( F7_FSGNJ_D___, F3_FSGNJ_D, OP_RV_F___ ) =>
            {
            },

            // fsgnjn.d  r-type
            ( F7_FSGNJ_D___, F3_FSGNJN_D, OP_RV_F___ ) =>
            {
            },

            // fsgnjx.d  r-type
            ( F7_FSGNJ_D___, F3_FSGNJX_D, OP_RV_F___ ) =>
            {
            },

            // fmin.d  r-type
            ( F7_FSMM_D___, F3_FMIN_D, OP_RV_F___ ) =>
            {
            },

            // fmax.d  r-type
            ( F7_FSMM_D___, F3_FMAX_D, OP_RV_F___ ) =>
            {
            },

            // fcvt.s.d  r-type
            ( F7_FCVT_S_D___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_S_D =>
            {
            },

            // fcvt.d.s  r-type
            ( F7_FCVT_D_S___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_D_S =>
            {
            },

            // feq.d  r-type
            ( F7_F_QLT_D___, F3_FEQ_D, OP_RV_F___ ) =>
            {
            },

            // flt.d  r-type
            ( F7_F_QLT_D___, F3_FLT_D, OP_RV_F___ ) =>
            {
            },

            // fle.d  r-type
            ( F7_F_QLT_D___, F3_FLE_D, OP_RV_F___ ) =>
            {
            },

            // fclass.d  r-type
            ( F7_FCLASS_D___, F3_FCLASS_D___, OP_RV_F___ ) if instruction.rs2 == RS2_FCLASS_D =>
            {
            },

            // fcvt.w.d  r-type
            ( F7_FCVT1___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_W_D =>
            {
            },

            // fcvt.wu.d  r-type
            ( F7_FCVT1___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_WU_D =>
            {
            },

            // fcvt.d.w  r-type
            ( F7_FCVT2___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_D_W =>
            {
            },

            // fcvt.d.wu  r-type
            ( F7_FCVT2___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_D_WU =>
            {
            },


            // RV64D Standard Extension (in addition to RV32D)

            // fcvt.l.d  r-type
            ( F7_FCVT1___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_L_D =>
            {
            },

            // fcvt.lu.d  r-type
            ( F7_FCVT1___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_LU_D =>
            {
            },

            // fmv.x.d  r-type
            ( F7_FMV_X_D___, F3_FMV_X_D___, OP_RV_F___ ) if instruction.rs2 == RS2_FMV_X_D =>
            {
            },

            // fcvt.d.l  r-type
            ( F7_FCVT2___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_D_L =>
            {
            },

            // fcvt.d.lu  r-type
            ( F7_FCVT2___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_D_LU =>
            {
            },

            // fmv.d.x  r-type
            ( F7_FMV_D_X___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FMV_D_X =>
            {
            },



            // RV32F Standard Extension

            // flw  i-type
            ( _, _, OP_FLW ) =>
                {
                },

            // fsw  s-type
            ( _, _, OP_FSW ) =>
                {
                },

            // fmadd.s  r4-type
            ( func7, _, OP_FMADD_S___ ) if (func7 & RS3_MASK) == F7_FMADD_S =>
                {
                },

            // fmsub.s  r4-type
            ( func7, _, OP_FMSUB_S___ ) if (func7 & RS3_MASK) == F7_FMSUB_S =>
                {
                },

            // fnmsub.s  r4-type
            ( _, _, OP_FNMSUB_S___ ) if (instruction.func7 & RS3_MASK) == F7_FNMSUB_S =>
                {
                },

            // fnmadd.s  r4-type
            ( _, _, OP_FNMADD_S___ ) if (instruction.func7 & RS3_MASK) == F7_FNMADD_S =>
                {
                },

            // fadd.s  r-type
            ( F7_FADD_S, _, OP_RV_F___ ) =>
                {
                },

            // fsub.s  r-type
            ( F7_FSUB_S, _, OP_RV_F___ ) =>
                {
                },

            // fmul.s  r-type
            ( F7_FMUL_S, _, OP_RV_F___ ) =>
                {
                },

            // fdiv.s  r-type
            ( F7_FDIV_S, _, OP_RV_F___ ) =>
                {
                },

            // fsqrt.s  r-type
            ( F7_FSQRT_S, _, OP_RV_F___ ) if instruction.rs2 == RS2_F7_FSQRT_S =>
                {
                },

            // fsgnj.s  r-type
            ( F7_FSGNJ___, F3_FSGNJ_S, OP_RV_F___ ) =>
                {
                },

            // fsgnjn.s  r-type
            ( F7_FSGNJ___, F3_FSGNJN_S, OP_RV_F___ ) =>
                {
                },

            // fsgnjx.s  r-type
            ( F7_FSGNJ___, F3_FSGNJX_S, OP_RV_F___ ) =>
                {
                },

            // fmin.s  r-type
            ( F7_FSMM___, F3_FMIN_S, OP_RV_F___ ) =>
                {
                },

            // fmax.s  r-type
            ( F7_FSMM___, F3_FMAX_S, OP_RV_F___ ) =>
                {
                },

            // fcvt.w.s  r-type
            ( F7_FCVT_W___, F3_FCVT_W_S, OP_RV_F___ ) =>
                {
                },

            // fcvt.wu.s  r-type
            ( F7_FCVT_W___, F3_FCVT_WU_S, OP_RV_F___ ) =>
                {
                },

            // fmv.x.w  r-type
            ( F7_FMV_X_W, F3_32_FMV___, OP_RV_F___ ) if instruction.rs2 == RS2_FMV_X_W =>
                {
                },

            // feq.s  r-type
            ( F7_FLE_S, F3_32_FMV___, OP_RV_F___ ) =>
                {
                },

            // flt.s  r-type
            ( F7_FMV_W_X, F3_32_FMV___, OP_RV_F___ ) if instruction.rs2 == RS2_FMV_W_X =>
                {
                },

            // fle.s  r-type
            ( F7_FEQ_S, F3_32_FE___, OP_RV_F___ ) =>
                {
                },

            // fclass.s  r-type
            ( F7_FLT_S, F3_32_FLT___, OP_RV_F___ ) =>
                {
                },

            // fcvt.s.w  r-type
            ( F7_FCLASS_S, F3_32_FC___, OP_RV_F___ ) if instruction.rs2 == RS2_FCLASS_S =>
                {
                },

            // fcvt.s.wu  r-type
            ( F7_FCVT_S___, F3_FCVT_S_W, OP_RV_F___ ) =>
                {
                },

            // fmv.w.x  r-type
            ( F7_FCVT_S___, F3_FCVT_S_WU, OP_RV_F___ ) =>
                {
                },

            // RV64F Standard Extension (in addition to RV32F)

            // fcvt.l.s  * r-type

            ( F7_FCVT_W___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_L_S =>
                {
                },

            // fcvt.lu.s  * r-type

            ( F7_FCVT_W___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_LU_S =>
                {
                },

            // fcvt.s.l  * r-type

            ( F7_FCVT_S___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_S_L =>
                {
                },

            // fcvt.s.lu  * r-type

            ( F7_FCVT_S___, _, OP_RV_F___ ) if instruction.rs2 == RS2_FCVT_S_LU =>
                {
                },



            // "A" Standard Extension for Atomic Instructions, Version 2.1

            // RV32A Standard Extension

            // lr.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if    (func7 & EXT_A_F7_MASK == F7_LR_W___)
                                                     && (instruction.rs2 == RS2_LR_W) =>
                {
                },

            // sc.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_SC_W =>
                {
                },

            // amoswap.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOSWAP_W =>
                {
                },

            // amoadd.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOADD_W =>
                {
                },

            // amoxor.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOXOR_W =>
                {
                },

            // amoand.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOAND_W =>
                {
                },

            // amoor.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOOR_W =>
                {
                },

            // amomin.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOMIN_W =>
                {
                },

            // amomax.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOMAX_W =>
                {
                },

            // amominu.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOMINU_W =>
                {
                },

            // amomaxu.w  r-type
            ( func7, F3_EXT_A32___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOMAXU_W =>
                {
                },

            // RV64A Standard Extension (in addition to RV32A)

            // lr.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if    (func7 & EXT_A_F7_MASK == F7_LR_D___)
                                                     && (instruction.rs2 == RS2_LR_D) =>
                {
                },

            // sc.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_SC_D =>
                {
                },

            // amoswap.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOSWAP_D =>
                {
                },

            // amoadd.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOADD_D =>
                {
                },

            // amoxor.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOXOR_D =>
                {
                },

            // amoand.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOAND_D =>
                {
                },

            // amoor.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOOR_D =>
                {
                },

            // amomin.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOMIN_D =>
                {
                },

            // amomax.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOMAX_D =>
                {
                },

            // amominu.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOMINU_D =>
                {
                },

            // amomaxu.d  r-type
            ( func7, F3_EXT_A64___, OP_EXT_A___ ) if  func7 & EXT_A_F7_MASK == F7_AMOMAXU_D =>
                {
                },



            // RV64M Standard Extension (in addition to RV32M)

            // mulw  r-type
            ( F7_M_EXT, F3_64_MULW___, OP_MO4___ ) =>
                {
                    let ( rs1, rs2 ) = self.i32_values_from_registers(instruction);
                    self.write_gp_reg(instruction.rd, rs1.wrapping_mul(rs2) as u64);
                },

            // divw  r-type
            ( F7_M_EXT, F3_64_DIVW___, OP_MO4___ ) =>
                {
                    let ( rs1, rs2 ) = self.i32_values_from_registers(instruction);
                    let result = if rs2 == 0
                        {
                            -1
                        }
                        else
                        {
                            rs1.wrapping_div(rs2)
                        };

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },

            // divuw  r-type
            ( F7_M_EXT, F3_64_DIVUW___, OP_MO4___ ) =>
                {
                    let ( rs1, rs2 ) = self.u32_values_from_registers(instruction);
                    let result = if rs2 == 0
                        {
                            0x_ffffffff
                        }
                        else
                        {
                            rs1.wrapping_div(rs2)
                        };

                    self.write_gp_reg(instruction.rd, result as u64);
                },

            // remw  r-type
            ( F7_M_EXT, F3_64_REMW___, OP_MO4___ ) =>
                {
                    let ( rs1, rs2 ) = self.i32_values_from_registers(instruction);
                    let result = if rs2 == 0
                        {
                            -1
                        }
                        else
                        {
                            rs1.wrapping_rem(rs2)
                        };

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },

            // remuw  r-type
            ( F7_M_EXT, F3_64_REMUW___, OP_MO4___ ) =>
                {
                    let ( rs1, rs2 ) = self.u32_values_from_registers(instruction);
                    let result = if rs2 == 0
                        {
                            0x_ffffffff
                        }
                        else
                        {
                            rs1.wrapping_rem(rs2)
                        };

                    self.write_gp_reg(instruction.rd, result as u64);
                },



            // RV32M Standard Extension

            // mul  r-type
            ( F7_M_EXT, F3_32_MUL___, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    self.write_gp_reg(instruction.rd, rs1.wrapping_mul(rs2));
                },

            // mulh  r-type
            ( F7_M_EXT, F3_32_MULH___, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    let result = ((rs1 as i128).wrapping_mul(rs2 as i128) >> 64) as u64;

                    self.write_gp_reg(instruction.rd, result);
                },

            // mulhsu  r-type
            ( F7_M_EXT, F3_32_MULHSU___, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.ivalues_from_registers(instruction);
                    let result = (if rs1 < 0
                        {
                            (rs1.abs() as i128 as u128).wrapping_mul(rs2 as u128)
                        }
                        else
                        {
                            (rs1 as u128).wrapping_mul(rs2 as u128)
                        }
                        >> 64) as u64;

                    self.write_gp_reg(instruction.rd, result);
                },

            // mulhu  r-type
            ( F7_M_EXT, F3_32_MULHU___, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    let result = ((rs1 as u128).wrapping_mul(rs2 as u128) >> 64) as u64;

                    self.write_gp_reg(instruction.rd, result);
                },

            // div  r-type
            ( F7_M_EXT, F3_32_DIV___, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.ivalues_from_registers(instruction);
                    let result = if rs2 == 0
                        {
                            u64::MAX as i64
                        }
                        else
                        {
                            rs1.wrapping_div(rs2)
                        };

                    self.write_gp_reg(instruction.rd, result as u64);
                },

            // divu  r-type
            ( F7_M_EXT, F3_32_DIVU___, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    let result = if rs2 == 0
                        {
                            u64::MAX
                        }
                        else
                        {
                            rs1.wrapping_div(rs2)
                        };

                    self.write_gp_reg(instruction.rd, result);
                },

            // rem  r-type
            ( F7_M_EXT, F3_32_REM___, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.ivalues_from_registers(instruction);
                    let result = if rs2 == 0
                        {
                            0
                        }
                        else
                        {
                            rs1.wrapping_rem(rs2)
                        };

                    self.write_gp_reg(instruction.rd, result as u64);
                },

            // remu  r-type
            ( F7_M_EXT, F3_32_REMU___, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    let result = if rs2 == 0
                        {
                            0
                        }
                        else
                        {
                            rs1.wrapping_rem(rs2)
                        };

                    self.write_gp_reg(instruction.rd, result);
                },



            // RV64I Base Instruction Set (in addition to RV32I)

            // lwu  i-type
            ( _, F3_LWU, OP_LD___ ) =>
                {
                    let address = self.address_from_it(instruction);
                    self.write_gp_reg(instruction.rd, self.read_u32(address) as u32 as u64);
                },

            // ld  i-type
            ( _, F3_LD, OP_LD___ ) =>
                {
                    let address = self.address_from_it(instruction);
                    self.write_gp_reg(instruction.rd, self.read_u64(address));
                },

            // sd  s-type
            ( _, F3_SD, OP_ST___ ) =>
                {
                    let address = self.address_from_st(instruction);
                    self.write_u64(address, self.read_gp_reg(instruction.rs2));
                },

            // addiw  i-type
            ( _, F3_ADDIW, OP_MO3___ ) =>
                {
                    let immediate = instruction.it_immediate();
                    let rs1 = self.read_gp_reg(instruction.rs1);

                    let result = if immediate != 0
                        {
                            rs1.wrapping_add(immediate) as i32
                        }
                        else
                        {
                            (rs1 & 0x_00000000_ffffffff) as i32
                        };

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },

            // slliw  r-type
            ( F7_SLLIW, F3_SHL___, OP_MO3___ ) =>
                {
                    let shift = instruction.rs2;
                    let rs1 = self.read_gp_reg(instruction.rs1);
                    let result = (rs1 << shift) as i32;

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },

            // srliw  r-type
            ( F7_SRLIW, F3_SHR___, OP_MO3___ ) =>
                {
                    let shift = instruction.rs2;
                    let rs1 = self.read_gp_reg(instruction.rs1);
                    let result = (rs1 >> shift) as i32;

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },

            // sraiw  r-type
            ( F7_SRAIW, F3_SHR___, OP_MO3___ ) =>
                {
                    let shift = instruction.rs2;
                    let rs1 = self.read_gp_reg(instruction.rs1) as i64;
                    let result = (rs1 >> shift) as i32;

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },

            // addw  r-type
            ( F7_ADDW, F3_ASW___, OP_MO4___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    let result = rs1.wrapping_add(rs2) as i32;

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },

            // subw  r-type
            ( F7_SUBW, F3_ASW___, OP_MO4___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    let result = rs1.wrapping_sub(rs2) as i32;

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },

            // sllw  r-type
            ( F7_SLLW, F3_SLLW___, OP_MO4___ ) =>
                {
                    let ( value, shift ) = self.values_from_registers(instruction);
                    let shift = shift & 0b_011111;
                    let result = (value << shift) as i32;

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },

            // srlw  r-type
            ( F7_SRLW, F3_SHR___, OP_MO4___ ) =>
                {
                    let ( value, shift ) = self.values_from_registers(instruction);
                    let shift = shift & 0b_011111;
                    let result = (value >> shift) as i32;

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },

            // sraw  r-type
            ( F7_SRAW, F3_SHR___, OP_MO4___ ) =>
                {
                    let ( value, shift ) = self.values_from_registers(instruction);
                    let value = value as i64;
                    let shift = shift & 0b_011111;

                    let result = (value >> shift) as i32;

                    self.write_gp_reg(instruction.rd, result as i64 as u64);
                },



            // RV32I Base Instruction Set, Version 2.1

            // lui  u-type
            ( _, _, OP_LUI ) =>
                {
                    self.write_gp_reg(instruction.rd, instruction.ut_immediate());
                },

            // auipc  u-type
            ( _, _, OP_AUIPC ) =>
                {
                    self.write_gp_reg(instruction.rd, self.address_from_ut(instruction) as u64);
                },

            // jal  j-type
            ( _, _, OP_JAL ) =>
                {
                    self.write_gp_reg(instruction.rd, self.pc as u64);
                    self.pc = self.address_from_jt(instruction);

                    // TODO: Generate a mis-aligned exception if not on a 4-byte boundry.
                },

            // jalr  i-type
            ( _, _, OP_JALR ) =>
                {
                    let address = self.address_from_it(instruction);

                    self.write_gp_reg(instruction.rd, self.pc as u64);
                    self.pc = address & (!1);

                    // TODO: Generate a mis-aligned exception if not on a 4-byte boundry.
                },

            // beq  b-type
            ( _, F3_BEQ, OP_BR___ ) =>
                {
                    let ( rs1, rs2 ) = self.ivalues_from_registers(instruction);

                    if rs1 == rs2
                    {
                        self.pc = self.address_from_bt(self.pc - 4, instruction);
                        // TODO: Generate a mis-aligned exception if not on a 4-byte boundry.
                    }
                },

            // bne  b-type
            ( _, F3_BNE, OP_BR___ ) =>
                {
                    let ( rs1, rs2 ) = self.ivalues_from_registers(instruction);

                    if rs1 != rs2
                    {
                        self.pc = self.address_from_bt(self.pc - 4, instruction);
                        // TODO: Generate a mis-aligned exception if not on a 4-byte boundry.
                    }
                },

            // blt  b-type
            ( _, F3_BLT, OP_BR___ ) =>
                {
                    let ( rs1, rs2 ) = self.ivalues_from_registers(instruction);

                    if rs1 < rs2
                    {
                        self.pc = self.address_from_bt(self.pc - 4, instruction);
                        // TODO: Generate a mis-aligned exception if not on a 4-byte boundry.
                    }
                },

            // bge  b-type
            ( _, F3_BGE, OP_BR___ ) =>
                {
                    let ( rs1, rs2 ) = self.ivalues_from_registers(instruction);

                    if rs1 >= rs2
                    {
                        self.pc = self.address_from_bt(self.pc - 4, instruction);
                        // TODO: Generate a mis-aligned exception if not on a 4-byte boundry.
                    }
                },

            // bltu  b-type
            ( _, F3_BLTU, OP_BR___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);

                    if rs1 < rs2
                    {
                        self.pc = self.address_from_bt(self.pc - 4, instruction);
                        // TODO: Generate a mis-aligned exception if not on a 4-byte boundry.
                    }
                },

            // bgeu  b-type
            ( _, F3_BGEU, OP_BR___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);

                    if rs1 >= rs2
                    {
                        self.pc = self.address_from_bt(self.pc - 4, instruction);
                        // TODO: Generate a mis-aligned exception if not on a 4-byte boundry.
                    }
                },

            // lb  i-type
            ( _, F3_LB, OP_LD___ ) =>
                {
                    let address = self.address_from_it(instruction);
                    self.write_gp_reg(instruction.rd, self.read_u8(address) as i8 as i64 as u64);
                },

            // lh  i-type
            ( _, F3_LH, OP_LD___ ) =>
                {
                    let address = self.address_from_it(instruction);
                    self.write_gp_reg(instruction.rd, self.read_u16(address) as i16 as i64 as u64);
                },

            // lw  i-type
            ( _, F3_LW, OP_LD___ ) =>
                {
                    let address = self.address_from_it(instruction);
                    self.write_gp_reg(instruction.rd, self.read_u32(address) as i32 as i64 as u64);
                },

            // lbu  i-type
            ( _, F3_LBU, OP_LD___ ) =>
                {
                    let address = self.address_from_it(instruction);
                    self.write_gp_reg(instruction.rd, self.read_u8(address) as u64);
                },

            // lhu  i-type
            ( _, F3_LHU, OP_LD___ ) =>
                {
                    let address = self.address_from_it(instruction);
                    self.write_gp_reg(instruction.rd, self.read_u16(address) as u64);
                },

            // sb  s-type
            ( _, F3_SB, OP_ST___ ) =>
                {
                    let address = self.address_from_st(instruction);
                    self.write_u8(address, self.read_gp_reg(instruction.rs2) as u8);
                },

            // sh  s-type
            ( _, F3_SH, OP_ST___ ) =>
                {
                    let address = self.address_from_st(instruction);
                    self.write_u16(address, self.read_gp_reg(instruction.rs2) as u16);
                },

            // sw  s-type
            ( _, F3_SW, OP_ST___ ) =>
                {
                    let address = self.address_from_st(instruction);
                    self.write_u32(address, self.read_gp_reg(instruction.rs2) as u32);
                },

            // addi  i-type
            ( _, F3_ADDI, OP_MO1___ ) =>
                {
                    let immediate = instruction.it_immediate();
                    let rs1 = self.read_gp_reg(instruction.rs1);

                    self.write_gp_reg(instruction.rd, rs1.wrapping_add(immediate));
                },

            // slti  i-type
            ( _, F3_SLTI, OP_MO1___ ) =>
                {
                    let immediate = instruction.it_immediate() as i64;
                    let rs1 = self.read_gp_reg(instruction.rs1) as i64;

                    self.write_gp_reg(instruction.rd, if rs1 < immediate { 1 } else { 0 });
                },

            // sltiu  i-type
            ( _, F3_SLTIU, OP_MO1___ ) =>
                {
                    let immediate = instruction.it_immediate();
                    let rs1 = self.read_gp_reg(instruction.rs1);

                    if rs1 != 1
                    {
                        self.write_gp_reg(instruction.rd, if rs1 < immediate { 1 } else { 0 });
                    }
                    else
                    {
                        self.write_gp_reg(instruction.rd, if rs1 == 0 { 1 } else { 0 });
                    }
                },

            // xori  i-type
            ( _, F3_XORI, OP_MO1___ ) =>
                {
                    let immediate = instruction.it_immediate();
                    let rs1 = self.read_gp_reg(instruction.rs1);

                    self.write_gp_reg(instruction.rd, rs1 ^ immediate);
                },

            // ori  i-type
            ( _, F3_ORI, OP_MO1___ ) =>
                {
                    let immediate = instruction.it_immediate() as u64;
                    let rs1 = self.read_gp_reg(instruction.rs1) as u64;

                    self.write_gp_reg(instruction.rd, rs1 | immediate);
                },

            // andi  i-type
            ( _, F3_ANDI, OP_MO1___ ) =>
                {
                    let immediate = instruction.it_immediate() as u64;
                    let rs1 = self.read_gp_reg(instruction.rs1) as u64;

                    self.write_gp_reg(instruction.rd, rs1 & immediate);
                },

            // slli  r-type
            ( F7_SLLI, F3_SL___, OP_MO1___ ) =>
                {
                    let shift = instruction.rs2;
                    let rs1 = self.read_gp_reg(instruction.rs1) as u64;

                    self.write_gp_reg(instruction.rd, rs1 << shift);
                },

            // srli  r-type
            ( F7_SRLI, F3_SR___, OP_MO1___ ) =>
                {
                    let shift = instruction.rs2;
                    let rs1 = self.read_gp_reg(instruction.rs1) as u64;

                    self.write_gp_reg(instruction.rd, rs1 >> shift);
                },

            // srai  r-type
            ( F7_SRAI, F3_SR___, OP_MO1___ ) =>
                {
                    let shift = instruction.rs2;
                    let rs1 = self.read_gp_reg(instruction.rs1) as i64;

                    self.write_gp_reg(instruction.rd, (rs1 >> shift) as u64);
                },

            // add  r-type
            ( F7_ADD, F3_AS___, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    let result = rs1.wrapping_add(rs2);

                    self.write_gp_reg(instruction.rd, result);
                },

            // sub  r-type
            ( F7_SUB, F3_AS___, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    let result = rs1.wrapping_sub(rs2);

                    self.write_gp_reg(instruction.rd, result);
                },

            // sll  r-type
            ( F7_SLL, F3_SLL, OP_MO2___ ) =>
                {
                    let ( value, shift ) = self.values_from_registers(instruction);
                    let shift = shift & 0b_111111;

                    self.write_gp_reg(instruction.rd, value << shift);
                },

            // slt  r-type
            ( F7_SLT, F3_SLT, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.ivalues_from_registers(instruction);
                    self.write_gp_reg(instruction.rd, if rs1 < rs2 { 1 } else { 0 });
                },

            // sltu  r-type
            ( F7_SLTU, F3_SLTU, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);

                    if instruction.rs1 == 0
                    {
                        self.write_gp_reg(instruction.rd, if  rs2 != 0 { 1 } else { 0 });
                    }
                    else
                    {
                        self.write_gp_reg(instruction.rd, if rs1 < rs2 { 1 } else { 0 });
                    }
                },

            // xor  r-type
            ( F7_XOR, F3_XOR, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    self.write_gp_reg(instruction.rd, rs1 ^ rs2);
                },

            // srl  r-type
            ( F7_SRL, F3_SR2___, OP_MO2___ ) =>
                {
                    let ( value, shift ) = self.values_from_registers(instruction);
                    let shift = shift & 0b_111111;

                    self.write_gp_reg(instruction.rd, value >> shift);
                },

            // sra  r-type
            ( F7_SRA, F3_SR2___, OP_MO2___ ) =>
                {
                    let ( value, shift ) = self.values_from_registers(instruction);

                    let value = value as i64;
                    let shift = shift & 0b_111111;

                    self.write_gp_reg(instruction.rd, (value >> shift) as u64);
                },

            // or  r-type
            ( F7_OR, F3_OR, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    self.write_gp_reg(instruction.rd, rs1 | rs2);
                },

            // and  r-type
            ( F7_AND, F3_AND, OP_MO2___ ) =>
                {
                    let ( rs1, rs2 ) = self.values_from_registers(instruction);
                    self.write_gp_reg(instruction.rd, rs1 & rs2);
                },

            // fence  * i-type
            ( _, F3_FENCE, OP_MISC_MEM___ ) =>
                {
                },

            // ecall  * i-type
            ( _, F3_SYS_PRIV___, OP_SYSTEM___ )
                if instruction.func12 == F12_ECALL =>
                {
                },

            // ebreak  * i-type
            ( _, F3_SYS_PRIV___, OP_SYSTEM___ )
                if instruction.func12 == F12_EBREAK =>
                {
                },


            _ =>
                {
                    dbg!("Opcode unimplemnented.");
                }
        }
    }
}
