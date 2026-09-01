use crate::compiler::code_generator::asm_translate::idiom::add;
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

    pub fn built_in(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        name: &String,
    ) -> Result<(),CodeGenError> {

        let put ="PUT".to_string();
        let get ="GET".to_string();
        
        if matches!(name, put) {
            self.built_in_put(entry,line_number,asm_program)
        } else if matches!(name, get) {
            self.built_in_put(entry,line_number,asm_program)            
        } else {
            Err(CodeGenError::UnexpectedError)
        }
        
    }

    pub fn built_in_put(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
    ) -> Result<(),CodeGenError> {

        let func = self.resolver
            .res_function(entry)
            .unwrap();
        
        self.func_start(entry,asm_program)?;

        self.add_line(asm_program, AssemblyInstruction::SKI,line_number);
        self.add_line(asm_program, idiom::bun(func),line_number);
        self.add_line(asm_program, AssemblyInstruction::INP,line_number);
        self.add_line(asm_program, AssemblyInstruction::OUT,line_number);
        self.add_line(asm_program, idiom::store_retval(),line_number);

        let Some((label,_)) = self.resolver.res_func_ret(entry) else {
                return Err(CodeGenError::UnexpectedError);
            };
            
        idiom::bun_ind(label);

        Ok(())
        
    }

    pub fn built_in_get(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
    ) -> Result<(),CodeGenError> {

        self.func_start(entry,asm_program)?;
        self.add_line(asm_program,idiom::load_ind_sp(),line_number);

        self.temp_label = Some(AssemblyLabel('C','O','F'));
        self.add_line(asm_program, AssemblyInstruction::SKO,line_number);
        self.add_line(asm_program, idiom::bun(AssemblyLabel('C','O','F')),line_number);
        self.add_line(asm_program, AssemblyInstruction::OUT,line_number);
        self.add_line(asm_program, idiom::store_retval(),line_number);

        Ok(())
    }


}
