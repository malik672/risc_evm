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