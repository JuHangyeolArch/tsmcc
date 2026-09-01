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

    pub fn adder(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        source1: usize,
        source2: usize,
        target: usize
    ) -> Result<(),CodeGenError> {

        let left_reg = self.resolver.res_register(entry,source1).unwrap();
        let right_reg = self.resolver.res_register(entry,source2).unwrap();
        let result_reg = self.resolver.res_register(entry,target).unwrap();


        self.calc_load_tmp1(asm_program,left_reg,line_number);
        self.calc_load_tmp2(asm_program,left_reg,line_number);

        self.add_temps(asm_program,line_number);

        self.store_temp1_stack(asm_program,result_reg,line_number);
        
        Ok(())
    }

    pub fn substractor(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        source1: usize,
        source2: usize,
        target: usize
    ) -> Result<(),CodeGenError> {

        let left_reg = self.resolver.res_register(entry,source1).unwrap();
        let right_reg = self.resolver.res_register(entry,source2).unwrap();
        let result_reg = self.resolver.res_register(entry,target).unwrap();


        self.calc_load_tmp1(asm_program,left_reg,line_number);
        self.calc_spo(asm_program,right_reg,line_number);
        self.add_line(
            asm_program,
            idiom::load_ind_spo(),
            line_number
        );
        
        self.add_line(
            asm_program,
            idiom::cmp_ac(),
            line_number
        );
        self.add_line(
            asm_program,
            idiom::store_temp2(),
            line_number
        );        
        self.add_temps(asm_program,line_number);

        self.store_temp1_stack(asm_program,result_reg,line_number);
        
        Ok(())
    }

    pub fn multiplier(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        source1: usize,
        source2: usize,
        target: usize
    ) -> Result<(),CodeGenError> {

        let left_reg = self.resolver.res_register(entry,source1).unwrap();
        let right_reg = self.resolver.res_register(entry,source2).unwrap();
        let result_reg = self.resolver.res_register(entry,target).unwrap();


        self.calc_spo(asm_program,left_reg,line_number);
        self.add_line(
            asm_program,
            idiom::load_spo(),
            line_number
        );

        self.add_line(
            asm_program,
            idiom::store(idiom::mux()),
            line_number
        );
        self.calc_spo(asm_program,right_reg,line_number);

        self.add_line(
            asm_program,
            idiom::load_spo(),
            line_number
        );

        self.add_line(
            asm_program,
            idiom::store(idiom::muy()),
            line_number
        );

        self.add_line(
            asm_program,
            idiom::bsa(idiom::mul()),
            line_number
        );


        self.add_line(
            asm_program,
            idiom::load(idiom::mup()),
            line_number
        );
        self.add_line(
            asm_program,
            idiom::store(idiom::temp1()),
            line_number
        );
        self.store_temp1_stack(asm_program,result_reg,line_number);
 

        
        Ok(())
    }

    pub fn divider(&mut self,
        entry: usize,
        line_number: usize,
        asm_program: &mut AssemblyProgram,
        source1: usize,
        source2: usize,
        target: usize
    ) -> Result<(),CodeGenError> {

        let left_reg = self.resolver.res_register(entry,source1).unwrap();
        let right_reg = self.resolver.res_register(entry,source2).unwrap();
        let result_reg = self.resolver.res_register(entry,target).unwrap();


        self.calc_spo(asm_program,left_reg,line_number);
        self.add_line(
            asm_program,
            idiom::load_spo(),
            line_number
        );

        self.add_line(
            asm_program,
            idiom::store(idiom::dix()),
            line_number
        );
        self.calc_spo(asm_program,right_reg,line_number);

        self.add_line(
            asm_program,
            idiom::load_spo(),
            line_number
        );

        self.add_line(
            asm_program,
            idiom::store(idiom::diy()),
            line_number
        );

        self.add_line(
            asm_program,
            idiom::bsa(idiom::div()),
            line_number
        );

       self.add_line(
            asm_program,
            idiom::load(idiom::dip()),
            line_number
        );
        self.add_line(
            asm_program,
            idiom::store(idiom::temp1()),
            line_number
        );
        self.store_temp1_stack(asm_program,result_reg,line_number);
        
        Ok(())
    }

    

}
