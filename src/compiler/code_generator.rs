use std::collections::HashMap;

use crate::compiler::CompileError;
use crate::compiler::iloc::{
    IlocProgram,
    IlocLine,
    IlocInstruction,
    Register,
    IlocFunction,
    GotoLabel,
    IlocLabel
};

use crate::compiler::assembly::{
    AssemblyInstruction,
    AssemblyLabel,
    AssemblyLine,
    AssemblyProgram,
    U12,
    AssemblyValue,
};

mod asm_libraries;
mod asm_translate;

mod label_resolution;


#[derive(Debug, Clone, PartialEq)]
pub struct CodeGenerator {
    is_inited: bool,
    is_over: bool,
    cur_ir_func: usize,
    cur_ir_line: usize,
    temp_label: Option<AssemblyLabel>,
    resolver: label_resolution::LabelResolver
    
}
impl CodeGenerator {


    pub fn new() -> Self {
        CodeGenerator {
            is_inited: false,
            is_over: false,
            cur_ir_func: 0,
            cur_ir_line: 0,
            temp_label: None,
            resolver: label_resolution::LabelResolver::new()
        }
    }


    pub fn step(&mut self, ir_program: &IlocProgram, asm_program: &mut AssemblyProgram) -> Option<Result<(),CompileError>> {

        if !self.is_inited {
            asm_libraries::init_asm(asm_program);
            asm_program.add_line(None, AssemblyInstruction::ORG(U12::new(0x100)), None);        
            self.resolver.init_resolver(ir_program);
            self.is_inited = true;
        }
        
        if !self.is_over {
            if let Err(e) = self.asm_translate(ir_program,asm_program) {
                return Some(Err(CompileError::CodeGenError(e)))
            } else {
                return Some(Ok(()))
            }

        }
        
        
        None
    }

}


#[derive(Debug,Clone,PartialEq)]
pub enum CodeGenError {
    UnexpectedError,
}
