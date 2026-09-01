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

    pub fn loader(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        source: usize,
        target: usize
    ) -> Result<(),CodeGenError> {

        let left_reg = self.resolver.res_register(entry,source).unwrap();
        let result_reg = self.resolver.res_register(entry,target).unwrap();

        self.calc_load_tmp1(asm_program,left_reg,line_number);              
        self.store_temp1_stack(asm_program,result_reg,line_number);
        
        Ok(())
    }

    pub fn loader_immmediate(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        val: u16,
        target: usize
    ) -> Result<(),CodeGenError> {

        
        let cons = self.resolver.res_constant(entry,val).unwrap();

        let result_reg = self.resolver.res_register(entry,target).unwrap();
        
        self.calc_spo(asm_program,result_reg,line_number);
        self.add_line(
            asm_program,
            idiom::load(cons),
            line_number
        );

        self.add_line(
            asm_program,
            idiom::store_ind_spo(),
            line_number
        );
        
        Ok(())
    }

    pub fn store(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        source: usize,
        target: usize
    ) -> Result<(),CodeGenError> {

        let left_reg = self.resolver.res_register(entry,source1).unwrap();
        let right_reg = self.resolver.res_register(entry,source2).unwrap();
        let result_reg = self.resolver.res_register(entry,target).unwrap();


        self.calc_load_tmp1(asm_program,left_reg,line_number);              
        self.add_line(
            asm_program,
            idiom::load_ind_spo(),
            line_number
        );
        self.add_line(
            asm_program,
            idiom::store_spo(),
            line_number
        );
        self.add_line(
            asm_program,
            idiom::load_temp1(),
            line_number
        );
        self.add_line(
            asm_program,
            idiom::store_ind_spo(),
            line_number
        );
        
        Ok(())
    }

    

}
