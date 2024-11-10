# RISC-V EVM Emulator

A low-level emulator that executes Ethereum Virtual Machine (EVM) bytecode on RISC-V architecture.

## Overview

This project translates EVM bytecode into RISC-V instructions, enabling direct execution on RISC-V hardware. The translation process involves:

```
EVM Bytecode -> IR (Intermediate Representation) -> RISC-V Instructions
```

Started this to learn about RISC-V instructions because the docs seemed interesting:
https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf

Thought maybe could do something cool with it but honestly who knows 🤷‍♂️

### Current Status
- Got some register stuff working
- Basic memory management
- Stack operations exist
- Parsing EVM bytecode somewhat
- It's a work in progress

### Why?
- Won't lie to you, Instruction set architecture is fun and very fragmented(lmaoo) and idk what this code is for

## Building
```bash
cargo build # if you really want to
```

## References
- RISC-V Spec (the actually useful part): https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf
- [Add more if we find cool stuff]

## License
Do whatever you want with it 🚀