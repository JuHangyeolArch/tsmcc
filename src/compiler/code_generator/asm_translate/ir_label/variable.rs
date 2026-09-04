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

    pub fn variable(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        idx: usize,
        target: usize,
    ) -> Result<(),CodeGenError> {

        let var = self.resolver.res_variable(entry,idx).unwrap();
        let reg = self.resolver.res_register(entry,target).unwrap();

        self.calc_load_tmp1(asm_program,var,line_number);
        self.store_temp1_stack(asm_program,reg,line_number);
 
        Ok(())
    }

    pub fn parameter(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        idx: usize,
        target: usize,
    ) -> Result<(),CodeGenError> {

        let par = self.resolver.res_parameter(entry,idx).unwrap();
        let reg = self.resolver.res_register(entry,target).unwrap();

        self.calc_load_tmp1(asm_program,par,line_number);
        self.store_temp1_stack(asm_program,reg,line_number);
 
        Ok(())
    }


    
}
