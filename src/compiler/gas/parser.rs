use core::convert::TryFrom;
use std::collections::HashMap;
use crate::compiler::memory::stack::Stack;

// EVM Opcode definition(CANCUN)
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    SAR,
    SHA3,
    ADDRESS,
    BALANCE,
    BASEFEE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    EXTCODEHASH,
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    GASLIMIT,
    CHAINID,
    SELFBALANCE,
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    MSIZE,
    GAS,
    JUMPDEST,
    PUSH1,
    PUSH2,
    PUSH3,
    PUSH4,
    PUSH5,
    PUSH6,
    PUSH7,
    PUSH8,
    PUSH9,
    PUSH10,
    PUSH11,
    PUSH12,
    PUSH13,
    PUSH14,
    PUSH15,
    PUSH16,
    PUSH17,
    PUSH18,
    PUSH19,
    PUSH20,
    PUSH21,
    PUSH22,
    PUSH23,
    PUSH24,
    PUSH25,
    PUSH26,
    PUSH27,
    PUSH28,
    PUSH29,
    PUSH30,
    PUSH31,
    PUSH32,
    DUP1,
    DUP2,
    DUP3,
    DUP4,
    DUP5,
    DUP6,
    DUP7,
    DUP8,
    DUP9,
    DUP10,
    DUP11,
    DUP12,
    DUP13,
    DUP14,
    DUP15,
    DUP16,
    SWAP1,
    SWAP2,
    SWAP3,
    SWAP4,
    SWAP5,
    SWAP6,
    SWAP7,
    SWAP8,
    SWAP9,
    SWAP10,
    SWAP11,
    SWAP12,
    SWAP13,
    SWAP14,
    SWAP15,
    SWAP16,
    LOG0,
    LOG1,
    LOG2,
    LOG3,
    LOG4,	
    PREVRANDAO,
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
    CREATE2,
    STATICCALL,
    REVERT,
    INVALID,
    SELFDESTRUCT,
}

