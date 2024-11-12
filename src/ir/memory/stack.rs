use crate::MyU256 as U256;

const STACK_SIZE: usize = 1024;
const SENTINEL: U256 = U256(U256::MAX); // Use MAX as a sentinel value

pub struct Stack {
    data: [U256; STACK_SIZE],
    top: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: [SENTINEL; STACK_SIZE],
            top: 0,
        }
    }

    pub fn push(&mut self, value: U256) -> Result<(), &'static str> {
        if self.top < self.data.len() {
            self.data[self.top] = value;
            self.top += 1;
            Ok(())
        } else {
            Err("Stack overflow")
        }
    }

    pub fn pop(&mut self) -> Result<U256, &'static str> {
        if self.top > 0 {
            self.top -= 1;
            let value = self.data[self.top];
            self.data[self.top] = SENTINEL;
            Ok(value)
        } else {
            Err("Stack underflow")
        }
    }

    pub fn peek(&self) -> Result<U256, &'static str> {
        if self.top > 0 {
            Ok(self.data[self.top - 1])
        } else {
            Err("Stack is empty")
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn len(&self) -> usize {
        self.top
    }
}