use std::collections::HashMap;
use super::register::{RegKind, Register};



#[derive(Debug)]
pub struct RegisterAllocator {
    // maps stack positions to physical registers
    allocated: HashMap<usize, Register>,
    available_temp: Vec<Register>,
    available_saved: Vec<Register>,
    callee_saved: Vec<Register>,
}

#[derive(Debug)]
pub struct SpillError {
    stack_pos: usize,
}


impl RegisterAllocator {
    pub fn new() -> Self {
        let mut allocator = RegisterAllocator {
            allocated: HashMap::new(),
            available_temp: Vec::with_capacity(7),
            available_saved: Vec::with_capacity(10),
            callee_saved: Vec::new(),
        };

        // Initialize temporary registers (t0-t2, t3-t6)
        for i in 5..=7 {
            allocator.available_temp.push(Register::from_raw(i));
        }
        for i in 28..=31 {
            allocator.available_temp.push(Register::from_raw(i));
        }

        // Initialize saved registers (s2-s11)
        for i in 18..=27 {
            allocator.available_saved.push(Register::from_raw(i));
        }

        allocator
    }

    /// Get a register for a stack position, allocating if necessary
    pub fn get_register(&mut self, stack_pos: usize) -> Register {
        if let Some(&reg) = self.allocated.get(&stack_pos) {
            reg
        } else {
            self.allocate(stack_pos)
        }
    }

    /// Allocate a register for a stack position
    pub fn allocate(&mut self, stack_pos: usize) -> Register {
        // First try temporary registers
        let reg = match self.available_temp.pop() {
            Some(reg) => reg,
            None => match self.available_saved.pop() {
                Some(reg) => {
                    self.callee_saved.push(reg);
                    reg
                }
                None => {
                    panic!("can't allocate to register")
                }
            }
        };
        
        self.allocated.insert(stack_pos, reg);
        reg
    }

    /// Free a register allocated to a stack position
    pub fn free(&mut self, stack_pos: usize) {
        if let Some(reg) = self.allocated.remove(&stack_pos) {
            match reg.kind() {
                RegKind::Temporary => {
                    self.available_temp.push(reg);
                }
                RegKind::Saved => {
                    self.callee_saved.retain(|r| *r != reg);
                    self.available_saved.push(reg);
                }
                _ => {} // Fixed and argument registers aren't managed here
            }
        }
    }

    /// Get list of currently used callee-saved registers
    pub fn get_callee_saved(&self) -> &[Register] {
        &self.callee_saved
    }

    /// Clear all allocations (useful between blocks)
    pub fn clear_allocations(&mut self) {
        // Return all allocated registers to their pools
        for reg in self.allocated.values() {
            match reg.kind() {
                RegKind::Temporary => self.available_temp.push(*reg),
                RegKind::Saved => self.available_saved.push(*reg),
                _ => {}
            }
        }
        self.allocated.clear();
        self.callee_saved.clear();
    }

    /// Reserve a specific register (for special purposes)
    pub fn reserve_register(&mut self, reg: Register) {
        match reg.kind() {
            RegKind::Temporary => {
                self.available_temp.retain(|&r| r != reg);
            }
            RegKind::Saved => {
                self.available_saved.retain(|&r| r != reg);
                self.callee_saved.push(reg);
            }
            _ => {}
        }
    }

    /// Check if a stack position has an allocated register
    pub fn has_allocation(&self, stack_pos: usize) -> bool {
        self.allocated.contains_key(&stack_pos)
    }

    /// get number of available registers
    pub fn available_count(&self) -> usize {
        self.available_temp.len() + self.available_saved.len()
    }
}


impl std::error::Error for SpillError {}

impl std::fmt::Display for SpillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Register spill required for stack position {}", self.stack_pos)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_allocator() {
        let alloc = RegisterAllocator::new();
        assert_eq!(alloc.available_temp.len(), 7); 
        assert_eq!(alloc.available_saved.len(), 10); 
        assert!(alloc.allocated.is_empty());
        assert!(alloc.callee_saved.is_empty());
    }

    #[test]
    fn test_allocation() {
        let mut alloc = RegisterAllocator::new();
        let reg1 = alloc.allocate(1);
        let reg2 = alloc.allocate(2);

        assert!(alloc.has_allocation(1));
        assert!(alloc.has_allocation(2));
        assert_eq!(alloc.get_register(1), reg1);
        assert_eq!(alloc.get_register(2), reg2);
    }

    #[test]
    fn test_free() {
        let mut alloc = RegisterAllocator::new();
        let initial_count = alloc.available_count();
        
        assert_eq!(alloc.available_count(), initial_count - 1);
        
        alloc.free(1);
        assert_eq!(alloc.available_count(), initial_count);
        assert!(!alloc.has_allocation(1));
    }

    #[test]
    fn test_clear_allocations() {
        let mut alloc = RegisterAllocator::new();
        let initial_count = alloc.available_count();
        
        alloc.allocate(1);
        alloc.allocate(2);
        alloc.allocate(3);
        
        alloc.clear_allocations();
        assert_eq!(alloc.available_count(), initial_count);
        assert!(alloc.allocated.is_empty());
        assert!(alloc.callee_saved.is_empty());
    }

    #[test]
    fn test_reserve_register() {
        let mut alloc = RegisterAllocator::new();
        let temp_reg = Register::from_raw(5); 
        let saved_reg = Register::from_raw(18); 
        
        alloc.reserve_register(temp_reg);
        alloc.reserve_register(saved_reg);
        
        assert!(!alloc.available_temp.contains(&temp_reg));
        assert!(!alloc.available_saved.contains(&saved_reg));
        assert!(alloc.callee_saved.contains(&saved_reg));
    }

    #[test]
    fn test_saved_register_tracking() {
        let mut alloc = RegisterAllocator::new();
        let initial_saved_count = alloc.available_saved.len();
        
        while !alloc.available_temp.is_empty() {
            alloc.allocate(alloc.allocated.len());
        }
        
        let pos = alloc.allocated.len();
        alloc.allocate(pos);
        
        assert_eq!(alloc.available_saved.len(), initial_saved_count - 1);
        assert_eq!(alloc.callee_saved.len(), 1);
    }
}