impl TryFrom<u8> for Opcode {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Opcode::STOP),
            0x01 => Ok(Opcode::ADD),
            0x02 => Ok(Opcode::MUL),
            0x03 => Ok(Opcode::SUB),
            0x04 => Ok(Opcode::DIV),
            0x05 => Ok(Opcode::SDIV),
            0x06 => Ok(Opcode::MOD),
            0x07 => Ok(Opcode::SMOD),
            0x08 => Ok(Opcode::ADDMOD),
            0x09 => Ok(Opcode::MULMOD),
            0x0A => Ok(Opcode::EXP),
            0x0B => Ok(Opcode::SIGNEXTEND),
            0x10 => Ok(Opcode::LT),
            0x11 => Ok(Opcode::GT),
            0x12 => Ok(Opcode::SLT),
            0x13 => Ok(Opcode::SGT),
            0x14 => Ok(Opcode::EQ),
            0x15 => Ok(Opcode::ISZERO),
            0x16 => Ok(Opcode::AND),
            0x17 => Ok(Opcode::OR),
            0x18 => Ok(Opcode::XOR),
            0x19 => Ok(Opcode::NOT),
            0x1A => Ok(Opcode::BYTE),
            0x1B => Ok(Opcode::SHL),
            0x1C => Ok(Opcode::SHR),
            0x1D => Ok(Opcode::SAR),
            0x20 => Ok(Opcode::SHA3),
            0x30 => Ok(Opcode::ADDRESS),
            0x31 => Ok(Opcode::BALANCE),
            0x32 => Ok(Opcode::ORIGIN),
            0x33 => Ok(Opcode::CALLER),
            0x34 => Ok(Opcode::CALLVALUE),
            0x35 => Ok(Opcode::CALLDATALOAD),
            0x36 => Ok(Opcode::CALLDATASIZE),
            0x37 => Ok(Opcode::CALLDATACOPY),
            0x38 => Ok(Opcode::CODESIZE),
            0x39 => Ok(Opcode::CODECOPY),
            0x3A => Ok(Opcode::GASPRICE),
            0x3B => Ok(Opcode::EXTCODESIZE),
            0x3C => Ok(Opcode::EXTCODECOPY),
            0x3D => Ok(Opcode::RETURNDATASIZE),
            0x3E => Ok(Opcode::RETURNDATACOPY),
            0x3F => Ok(Opcode::EXTCODEHASH),
            0x40 => Ok(Opcode::BLOCKHASH),
            0x41 => Ok(Opcode::COINBASE),
            0x42 => Ok(Opcode::TIMESTAMP),
            0x43 => Ok(Opcode::NUMBER),
            0x44 => Ok(Opcode::PREVRANDAO),
            0x45 => Ok(Opcode::GASLIMIT),
            0x46 => Ok(Opcode::CHAINID),
            0x47 => Ok(Opcode::SELFBALANCE),
            0x48 => Ok(Opcode::BASEFEE),
            0x50 => Ok(Opcode::POP),
            0x51 => Ok(Opcode::MLOAD),
            0x52 => Ok(Opcode::MSTORE),
            0x53 => Ok(Opcode::MSTORE8),
            0x54 => Ok(Opcode::SLOAD),
            0x55 => Ok(Opcode::SSTORE),
            0x56 => Ok(Opcode::JUMP),
            0x57 => Ok(Opcode::JUMPI),
            0x58 => Ok(Opcode::PC),
            0x59 => Ok(Opcode::MSIZE),
            0x5A => Ok(Opcode::GAS),
            0x5B => Ok(Opcode::JUMPDEST),
            0x60..=0x7F => Ok(match value {
                0x60 => Opcode::PUSH1,
                0x61 => Opcode::PUSH2,
                0x62 => Opcode::PUSH3,
                0x63 => Opcode::PUSH4,
                0x64 => Opcode::PUSH5,
                0x65 => Opcode::PUSH6,
                0x66 => Opcode::PUSH7,
                0x67 => Opcode::PUSH8,
                0x68 => Opcode::PUSH9,
                0x69 => Opcode::PUSH10,
                0x6A => Opcode::PUSH11,
                0x6B => Opcode::PUSH12,
                0x6C => Opcode::PUSH13,
                0x6D => Opcode::PUSH14,
                0x6E => Opcode::PUSH15,
                0x6F => Opcode::PUSH16,
                0x70 => Opcode::PUSH17,
                0x71 => Opcode::PUSH18,
                0x72 => Opcode::PUSH19,
                0x73 => Opcode::PUSH20,
                0x74 => Opcode::PUSH21,
                0x75 => Opcode::PUSH22,
                0x76 => Opcode::PUSH23,
                0x77 => Opcode::PUSH24,
                0x78 => Opcode::PUSH25,
                0x79 => Opcode::PUSH26,
                0x7A => Opcode::PUSH27,
                0x7B => Opcode::PUSH28,
                0x7C => Opcode::PUSH29,
                0x7D => Opcode::PUSH30,
                0x7E => Opcode::PUSH31,
                0x7F => Opcode::PUSH32,
                _ => unreachable!(),
            }),
            0x80..=0x8F => Ok(match value {
                0x80 => Opcode::DUP1,
                0x81 => Opcode::DUP2,
                0x82 => Opcode::DUP3,
                0x83 => Opcode::DUP4,
                0x84 => Opcode::DUP5,
                0x85 => Opcode::DUP6,
                0x86 => Opcode::DUP7,
                0x87 => Opcode::DUP8,
                0x88 => Opcode::DUP9,
                0x89 => Opcode::DUP10,
                0x8A => Opcode::DUP11,
                0x8B => Opcode::DUP12,
                0x8C => Opcode::DUP13,
                0x8D => Opcode::DUP14,
                0x8E => Opcode::DUP15,
                0x8F => Opcode::DUP16,
                _ => unreachable!(),
            }),
            0x90..=0x9F => Ok(match value {
                0x90 => Opcode::SWAP1,
                0x91 => Opcode::SWAP2,
                0x92 => Opcode::SWAP3,
                0x93 => Opcode::SWAP4,
                0x94 => Opcode::SWAP5,
                0x95 => Opcode::SWAP6,
                0x96 => Opcode::SWAP7,
                0x97 => Opcode::SWAP8,
                0x98 => Opcode::SWAP9,
                0x99 => Opcode::SWAP10,
                0x9A => Opcode::SWAP11,
                0x9B => Opcode::SWAP12,
                0x9C => Opcode::SWAP13,
                0x9D => Opcode::SWAP14,
                0x9E => Opcode::SWAP15,
                0x9F => Opcode::SWAP16,
                _ => unreachable!(),
            }),
            0xA0 => Ok(Opcode::LOG0),
            0xA1 => Ok(Opcode::LOG1),
            0xA2 => Ok(Opcode::LOG2),
            0xA3 => Ok(Opcode::LOG3),
            0xA4 => Ok(Opcode::LOG4),
            0xF0 => Ok(Opcode::CREATE),
            0xF1 => Ok(Opcode::CALL),
            0xF2 => Ok(Opcode::CALLCODE),
            0xF3 => Ok(Opcode::RETURN),
            0xF4 => Ok(Opcode::DELEGATECALL),
            0xF5 => Ok(Opcode::CREATE2),
            0xFA => Ok(Opcode::STATICCALL),
            0xFD => Ok(Opcode::REVERT),
            0xFE => Ok(Opcode::INVALID),
            0xFF => Ok(Opcode::SELFDESTRUCT),
            _ => Err("Invalid opcode"),
        }
    }
}

