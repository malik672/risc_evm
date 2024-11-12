use std::collections::HashMap;
use crate::register::{Register, RegKind};

pub struct RegisterAllocator {
    allocated: HashMap<usize, Register>,
    available_temp: Vec<Register>,
    available_saved: Vec<Register>,
    callee_saved: Vec<Register>,
}