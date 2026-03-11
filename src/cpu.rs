// cpu.rs

use crate::Memory;
pub struct Registers {
    pub a: u8, pub f: u8,  // AF
    pub b: u8, pub c: u8,  // BC
    pub d: u8, pub e: u8,  // DE
    pub h: u8, pub l: u8,  // HL
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    // Access BC as 16-bit
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    pub fn get_zero_flag(&mut self) -> bool {
        (self.f >> 7) & 1 == 1
    }

    pub fn set_zero_flag(&mut self, val: bool) {
        if val {
            self.f |= 0x80; // Set bit 7
        } else {
            self.f &= 0x7F; // Clear bit 7
        }
    }

    pub fn get_subtract_flag(&mut self) -> bool {
        (self.f >> 6) & 1 == 1
    }

    pub fn set_subtract_flag(&mut self, val: bool) {
        if val {
            self.f |= 0x40; // Set bit 6
        } else {
            self.f &= 0xBF; // Clear bit 6
        }
    }

    pub fn get_half_carry_flag(&mut self) -> bool {
        (self.f >> 5) & 1 == 1
    }

    pub fn set_half_carry_flag(&mut self, val: bool) {
        if val {
            self.f |= 0x20; // Set bit 5
        } else {
            self.f &= 0xDF; // Clear bit 5
        }
    }

    pub fn get_carry_flag(&mut self) -> bool {
        (self.f >> 4) & 1 == 1
    }

    pub fn set_carry_flag(&mut self, val: bool) {
        if val {
            self.f |= 0x10; // Set bit 4
        } else {
            self.f &= 0xEF; // Clear bit 4
        }
    }
}

pub struct CPU {
    pub regs: Registers,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            regs: Registers {
                a: 0, f: 0,
                b: 0, c: 0,
                d: 0, e: 0,
                h: 0, l: 0,
                sp: 0xFFFE,
                pc: 0x0100,
            }
        }
    }

// cpu.rs
pub fn step(&mut self, mem: &mut Memory) {
    let opcode = mem.read(self.regs.pc);
    self.regs.pc = self.regs.pc.wrapping_add(1);

    match opcode {
        0x00 => {}                          // NOP
        0x3E => {                           // LD A, n
            self.regs.a = mem.read(self.regs.pc);
            self.regs.pc = self.regs.pc.wrapping_add(1);
        }
        0xAF => {                           // XOR A
            self.regs.a ^= self.regs.a;
            self.regs.f = 0x80;             // set Zero flag
        }
        _ => println!("Unknown opcode: {:#04X}", opcode),
    }
}
}