// EVM Instruction
#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: Option<Vec<u8>>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self { opcode: Opcode::STOP, operand: Default::default() }
    }
}

// Intermediate Representation
#[derive(Debug, Clone)]
pub enum IRInstruction {
    BinaryOp {
        op: &'static str,
        dest: usize,
        src1: usize,
        src2: usize,
    },
    UnaryOp {
        op: &'static str,
        dest: usize,
        src: usize,
    },
    LoadConst {
        dest: usize,
        value: Vec<u8>,
    },
    Jump {
        target: usize,
    },
    ConditionalJump {
        condition: usize,
        target: usize,
    },
    Call {
        target: usize,
    },
    Return,
}

// RISC-V Instruction
#[derive(Debug, Clone)]
pub enum RiscVInstruction {
    ADD { rd: usize, rs1: usize, rs2: usize },
    SUB { rd: usize, rs1: usize, rs2: usize },
    AND { rd: usize, rs1: usize, rs2: usize },
    OR { rd: usize, rs1: usize, rs2: usize },
    XOR { rd: usize, rs1: usize, rs2: usize },
    SLL { rd: usize, rs1: usize, rs2: usize },
    SRL { rd: usize, rs1: usize, rs2: usize },
    SRA { rd: usize, rs1: usize, rs2: usize },
    LW { rd: usize, rs1: usize, imm: i32 },
    SW { rs1: usize, rs2: usize, imm: i32 },
    LI { rd: usize, imm: i64 },
    J { offset: i32 },
    BEQ { rs1: usize, rs2: usize, offset: i32 },
    JAL { rd: usize, offset: i32 },
    JALR { rd: usize, rs1: usize, offset: i32 },
}

