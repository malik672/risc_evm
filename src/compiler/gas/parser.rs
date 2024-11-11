use crate::compiler::memory::{memory::Memory, stack::Stack};
use crate::{MyU256 as U256, I256};
use alloy_primitives::hex::ToHex;
use alloy_primitives::U256 as U;
use core::convert::TryFrom;
use std::ops::{Add, Mul, Sub};

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

impl Opcode {
    fn is_push(&self) -> bool {
        matches!(
            self,
            Opcode::PUSH1
                | Opcode::PUSH2
                | Opcode::PUSH3
                | Opcode::PUSH4
                | Opcode::PUSH5
                | Opcode::PUSH6
                | Opcode::PUSH7
                | Opcode::PUSH8
                | Opcode::PUSH9
                | Opcode::PUSH10
                | Opcode::PUSH11
                | Opcode::PUSH12
                | Opcode::PUSH13
                | Opcode::PUSH14
                | Opcode::PUSH15
                | Opcode::PUSH16
                | Opcode::PUSH17
                | Opcode::PUSH18
                | Opcode::PUSH19
                | Opcode::PUSH20
                | Opcode::PUSH21
                | Opcode::PUSH22
                | Opcode::PUSH23
                | Opcode::PUSH24
                | Opcode::PUSH25
                | Opcode::PUSH26
                | Opcode::PUSH27
                | Opcode::PUSH28
                | Opcode::PUSH29
                | Opcode::PUSH30
                | Opcode::PUSH31
                | Opcode::PUSH32
        )
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
        Self {
            opcode: Opcode::STOP,
            operand: Default::default(),
        }
    }
}

// Intermediate Representation
#[derive(Debug, Clone)]
pub enum IRInstruction {
    BinaryOp {
        op: &'static str,
        dest: U256,
        src1: U256,
        src2: U256,
    },
    UnaryOp {
        op: &'static str,
        dest: U256,
        src: U256,
    },
    TernaryOp {
        op: &'static str,
        dest: U256,
        src1: U256,
        src2: U256,
        src3: U256,
    },
    LoadConst {
        dest: U256,
        value: U256,
    },
    MemoryLoad {
        offset: U256,
        dest: U256,
    },
    MemoryStore {
        offset: U256,
        value: U256,
    },
    Jump {
        target: U256,
    },
    ConditionalJump {
        condition: U256,
        target: U256,
    },
    Call {
        target: U256,
    },
    Stop,
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
    let mut instructions = Vec::new();
    let mut i = 0;

    while i < bytecode.len() {
        let opcode = Opcode::try_from(bytecode[i])?;
        i += 1;

        let operand = match bytecode[i - 1] {
            n @ 0x60..=0x7F => {
                let size = (n - 0x60 + 1) as usize;
                parse_push_operand(bytecode, &mut i, size)?
            }
            _ => None,
        };

        instructions.push(Instruction { opcode, operand });
    }

    Ok(instructions)
}

fn parse_push_operand(
    bytecode: &[u8],
    index: &mut usize,
    size: usize,
) -> Result<Option<Vec<u8>>, &'static str> {
    ///todo: should be a feature(too performance expensive for every check), so we assume the bytecode is well formed
    // if size == 0 || size > 32 {
    //     return Err("Invalid PUSH size");
    // }

    // if *index + size > bytecode.len() {
    //     return Err("Unexpected end of bytecode");
    // }
    let operand = bytecode[*index..*index + size].to_vec();
    *index += size;

    Ok(Some(operand))
}

fn pad_left(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];

    // Example for PUSH2 with bytes [0xAB, 0xCD]:
    // padded before: [0,0,0,0,0,0,...,0,0] (32 zeros)
    // copy starts at: 32 - 2 = 30
    // after copy:    [0,0,0,0,...,0,AB,CD]
    padded[32 - bytes.len()..].copy_from_slice(bytes);
    padded
}

