/// Represents a RV64I physical register
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_creation() {
        let reg = Register::from_raw(5);
        assert_eq!(reg.raw(), 5);
    }

    #[test]
    fn test_register_kinds() {
     
        for i in 0..=4 {
            assert_eq!(Register::from_raw(i).kind(), RegKind::Fixed);
        }

      
        for i in 5..=7 {
            assert_eq!(Register::from_raw(i).kind(), RegKind::Temporary);
        }
        for i in 28..=31 {
            assert_eq!(Register::from_raw(i).kind(), RegKind::Temporary);
        }


        for i in 8..=9 {
            assert_eq!(Register::from_raw(i).kind(), RegKind::Saved);
        }
        for i in 18..=27 {
            assert_eq!(Register::from_raw(i).kind(), RegKind::Saved);
        }

        for i in 10..=17 {
            assert_eq!(Register::from_raw(i).kind(), RegKind::Argument);
        }
    }

    #[test]
    fn test_callee_saved() {
       
        assert!(Register::from_raw(8).is_callee_saved());
        assert!(Register::from_raw(9).is_callee_saved());
        for i in 18..=27 {
            assert!(Register::from_raw(i).is_callee_saved());
        }

        // Other registers should not be callee- saved
        assert!(!Register::from_raw(0).is_callee_saved()); 
        assert!(!Register::from_raw(5).is_callee_saved()); 
        assert!(!Register::from_raw(10).is_callee_saved()); 
    }

    #[test]
    fn test_abi_names() {
        // Test fixed registers
        assert_eq!(Register::from_raw(0).abi_name(), "zero");
        assert_eq!(Register::from_raw(1).abi_name(), "ra");
        assert_eq!(Register::from_raw(2).abi_name(), "sp");
        assert_eq!(Register::from_raw(3).abi_name(), "gp");
        assert_eq!(Register::from_raw(4).abi_name(), "tp");

        // Test some temporaries
        assert_eq!(Register::from_raw(5).abi_name(), "t0");
        assert_eq!(Register::from_raw(28).abi_name(), "t3");

        // Test some saved registers
        assert_eq!(Register::from_raw(8).abi_name(), "s0/fp");
        assert_eq!(Register::from_raw(18).abi_name(), "s2");

        // Test some argument registers
        assert_eq!(Register::from_raw(10).abi_name(), "a0");
        assert_eq!(Register::from_raw(17).abi_name(), "a7");
    }

    #[test]
    #[should_panic]
    fn test_invalid_register() {
        Register::from_raw(32).kind();
    }
}