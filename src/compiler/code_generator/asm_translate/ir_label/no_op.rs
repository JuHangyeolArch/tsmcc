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
    asm_translate::idiom,
};


impl CodeGenerator {

    pub fn no_op(&mut self,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
    ) -> Result<(),CodeGenError> {

        self.add_line(
            asm_program,
            AssemblyInstruction::CLE,
            line_number  
        );
        
        Ok(())
    }


}
