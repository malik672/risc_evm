/// Represents a RV64I physical register
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
