
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


use std::collections::HashMap;

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

    pub fn func_data(
        &mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram
    ) -> Result<(),CodeGenError> {

        let start =  self.resolver.res_function(entry).unwrap();
 
        self.temp_label = Some(self.resolver.res_funcbun_ret(entry).unwrap());
        
        self.add_line(asm_program,idiom::bun(start),line_number);

        let ret = self.resolver.res_func_ret(entry).unwrap();
        
        self.temp_label = Some(ret.0);
        self.add_line(asm_program,idiom::data(ret.1),line_number);
        
        let cons = self.resolver
            .get_cons_list(entry);
        for con in cons.iter() {
            self.temp_label = Some(con.0);
            self.add_line(asm_program,idiom::data(con.1),line_number);
        }

        let regs = self.resolver
            .get_reg_list(entry);
        
        for reg in regs.iter() {
            self.temp_label = Some(reg.1.0);
            self.add_line(asm_program,idiom::data(reg.1.1),line_number);

        }
        
        let vars = self.resolver
            .get_var_list(entry);
        for var in vars {
            self.temp_label = Some(var.1.0);
            self.add_line(asm_program,idiom::data(var.1.1),line_number);
        }
        
        let params = self.resolver
            .get_par_list(entry);
        for param in params.iter() {
            self.temp_label = Some(param.1.0);
            self.add_line(asm_program,idiom::data(param.1.1),line_number);
        }

        Ok(())

    }
}
