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

    pub fn func_start(&mut self,
        entry: usize,
        asm_program: &mut AssemblyProgram
    ) -> Result<(),CodeGenError > {
        
        let func_label  = self.resolver
            .res_function(entry).unwrap();

        self.temp_label = Some(func_label);
        Ok(())        
    }
    
    pub fn prologue(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram) -> Result<(),CodeGenError> {

            for i in 0..self.resolver.get_var_count(entry) {
                self.dec_sp(asm_program, line_number);
            }

            for i in 0..self.resolver.get_reg_count(entry) {
                self.dec_sp(asm_program, line_number);
            }
            
            Ok(())
        }

    pub fn epilogue(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram) -> Result<(),CodeGenError> {


            for i in 0..self.resolver.get_var_count(entry) {
                self.inc_sp(asm_program, line_number);
            }

            let Some((label,_)) = self.resolver.res_func_ret(entry) else {
                return Err(CodeGenError::UnexpectedError);
            };
            
            idiom::bun_ind(label);
            
            Ok(())
        }


    
}
