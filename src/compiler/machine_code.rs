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

    pub fn get(&self, pos: usize) -> Option<MachineLine> {
        if let Some(line) = self.lines.get(pos) {
            Some(line.clone())
        } else {
            None
        }
    }

    pub fn as_iter(&self) -> impl Iterator<Item = MachineLine> {
        self.lines.clone().into_iter()
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

    pub fn to_string(&self) -> String {
        self.address.to_hex_uppercase() + ": " + &format!("{:03X}", self.value)
    }
}

impl ProducedBy for MachineLine {
    fn produced_by(&self) ->  Option<SpanInfo> {
        Some(self.span.clone())
    }
}

