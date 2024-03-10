

/// Represents the CPU registers for the Gameboy
///
/// The Gameboy CPU has 8 8-bit registers, often used in pairs to represent 16-bit registers.
/// The registers are essential for storing data, performing arithmetic, and more.
///
struct Registers {
    a: u8, // u8 is an unsigned 8-bit integer
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

/// The FlagsRegister, representing the "f" register on the Gameboy CPU, flags certain states.
/// The lower 4 bits of the register are unused, and the upper 4 bits represent the flags.
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

/// By using the standard library of rust, the functions easily convert our FlagsRegister
/// from a u8 and back.
impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       {1} else {0}) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   {1} else {0}) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry {1} else {0}) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      {1} else {0}) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero        = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract    = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry  = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry       = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

// REGISTERS //
impl Registers {
    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }
    fn get_bc(&self) -> u16 { // & means we are passing a reference to self. -> u16 means we are returning a u16.
        (self.b as u16) << 8 | (self.c as u16)
        // << 8 bitwise left shift 8 bits. AKA moving the 8 bits of the register b to the left by 8 bits.
        // | is a bitwise OR. AKA combining the 8 bits of register b with the 8 bits of register c.
    }

    fn set_bc(&mut self, value: u16) { // &mut means we are passing a mutable reference to self.
        self.b = ((value & 0xFF00) >> 8) as u8; // >> 8 is a bitwise right shift 8 bits.
        // & 0xFF00 is a bitwise AND. AKA we are only keeping the 8 bits
        // of the value that are to the left of the 8 bits of the register b.
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}