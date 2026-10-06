use std::collections::HashMap;

use crate::compiler::ProducedBy;

use crate::compiler::SpanInfo;
use crate::compiler::CompileError;

use crate::compiler::assembly::{
    AssemblyInstruction,
    AssemblyLabel,
    AssemblyLine,
    AssemblyProgram,
    U12,
    AssemblyValue,
};


#[derive(Debug, Clone, PartialEq)]
pub struct AsmTable {
    table: HashMap<AssemblyLabel,U12>
}

impl AsmTable {
    pub fn new() -> Self {
        AsmTable {
            table: HashMap::new()
        }
    }

    pub fn push(&mut self, label:AssemblyLabel, address: u16) {
        self.table.insert(label,U12::new(address));

    }

    pub fn get(&self, label: AssemblyLabel) -> Option<&U12> {
        self.table.get(&label)
        
    }

    pub fn resolve_value(
        &self,
        val: AssemblyValue,
    ) -> u16 {

        println!("{:?}",val);
        
        match val {
            
            AssemblyValue::Label(label) => {self.get(label).unwrap().value()},
            AssemblyValue::Constant(number) => {number.value()},
        }
        
    }

    
}