// IR Generator
pub fn generate_ir(
    instructions: &[Instruction],
    stack: &mut Stack,
    memory: &mut Memory,
) -> Vec<IRInstruction> {
    let mut ir = Vec::new();

    for inst in instructions {
        match inst.opcode {
            Opcode::STOP => {
                ir.push(IRInstruction::Stop);
                break;
            }
            Opcode::ADD | Opcode::SUB | Opcode::MUL | Opcode::DIV | Opcode::MOD => {
                let op = match inst.opcode {
                    Opcode::ADD => "add",
                    Opcode::SUB => "sub",
                    Opcode::MUL => "mul",
                    Opcode::DIV => "div",
                    Opcode::MOD => "mod",
                    _ => unreachable!(),
                };
                //capture stack length before any operation
                let stack_pos = stack.len();

                let src2 = stack.pop().expect("stack underflow");
                let src1 = stack.pop().expect("stack underflow");
                let result = match op {
                    "add" => src1.add(src2),
                    "sub" => src1.sub(src2),
                    "mul" => src1.mul(src2),
                    "div" => {
                        if src2 != U256::default() {
                            src1 / src2
                        } else {
                            U256::default()
                        }
                    }
                    "mod" => {
                        if src2 != U256::default() {
                            src1 % src2
                        } else {
                            U256::default()
                        }
                    }
                    _ => unreachable!(),
                };
                stack.push(result).expect("");
                ir.push(IRInstruction::BinaryOp {
                    op,
                    dest: U256(U::from(stack_pos - 2)),
                    src1: U256(U::from(stack_pos - 2)),
                    src2: U256(U::from(stack_pos - 1)),
                });
            }
            Opcode::SMOD => {
                let b = stack.pop().expect("Stack underflow");
                let a = stack.pop().expect("Stack underflow");

                //capture stack length before any operation
                let stack_pos = stack.len();

                let result = if b.0.is_zero() {
                    U256::default()
                } else {
                    let a_i256 = I256(a.0);
                    let b_i256 = I256(b.0);
                    let r = a_i256 % b_i256;
                    U256(r.0)
                };
                stack.push(result).expect("Stack overflow");
                ir.push(IRInstruction::BinaryOp {
                    op: "smod",
                    dest: U256(U::from(stack_pos - 2)),
                    src1: U256(U::from(stack_pos - 2)),
                    src2: U256(U::from(stack_pos - 1)),
                });
            }
            Opcode::ADDMOD => {
                let n = stack.pop().expect("Stack underflow");
                let b = stack.pop().expect("Stack underflow");
                let a = stack.pop().expect("Stack underflow");

                //capture stack length before any operation
                let stack_pos = stack.len();
                let result = if n.0.is_zero() {
                    U256::default()
                } else {
                    let sum = a.0.overflowing_add(b.0).0;
                    U256(sum % n.0)
                };
                stack.push(result).expect("Stack overflow");
                ir.push(IRInstruction::TernaryOp {
                    op: "addmod",
                    dest: U256(U::from(stack_pos - 3)),
                    src1: U256(U::from(stack_pos - 3)),
                    src2: U256(U::from(stack_pos - 2)),
                    src3: U256(U::from(stack_pos - 1)),
                });
            }
            Opcode::MULMOD => {
                let n = stack.pop().expect("Stack underflow");
                let b = stack.pop().expect("Stack underflow");
                let a = stack.pop().expect("Stack underflow");

                //capture stack length before any operation
                let stack_pos = stack.len();
                let result = if n.0.is_zero() {
                    U256::default()
                } else {
                    let product = a.0.overflowing_mul(b.0).0;
                    U256(product % n.0)
                };
                stack.push(result).expect("Stack overflow");
                ir.push(IRInstruction::TernaryOp {
                    op: "mulmod",
                    dest: U256(U::from(stack_pos - 3)),
                    src1: U256(U::from(stack_pos - 3)),
                    src2: U256(U::from(stack_pos - 2)),
                    src3: U256(U::from(stack_pos - 1)),
                });
            }
            Opcode::EXP => {
                let b = stack.pop().expect("Stack underflow");
                let a = stack.pop().expect("Stack underflow");
                let result = U256(a.0.overflowing_pow(b.0).0);
                stack.push(result).expect("Stack overflow");

                //capture stack length before any operation
                let stack_pos = stack.len();
                ir.push(IRInstruction::BinaryOp {
                    op: "exp",
                    dest: U256(U::from(stack_pos - 2)),
                    src1: U256(U::from(stack_pos - 2)),
                    src2: U256(U::from(stack_pos - 1)),
                });
            }
            Opcode::SIGNEXTEND => {}
            Opcode::LT | Opcode::GT | Opcode::SLT | Opcode::SGT | Opcode::EQ => {
                let b = stack.pop().expect("Stack underflow");
                let a = stack.pop().expect("Stack underflow");
                let ops;

                //capture stack length before any operation
                let stack_pos = stack.len();
                let result = match inst.opcode {
                    Opcode::LT => {
                        ops = "LT";
                        U256(U::from(a.0 < b.0))
                    }
                    Opcode::GT => {
                        ops = "GT";
                        U256(U::from(a.0 > b.0))
                    }
                    Opcode::SLT => {
                        ops = "SLT";
                        let a_i256 = I256(a.0);
                        let b_i256 = I256(b.0);
                        U256(U::from(a_i256 < b_i256))
                    }
                    Opcode::SGT => {
                        ops = "SGT";
                        let a_i256 = I256(a.0);
                        let b_i256 = I256(b.0);
                        U256(U::from(a_i256 > b_i256))
                    }
                    Opcode::EQ => {
                        ops = "EQ";
                        U256(U::from(a.0 == b.0))
                    }
                    _ => unreachable!(),
                };
                stack.push(result).expect("Stack overflow");
                ir.push(IRInstruction::BinaryOp {
                    op: ops,
                    dest: U256(U::from(stack_pos - 2)),
                    src1: U256(U::from(stack_pos - 2)),
                    src2: U256(U::from(stack_pos - 1)),
                });
            }
            Opcode::NOT => {
                let a = stack.pop().expect("Stack underflow");
                let result = U256(!a.0);
                stack.push(result).expect("Stack overflow");

                //capture stack length before any operation
                let stack_pos = stack.len();
                ir.push(IRInstruction::UnaryOp {
                    op: "not",
                    dest: U256(U::from(0)),
                    src: U256(U::from(stack_pos - 1)),
                });
            }

            Opcode::BYTE => {
                let i = stack.pop().expect("Stack underflow");
                let x = stack.pop().expect("Stack underflow");
                let result = if i.0 >= U::from(32) {
                    U256::default()
                } else {
                    let byte = (x.0 >> (U::from(8) * (U::from(31) - i.0))) & U::from(0xFF);
                    U256(byte)
                };
                //capture stack length before any operation
                let stack_pos = stack.len();
                stack.push(result).expect("Stack overflow");
                ir.push(IRInstruction::BinaryOp {
                    op: "byte",
                    dest: U256(U::from(stack_pos - 2)),
                    src1: U256(U::from(stack_pos - 2)),
                    src2: U256(U::from(stack_pos - 1)),
                });
            }
            Opcode::SHA3 => {
                let offset = stack.pop().expect("Stack underflow");
                let size = stack.pop().expect("Stack underflow");
            }
            opcode if opcode.is_push() => {
                if let Some(operand) = &inst.operand {
                    let stack_pos = stack.len();
                    let value = U256(U::from_be_bytes(pad_left(operand)));
                    stack.push(value).expect("can't push to stack");
                    ir.push(IRInstruction::LoadConst {
                        dest: U256(U::from(stack_pos)),
                        value,
                    });
                }
            }
            Opcode::POP => {
                stack.pop().expect("");
                //capture stack length before any operation
                let stack_pos = stack.len();
                ir.push(IRInstruction::UnaryOp {
                    op: "pop",
                    dest: U256(U::from(0)),
                    src: U256(U::from(stack.len() + 1)),
                });
            }
            Opcode::JUMP => {
                let target = stack.pop().expect("");
                ir.push(IRInstruction::Jump { target });
            }
            Opcode::JUMPI => {
                let condition = stack.pop().expect("");
                let target = stack.pop().expect("");
                ir.push(IRInstruction::ConditionalJump { condition, target });
            }
            Opcode::MLOAD => {
                let index = stack.pop().expect("Stack underflow");
                let value = memory.read_word(index.as_usize());

                let _ = stack.push(U256::try_from(value).unwrap());

                ir.push(IRInstruction::MemoryLoad {
                    offset: U256(U::from(stack.len() - 1)),
                    dest: U256::default(),
                })
            }
            Opcode::MSTORE => {
                let offset = stack.pop().expect("Stack underflow");
                let value = stack.pop().expect("Stack underflow");

                memory.write_word(offset.as_usize(), value.to_be_bytes());

                ir.push(IRInstruction::MemoryStore {
                    offset: U256(U::from(stack.len())),
                    value: U256::default(),
                })
            }
            _ => {}
        }
    }

    ir
}

// Runtime Support
pub struct EVMRuntime {
    memory: Vec<u8>,
    stack: Vec<[u8; 32]>,
}

impl EVMRuntime {
    pub fn new() -> Self {
        EVMRuntime {
            memory: vec![0; 1024 * 1024],
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

        let ir = generate_ir(&instructions, &mut Stack::new(), &mut Memory::new());

        assert_eq!(ir.len(), 3);
        match &ir[0] {
            IRInstruction::LoadConst { dest, value } => {
                assert_eq!(*dest, U256(U::from(1)));
                assert_eq!(*value, U256(U::from(0x80)));
            }
            _ => panic!("Expected LoadConst"),
        }
        match &ir[1] {
            IRInstruction::LoadConst { dest, value } => {
                assert_eq!(*dest, U256(U::from(2)));
                assert_eq!(*value, U256(U::from(0x40)));
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
                assert_eq!(*dest, U256(U::from(1)));
                assert_eq!(*src1, U256(U::from(2)));
                assert_eq!(*src2, U256(U::from(1)));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }
}
