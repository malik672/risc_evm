use std::collections::HashMap;
use super::register::{RegKind, Register};

pub struct RegisterAllocator {
    // maps stack positions to physical registers
    allocated: HashMap<usize, Register>,
    available_temp: Vec<Register>,
    available_saved: Vec<Register>,
    callee_saved: Vec<Register>,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        let mut allocator = RegisterAllocator {
            allocated: HashMap::new(),
            available_temp: Vec::new(),
            available_saved: Vec::new(),
            callee_saved: Vec::new(),
        };

        // initialize temporary registers (t0-t2, t3-t6)
        for i in 5..=7 {
            allocator.available_temp.push(Register::from_raw(i));
        }
        for i in 28..=31 {
            allocator.available_temp.push(Register::from_raw(i));
        }

        // initialize saved registers (s2-s11)
        for i in 18..=27 {
            allocator.available_saved.push(Register::from_raw(i));
        }

        allocator
    }

    /// get a register for a stack position, allocating if necessary
    pub fn get_register(&mut self, stack_pos: usize) -> Register {
        if let Some(&reg) = self.allocated.get(&stack_pos) {
            reg
        } else {
            self.allocate(stack_pos)
        }
    }

    /// allocate a register for a stack position
    pub fn allocate(&mut self, stack_pos: usize) -> Register {
        if let Some(reg) = self.available_temp.pop() {
            self.allocated.insert(stack_pos, reg);
            return reg;
        }

        if let Some(reg) = self.available_saved.pop() {
            self.callee_saved.push(reg);
            self.allocated.insert(stack_pos, reg);
            return reg;
        }

        // If no registers available, we need to spill
        // or we simply find a way to free registres: something like a garbage dump
        panic!("No available registers - spilling needed");
    }

    /// free a register allocated to a stack position
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
                //todo: Fixed
                _ => {} 
            }
        }
    }

    /// get list of currently used callee-saved registers
    pub fn get_callee_saved(&self) -> &[Register] {
        &self.callee_saved
    }

    /// clear all allocations (useful between blocks)
    pub fn clear_allocations(&mut self) {
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

    /// reserve a specific register (for special purposes)
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

    /// check if a stack position has an allocated register
    pub fn has_allocation(&self, stack_pos: usize) -> bool {
        self.allocated.contains_key(&stack_pos)
    }

    /// get number of available registers
    pub fn available_count(&self) -> usize {
        self.available_temp.len() + self.available_saved.len()
    }
}
