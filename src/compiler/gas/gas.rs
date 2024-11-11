use super:: parser::*;
use crate::MyU256 as U256;
use crate::primitives::SpecId;



#[derive(Debug)]
pub enum GasError {
    OutOfGas,
}

pub struct PipelinedGasMeter {
    gas_left: u64,
    
    spec_id: SpecId,
    pipeline_stages: usize,
    speculative_gas: Vec<u64>,
}

impl PipelinedGasMeter {
    pub fn new(gas_limit: u64, spec_id: SpecId, pipeline_stages: usize) -> Self {
        PipelinedGasMeter {
            gas_left: gas_limit,
            spec_id,
            pipeline_stages,
            speculative_gas: vec![0; pipeline_stages],
        }
    }
    
    pub fn commit_gas(&mut self, stage: usize) -> Result<(), GasError> {
        let gas_to_commit = self.speculative_gas[stage];
        if self.gas_left < gas_to_commit {
            return Err(GasError::OutOfGas);
        }
        self.gas_left -= gas_to_commit;
        self.speculative_gas[stage] = 0;
        Ok(())
    }

    pub fn rollback_gas(&mut self, stage: usize) {
        self.speculative_gas[stage] = 0;
    }

    pub fn gas_left(&self) -> u64 {
        self.gas_left
    }

}


