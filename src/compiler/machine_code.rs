use crate::compiler::ProducedBy;

use crate::compiler::SpanInfo;
use crate::compiler::CompileError;

use crate::compiler::assembly::{
    U12    
};

#[derive(Debug, Clone, PartialEq)]
pub struct MachineProgram {
    lines: Vec<MachineLine>
}

impl MachineProgram{

    pub fn new() -> Self {
        MachineProgram {
            lines: Vec::new()

        }
    }

    pub fn push(&mut self, address:U12,value: u16,origin: usize) {
        let line = MachineLine::new(address,value,origin);
        self.lines.push(line);
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct MachineLine {
    address: U12,
    value: u16,
    span: SpanInfo
}

impl MachineLine{

    pub fn new(address: U12, value: u16, origin: usize) -> Self {
        MachineLine {
            address,
            value,
            span: SpanInfo::new(super::SnapShotPhase::Assembler,Some(origin),Some(origin + 1))
        }
    }


}
