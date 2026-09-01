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

    pub fn parin(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        idx: usize,
    ) -> Result<(),CodeGenError> {

        let reg = self.resolver.res_register(entry,idx).unwrap();

        self.store_sp_spt(asm_program, line_number);
        self.param_in(asm_program,reg,line_number);

        Ok(())
    }

    pub fn parout(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
    ) -> Result<(),CodeGenError> {


        self.inc_sp(asm_program,line_number);

        Ok(())
    }

    pub fn caller(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        callee: usize,
        idx: usize,
    ) -> Result<(),CodeGenError> {
        
        let bun = self.resolver.res_funcbun_ret(callee).unwrap();
        let bsa = self.resolver.res_func_ret(callee).unwrap();
        let reg = self.resolver.res_register(entry,idx).unwrap();
        
        self.param_in(asm_program,bun,line_number);
        self.branch_func(asm_program,bsa.0,line_number);
        //after return

        for i in 0..self.resolver.get_par_count(callee) + 2 {
            self.inc_sp(asm_program, line_number);
        }

        self.load_retval(asm_program,reg,line_number);
        
        Ok(())
    }

    
}
