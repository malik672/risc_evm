//! RISC-V RV64I Register Module

/// Represents a RV64I physical register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register(u8);

// +---------------+-------------------------+-----------------------------------------------------------+
// | Group         | Registers               | Description                                               |
// +---------------+-------------------------+-----------------------------------------------------------+
// | Fixed         | x0 (zero), x1 (ra),     | Reserved for fixed purposes (zero,        |
// |               | x2 (sp), x3 (gp),       | return address, stack pointer).                           |
// |               | x4 (tp)                 |                                                           |
// +---------------+-------------------------+-----------------------------------------------------------+
// | Temporary     | x5-x7 (t0-t2),          | for temporary values, freely overwritten during       |
// |               | x28-x31 (t3-t6)         | execution.                                                |
// +---------------+-------------------------+-----------------------------------------------------------+
// | Saved         | x8 (s0/fp), x9 (s1),    | Registers that preserve values across function calls.      |
// |               | x18-x27 (s2-s11)        |                                                           |
// +---------------+-------------------------+-----------------------------------------------------------+
// | Argument      | x10-x17 (a0-a7)         | Registers used for passing function arguments and          |
// |               |                         | returning values.                                         |
// +---------------+-------------------------+-----------------------------------------------------------+
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegKind {
    Fixed,      
    Temporary,  
    Saved, 
    Argument,   
}

impl Register {
    /// Create a new register from raw index (0-31)
    pub fn from_raw(index: u8) -> Self {
        Register(index)
    }

    /// Get raw register number
    pub fn raw(&self) -> u8 {
        self.0
    }

    /// Get register kind
    pub fn kind(&self) -> RegKind {
        match self.0 {
            0..=4 => RegKind::Fixed,
            5..=7 | 28..=31 => RegKind::Temporary,
            8..=9 | 18..=27 => RegKind::Saved,
            10..=17 => RegKind::Argument,
            _ => unreachable!(),
        }
    }

    /// Check if register is callee-saved
    pub fn is_callee_saved(&self) -> bool {
        matches!(self.kind(), RegKind::Saved)
    }

    /// Get ABI name of register
    pub fn abi_name(&self) -> &'static str {
        match self.0 {
            0 => "zero",
            1 => "ra",
            2 => "sp",
            3 => "gp",
            4 => "tp",
            5 => "t0",
            6 => "t1",
            7 => "t2",
            8 => "s0/fp",
            9 => "s1",
            10 => "a0",
            11 => "a1", 
            12 => "a2",
            13 => "a3",
            14 => "a4", 
            15 => "a5",
            16 => "a6",
            17 => "a7",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "s8",
            25 => "s9",
            26 => "s10",
            27 => "s11",
            28 => "t3",
            29 => "t4",
            30 => "t5",
            31 => "t6",
            _ => unreachable!(),
        }
    }
}

impl Register {
    // Fixed registers
    pub const ZERO: Register = Register(0);  // x0: 
    pub const RA: Register = Register(1);    // x1: 
    pub const SP: Register = Register(2);    // x2: 
    pub const GP: Register = Register(3);    // x3: 
    pub const TP: Register = Register(4);    // x4: 

    // Temporary registers (caller-saved)
    pub const T0: Register = Register(5);    // x5
    pub const T1: Register = Register(6);    // x6
    pub const T2: Register = Register(7);    // x7
    pub const T3: Register = Register(28);   // x28
    pub const T4: Register = Register(29);   // x29
    pub const T5: Register = Register(30);   // x30
    pub const T6: Register = Register(31);   // x31

    // Saved registers (callee-saved)
    pub const S0: Register = Register(8);    // x8: 
    pub const FP: Register = Register(8);    // x8: 
    pub const S1: Register = Register(9);    // x9
    pub const S2: Register = Register(18);   // x18
    pub const S3: Register = Register(19);   // x19
    pub const S4: Register = Register(20);   // x20
    pub const S5: Register = Register(21);   // x21
    pub const S6: Register = Register(22);   // x22
    pub const S7: Register = Register(23);   // x23
    pub const S8: Register = Register(24);   // x24
    pub const S9: Register = Register(25);   // x25
    pub const S10: Register = Register(26);  // x26
    pub const S11: Register = Register(27);  // x27

    // Argument registers (caller-saved)
    pub const A0: Register = Register(10);   // x10: 
    pub const A1: Register = Register(11);   // x11: 
    pub const A2: Register = Register(12);   // x12
    pub const A3: Register = Register(13);   // x13
    pub const A4: Register = Register(14);   // x14
    pub const A5: Register = Register(15);   // x15
    pub const A6: Register = Register(16);   // x16
    pub const A7: Register = Register(17);   // x17

}