// Bytecode Parser
pub fn parse_bytecode(bytecode: &[u8]) -> Result<Vec<Instruction>, &'static str> {
    let mut instructions = vec![Instruction::default(); bytecode.len() / 2];
    let mut i = 0;

    while i < bytecode.len() {
        let opcode = Opcode::try_from(bytecode[i])?;
        i += 1;

        let operand = match opcode {
            Opcode::PUSH1 => parse_push_operand(bytecode, &mut i, 1)?,
            Opcode::PUSH2 => parse_push_operand(bytecode, &mut i, 2)?,
            Opcode::PUSH3 => parse_push_operand(bytecode, &mut i, 3)?,
            Opcode::PUSH4 => parse_push_operand(bytecode, &mut i, 4)?,
            Opcode::PUSH5 => parse_push_operand(bytecode, &mut i, 5)?,
            Opcode::PUSH6 => parse_push_operand(bytecode, &mut i, 6)?,
            Opcode::PUSH7 => parse_push_operand(bytecode, &mut i, 7)?,
            Opcode::PUSH8 => parse_push_operand(bytecode, &mut i, 8)?,
            Opcode::PUSH9 => parse_push_operand(bytecode, &mut i, 9)?,
            Opcode::PUSH10 => parse_push_operand(bytecode, &mut i, 10)?,
            Opcode::PUSH11 => parse_push_operand(bytecode, &mut i, 11)?,
            Opcode::PUSH12 => parse_push_operand(bytecode, &mut i, 12)?,
            Opcode::PUSH13 => parse_push_operand(bytecode, &mut i, 13)?,
            Opcode::PUSH14 => parse_push_operand(bytecode, &mut i, 14)?,
            Opcode::PUSH15 => parse_push_operand(bytecode, &mut i, 15)?,
            Opcode::PUSH16 => parse_push_operand(bytecode, &mut i, 16)?,
            Opcode::PUSH17 => parse_push_operand(bytecode, &mut i, 17)?,
            Opcode::PUSH18 => parse_push_operand(bytecode, &mut i, 18)?,
            Opcode::PUSH19 => parse_push_operand(bytecode, &mut i, 19)?,
            Opcode::PUSH20 => parse_push_operand(bytecode, &mut i, 20)?,
            Opcode::PUSH21 => parse_push_operand(bytecode, &mut i, 21)?,
            Opcode::PUSH22 => parse_push_operand(bytecode, &mut i, 22)?,
            Opcode::PUSH23 => parse_push_operand(bytecode, &mut i, 23)?,
            Opcode::PUSH24 => parse_push_operand(bytecode, &mut i, 24)?,
            Opcode::PUSH25 => parse_push_operand(bytecode, &mut i, 25)?,
            Opcode::PUSH26 => parse_push_operand(bytecode, &mut i, 26)?,
            Opcode::PUSH27 => parse_push_operand(bytecode, &mut i, 27)?,
            Opcode::PUSH28 => parse_push_operand(bytecode, &mut i, 28)?,
            Opcode::PUSH29 => parse_push_operand(bytecode, &mut i, 29)?,
            Opcode::PUSH30 => parse_push_operand(bytecode, &mut i, 30)?,
            Opcode::PUSH31 => parse_push_operand(bytecode, &mut i, 31)?,
            Opcode::PUSH32 => parse_push_operand(bytecode, &mut i, 32)?,
            _ => None,
        };

        instructions.push(Instruction { opcode, operand });
    }

    Ok(instructions)
}

fn parse_push_operand(bytecode: &[u8], index: &mut usize, size: usize) -> Result<Option<Vec<u8>>, &'static str> {
    if *index + size <= bytecode.len() {
        let operand = bytecode[*index..*index + size].to_vec();
        *index += size;
        Ok(Some(operand))
    } else {
        Err("Unexpected end of bytecode")
    }
}

