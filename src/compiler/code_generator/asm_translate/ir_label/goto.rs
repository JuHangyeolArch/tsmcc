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

    pub fn goto_label(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        idx: usize,
    ) -> Result<(),CodeGenError> {

        let branch = self.resolver.res_branch(entry,idx);


        self.temp_label = branch;

        Ok(())
    }


}
