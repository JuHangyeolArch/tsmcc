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

    pub fn save_retval(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        idx: usize,
    ) -> Result<(),CodeGenError> {

        let reg = self.resolver.res_register(entry,idx).unwrap();
        
        self.store_retval(asm_program,reg,line_number);

        
        Ok(())
    }


}