// IR Generator
pub fn generate_ir(instructions: &[Instruction], stack:&mut Stack) -> Vec<IRInstruction> {
    let mut ir = Vec::new();

    for inst in instructions {
        match inst.opcode {
            Opcode::ADD | Opcode::SUB | Opcode::MUL | Opcode::DIV | Opcode::MOD => {
                let op = match inst.opcode {
                    Opcode::ADD => "add",
                    Opcode::SUB => "sub",
                    Opcode::MUL => "mul",
                    Opcode::DIV => "div",
                    Opcode::MOD => "mod",
                    _ => unreachable!(),
                };
                let src2 = stack.pop().expect("stack underflow");
                let src1 = stack.pop().expect("stack underflow");
                let result = match op {
                    "add" => src1.wrapping_add(src2),
                    "sub" => src1.wrapping_sub(src2),
                    "mul" => src1.wrapping_mul(src2),
                    "div" => if src2 != 0 { src1 / src2 } else { 0 },
                    "mod" => if src2 != 0 { src1 % src2 } else { 0 },
                    _ => unreachable!(),
                };
                stack.push(result).expect("");
                ir.push(IRInstruction::BinaryOp {
                    op,
                    dest: stack.len() - 1,
                    src1: stack.len(),
                    src2: stack.len() + 1,
                });
            },
            Opcode::PUSH1 => {
                if let Some(operand) = &inst.operand {
                    let value = operand[0] as u64;
                    stack.push(value).expect("can't push to stack");
                    ir.push(IRInstruction::LoadConst {
                        dest: stack.len(),
                        value: [value as u8].to_vec(),
                    });
                }
            },
            Opcode::POP => {
                stack.pop().expect("");
                ir.push(IRInstruction::UnaryOp {
                    op: "pop",
                    dest: 0,
                    src: stack.len(),
                });
            },
            Opcode::JUMP => {
                let target = stack.pop().expect("");
                ir.push(IRInstruction::Jump {
                    target: target as usize,
                });
            },
            Opcode::JUMPI => {
                let condition = stack.pop().expect("");
                let target = stack.pop().expect("");
                ir.push(IRInstruction::ConditionalJump {
                    condition: condition as usize,
                    target: target as usize,
                });
            },
            Opcode::MLOAD => {
               //Memory not yet implemented
            },
            Opcode::MSTORE => {
               //Memory not yet implemented 
            },
            _ => {}
        }
    }

    ir
}

// Optimizer
pub fn optimize_ir(ir: &[IRInstruction]) -> Vec<IRInstruction> {
    // This is a very basic optimizer that just removes redundant loads(complexity incomning)
    let mut optimized = Vec::new();
    let mut last_load: Option<(usize, Vec<u8>)> = None;

    for inst in ir {
        match inst {
            IRInstruction::LoadConst { dest, value } => {
                if let Some((last_dest, ref last_value)) = last_load {
                    if *value == *last_value {
                        optimized.push(IRInstruction::UnaryOp {
                            op: "mov",
                            dest: *dest,
                            src: last_dest,
                        });
                        continue;
                    }
                }
                last_load = Some((*dest, value.clone()));
            }
            _ => last_load = None,
        }
        optimized.push(inst.clone());
    }

    optimized
}

// RISC-V Code Generator
pub fn generate_riscv(ir: &[IRInstruction]) -> Vec<RiscVInstruction> {
    let mut riscv = Vec::new();
    let mut label_map = HashMap::new();

    // First pass: collect jump targets
    for (i, inst) in ir.iter().enumerate() {
        if let IRInstruction::Jump { target } = inst {
            label_map.insert(*target, i);
        }
    }

    // Second pass: generate RISC-V instructions
    for (i, inst) in ir.iter().enumerate() {
        match inst {
            IRInstruction::BinaryOp {
                op,
                dest,
                src1,
                src2,
            } => {
                match *op {
                    "add" => riscv.push(RiscVInstruction::ADD {
                        rd: *dest,
                        rs1: *src1,
                        rs2: *src2,
                    }),
                    // ... other binary operations ...
                    _ => {} // Placeholder for unimplemented operations
                }
            }
            IRInstruction::LoadConst { dest, value } => {
                let imm = i64::from_be_bytes([0, 0, 0, 0, 0, 0, value[0], value[1]]);
                riscv.push(RiscVInstruction::LI { rd: *dest, imm });
            }
            IRInstruction::Jump { target } => {
                if let Some(&label) = label_map.get(target) {
                    let offset = (label as i32 - i as i32) * 4;
                    riscv.push(RiscVInstruction::J { offset });
                }
            }
            _ => {} // Placeholder for unimplemented instructions
        }
    }

    riscv
}

