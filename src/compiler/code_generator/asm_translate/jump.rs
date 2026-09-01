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

    pub fn jump(&mut self,
        entry: usize,
        line_number: usize,        
        asm_program: &mut AssemblyProgram,
        source: usize,

    ) -> Result<(),CodeGenError> {

        let reg = self.resolver.res_register(entry,source).unwrap();


        self.calc_spo(asm_program,reg,line_number);
        self.add_line(
            asm_program,
            idiom::bun_ind(idiom::spo()),
            line_number
        );
        
        Ok(())
    }
    
    pub fn jump_immediate(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        idx: usize,
    ) -> Result<(),CodeGenError> {

        let branch = self.resolver.res_branch(entry,idx).unwrap();
        
        self.add_line(
            asm_program,
            idiom::bun(branch),
            line_number
        );

        Ok(())    
    }
}
