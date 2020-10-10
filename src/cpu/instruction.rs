//use std::num::FromPrimitive;

struct FieldMask
{
    mask: u32,
    shift: isize
}


fn decode_immediate_signed(source: u32, fields: &[ FieldMask ], immediate_size: usize) -> u64
{
    let mut immediate = 0;

    // Gather up and construct our value from the fields in the encoded source.
    for field in fields
    {
        if field.shift > 0
        {
            immediate |= (source & field.mask) >> field.shift;
        }
        else
        {
            immediate |= (source & field.mask) << field.shift.abs();
        }
    }


    // Check the sign bit and if set, sign extend to 32-bits.
    let sign_bit = 1 << (immediate_size - 1);

    if (immediate_size < 32) && (immediate & sign_bit) != 0
    {
        let sign_extension = 0b_11111111_11111111_11111111_11111111 << immediate_size;
        immediate |= sign_extension;
    }

    // Since in this case we are 64-bit, extend the rest of the way.
    ((immediate as i32) as i64) as u64
}


#[derive(Debug, Copy, Clone)]
pub struct Instruction
{
    raw_instruction: u32,

    pub opcode: u32,

    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,

    pub func3: u32,
    pub func7: u32,
    pub func12: u32
}


impl Instruction
{
    pub const fn new(raw_instruction: u32) -> Self
    {
        let opcode = raw_instruction & 0b_00000000_00000000_00000000_01111111;

        let rd  =  ((raw_instruction & 0b_00000000_00000000_00001111_10000000) >> 7) as usize;
        let rs1 =  ((raw_instruction & 0b_00000000_00001111_10000000_00000000) >> 15) as usize;
        let rs2 =  ((raw_instruction & 0b_00000001_11110000_00000000_00000000) >> 20) as usize;

        let func3  = (raw_instruction & 0b_00000000_00000000_01110000_00000000) >> 12;
        let func7  = (raw_instruction & 0b_11111110_00000000_00000000_00000000) >> 25;
        let func12 = (raw_instruction & 0b_11111111_11110000_00000000_00000000) >> 21;

        Self { raw_instruction, opcode, rd, rs1, rs2, func3, func7, func12 }
    }


    pub fn it_immediate(&self) -> u64
    {
        let masks = [ FieldMask { mask: 0b_11111111_11110000_00000000_00000000, shift: 20 } ];

        decode_immediate_signed(self.raw_instruction, &masks, 12)
    }


    pub fn st_immediate(&self) -> u64
    {
        let masks = [ FieldMask { mask: 0b_11111110_00000000_00000000_00000000, shift: 20 },
                      FieldMask { mask: 0b_00000000_00000000_00001111_10000000, shift: 7  } ];

        decode_immediate_signed(self.raw_instruction, &masks, 12)
    }

    pub fn bt_immediate(&self) -> u64
    {
        let masks = [ FieldMask { mask: 0b_10000000_00000000_00000000_00000000, shift:  19 },
                      FieldMask { mask: 0b_01111110_00000000_00000000_00000000, shift:  20 },
                      FieldMask { mask: 0b_00000000_00000000_00001111_00000000, shift:  7  },
                      FieldMask { mask: 0b_00000000_00000000_00000000_10000000, shift: -4  } ];

        decode_immediate_signed(self.raw_instruction, &masks, 20)
    }

    pub fn ut_immediate(&self) -> u64
    {
        let masks = [ FieldMask { mask: 0b_11111111_11111111_11110000_00000000, shift: 0 } ];

        decode_immediate_signed(self.raw_instruction, &masks, 32)
    }

    pub fn jt_immediate(&self) -> u64
    {
        let masks = [ FieldMask { mask: 0b_10000000_00000000_00000000_00000000, shift: 10 },
                      FieldMask { mask: 0b_01111111_11100000_00000000_00000000, shift: 21 },
                      FieldMask { mask: 0b_00000000_00010000_00000000_00000000, shift: 9  },
                      FieldMask { mask: 0b_00000000_00001111_11110000_00000000, shift: 0  } ];

        decode_immediate_signed(self.raw_instruction, &masks, 20)
    }
}