// Runtime Support
pub struct EVMRuntime {
    memory: Vec<u8>,
    stack: Vec<[u8; 32]>,
}

impl EVMRuntime {
    pub fn new() -> Self {
        EVMRuntime {
            memory: vec![0; 1024 * 1024], // 1MB of memory
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, value: [u8; 32]) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<[u8; 32]> {
        self.stack.pop()
    }

    pub fn mstore(&mut self, offset: usize, value: &[u8; 32]) {
        if offset + 32 <= self.memory.len() {
            self.memory[offset..offset + 32].copy_from_slice(value);
        }
    }

    pub fn mload(&self, offset: usize) -> [u8; 32] {
        let mut value = [0u8; 32];
        if offset + 32 <= self.memory.len() {
            value.copy_from_slice(&self.memory[offset..offset + 32]);
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bytecode() {
        let bytecode = vec![0x60, 0x80, 0x60, 0x40, 0x52];
        let instructions = parse_bytecode(&bytecode).unwrap();

        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0].opcode, Opcode::PUSH1);
        assert_eq!(instructions[0].operand, Some(vec![0x80]));
        assert_eq!(instructions[1].opcode, Opcode::PUSH1);
        assert_eq!(instructions[1].operand, Some(vec![0x40]));
        assert_eq!(instructions[2].opcode, Opcode::MSTORE);
        assert_eq!(instructions[2].operand, None);
    }

    #[test]
    fn test_generate_ir() {
        let instructions = vec![
            Instruction {
                opcode: Opcode::PUSH1,
                operand: Some(vec![0x80]),
            },
            Instruction {
                opcode: Opcode::PUSH1,
                operand: Some(vec![0x40]),
            },
            Instruction {
                opcode: Opcode::ADD,
                operand: None,
            },
        ];

        let ir = generate_ir(&instructions,    &mut Stack::new());

        assert_eq!(ir.len(), 3);
        match &ir[0] {
            IRInstruction::LoadConst { dest, value } => {
                assert_eq!(*dest, 0);
                assert_eq!(*value, vec![0x80]);
            }
            _ => panic!("Expected LoadConst"),
        }
        match &ir[1] {
            IRInstruction::LoadConst { dest, value } => {
                assert_eq!(*dest, 1);
                assert_eq!(*value, vec![0x40]);
            }
            _ => panic!("Expected LoadConst"),
        }
        match &ir[2] {
            IRInstruction::BinaryOp {
                op,
                dest,
                src1,
                src2,
            } => {
                assert_eq!(*op, "add");
                assert_eq!(*dest, 0);
                assert_eq!(*src1, 0);
                assert_eq!(*src2, 1);
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_optimize_ir() {
        let ir = vec![
            IRInstruction::LoadConst {
                dest: 0,
                value: vec![0x80],
            },
            IRInstruction::LoadConst {
                dest: 1,
                value: vec![0x80],
            },
            IRInstruction::BinaryOp {
                op: "add",
                dest: 2,
                src1: 0,
                src2: 1,
            },
        ];

        let optimized = optimize_ir(&ir);

        assert_eq!(optimized.len(), 3);
        match &optimized[1] {
            IRInstruction::UnaryOp { op, dest, src } => {
                assert_eq!(*op, "mov");
                assert_eq!(*dest, 1);
                assert_eq!(*src, 0);
            }
            _ => panic!("Expected UnaryOp"),
        }
    }

    #[test]
    fn test_generate_riscv() {
        let ir = vec![
            IRInstruction::LoadConst {
                dest: 0,
                value: vec![0x80],
            },
            IRInstruction::LoadConst {
                dest: 1,
                value: vec![0x40],
            },
            IRInstruction::BinaryOp {
                op: "add",
                dest: 2,
                src1: 0,
                src2: 1,
            },
        ];
    }
}
