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


use crate::compiler::code_generator::{
    CodeGenerator,
    CodeGenError,
    label_resolution::{
        AsmFuncData,
        LabelResolver,
    },
};


impl CodeGenerator {

    fn match_ir_label(&mut self,
        entry: usize,
        ir_line: &IlocLabel,
        asm_program: &mut AssemblyProgram) -> Result<(),CodeGenerator> {

            match ir_line {}




    }

}
