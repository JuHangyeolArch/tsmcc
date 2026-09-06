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
};

pub fn data(val: u16) -> AssemblyInstruction {
    AssemblyInstruction::Data(val)
}
 
pub fn temp1() -> AssemblyLabel {
    AssemblyLabel('T','P','1')
}

pub fn temp2() -> AssemblyLabel {
    AssemblyLabel('T','P','2')
}

pub fn sp() -> AssemblyLabel {
    AssemblyLabel('S','P',' ')
}

pub fn spo() -> AssemblyLabel {
    AssemblyLabel('S','P','O')
}

pub fn spt() -> AssemblyLabel {
    AssemblyLabel('S','P','T')
}


pub fn retval() -> AssemblyLabel {
    AssemblyLabel('R','T','V')
}

pub fn mul() -> AssemblyLabel {
    AssemblyLabel('M','U','L')
}


pub fn mux() -> AssemblyLabel {
    AssemblyLabel('M','U','X')
}

pub fn muy() -> AssemblyLabel {
    AssemblyLabel('M','U','X')
}

pub fn mup() -> AssemblyLabel {
    AssemblyLabel('M','U','X')
}


pub fn div() -> AssemblyLabel {
    AssemblyLabel('D','I','V')
}


pub fn dix() -> AssemblyLabel {
    AssemblyLabel('D','I','X')
}

pub fn diy() -> AssemblyLabel {
    AssemblyLabel('D','I','X')
}

pub fn dip() -> AssemblyLabel {
    AssemblyLabel('D','I','X')
}

pub fn con() -> AssemblyLabel {
    AssemblyLabel('C','O','N')
}


pub fn cox() -> AssemblyLabel {
    AssemblyLabel('C','O','X')
}


pub fn cop() -> AssemblyLabel {
    AssemblyLabel('C','O','P')
}


pub fn and(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::AND(
        AssemblyValue::Label(
            label
        ),
        false
    )

}

pub fn and_ind(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::AND(
        AssemblyValue::Label(
            label
        ),
        true
    )

}



pub fn add(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::ADD(
        AssemblyValue::Label(
            label
        ),
        false
    )

}

pub fn add_ind(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::ADD(
        AssemblyValue::Label(
            label
        ),
        true
    )

}


pub fn add_spo() -> AssemblyInstruction {
    add(spo())
}


pub fn add_ind_spo() -> AssemblyInstruction {
    add_ind(spo())
}

pub fn add_temp1() -> AssemblyInstruction {
    add(temp1())
}


pub fn add_ind_temp1() -> AssemblyInstruction {
    add_ind(temp1())
}

pub fn add_temp2() -> AssemblyInstruction {
    add(temp2())
}


pub fn add_ind_temp2() -> AssemblyInstruction {
    add_ind(temp2())
}


pub fn bun(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::BUN(
        AssemblyValue::Label(
            label
        ),
        false
    )

}

pub fn bun_ind(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::BUN(
        AssemblyValue::Label(
            label
        ),
        true
    )

}

pub fn bsa(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::BSA(
        AssemblyValue::Label(
            label
        ),
        false
    )


}    

pub fn bsa_ind(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::BSA(
        AssemblyValue::Label(
            label
        ),
        true
    )

}

pub fn bsa_sp() -> AssemblyInstruction {
    bsa(sp())
}

pub fn bsa_ind_sp() -> AssemblyInstruction {
    bsa_ind(sp())
}


pub fn bun_spo() -> AssemblyInstruction {
    bun_ind(spo())
}

pub fn bun_ind_spo() -> AssemblyInstruction {
    AssemblyInstruction::BUN(
        AssemblyValue::Label(
            spo()
        )
        ,
        true
    )

}

pub fn cmp_ac() -> AssemblyInstruction {
    AssemblyInstruction::CMA    
}

pub fn skip_zero_ac() -> AssemblyInstruction {
    AssemblyInstruction::SZA
}
    
pub fn inc_ac() -> AssemblyInstruction {
    AssemblyInstruction::INC
}




pub fn load(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::LDA(
        AssemblyValue::Label(
            label
        ),
        false
    )
}

pub fn load_ind(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::LDA(
        AssemblyValue::Label(
            label
        ),
        true
    )
}

pub fn load_retval() -> AssemblyInstruction {
    load(retval())
}


pub fn load_sp() -> AssemblyInstruction {
    load(sp())
}

pub fn load_ind_sp() -> AssemblyInstruction {
    load_ind(sp())
}


pub fn store(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::STA(
        AssemblyValue::Label(
            label
        ),
        false
    )
}

pub fn store_ind(label: AssemblyLabel) -> AssemblyInstruction {
    AssemblyInstruction::STA(
        AssemblyValue::Label(
            label
        ),
        true
    )
}

pub fn store_retval() -> AssemblyInstruction {
    store(retval())
}


pub fn store_sp() -> AssemblyInstruction {
    store(sp())
}

pub fn store_spo() -> AssemblyInstruction {
    store(spo())
}

pub fn store_spt() -> AssemblyInstruction {
    store(spt())
}


pub fn store_ind_sp() -> AssemblyInstruction {
    store_ind(sp())
}

pub fn store_ind_spo() -> AssemblyInstruction {
    store_ind(spo())
}


pub fn store_ind_spt() -> AssemblyInstruction {
    store_ind(spt())
}


pub fn load_spo() -> AssemblyInstruction {
    load(spo())
}


pub fn load_ind_spo() -> AssemblyInstruction {
    load_ind(spo())
}

pub fn load_spt() -> AssemblyInstruction {
    load(spt())
}

pub fn load_ind_spt() -> AssemblyInstruction {
    load_ind(spt())
}


pub fn load_temp1() -> AssemblyInstruction {
    load(temp1())
}

    
pub fn store_temp1() -> AssemblyInstruction {
    store(temp1())
}

pub fn load_temp2() -> AssemblyInstruction {
    load(temp2())
}

pub fn store_temp2() -> AssemblyInstruction {
    store(temp2())
}

impl CodeGenerator {


    pub fn dec_sp(&mut self,
        asm_program: &mut AssemblyProgram,
        origin: usize
    ) {

        self.add_line(
            asm_program,
            load_sp(),
            origin
        );
        self.add_line(
            asm_program,
            cmp_ac(),
            origin
        );
        self.add_line(
            asm_program,
            inc_ac(),
            origin
        );
        self.add_line(
            asm_program,
            cmp_ac(),
            origin
        );
        self.add_line(
            asm_program,
            store_sp(),
            origin
        );                
    }
    
    pub fn inc_sp(&mut self,
        asm_program: &mut AssemblyProgram,
        origin: usize
    ) {

        self.add_line(
            asm_program,
            load_sp(),
            origin
        );
        self.add_line(
            asm_program,
            inc_ac(),
            origin
        );
        self.add_line(
            asm_program,
            store_sp(),
            origin
        );                
    }

    pub fn param_in(&mut self,
        asm_program: &mut AssemblyProgram,
        label: AssemblyLabel,
        origin: usize
    ) {
        self.dec_sp(asm_program, origin);        
        self.calc_spt_spo(asm_program, label, origin);
        self.add_line(
            asm_program,
            load_ind_spo(),
            origin
        );
        
        self.add_line(
            asm_program,
            store_ind_sp(),
            origin
        );
    }

    pub fn save_ret(&mut self,
        asm_program: &mut AssemblyProgram,
        label: AssemblyLabel,
        origin: usize
    ) {
        self.dec_sp(asm_program, origin);        
        self.add_line(
            asm_program,
            load(label),
            origin
        );        
        self.add_line(
            asm_program,
            store_ind_sp(),
            origin
        );

    }

    pub fn branch_func(&mut self,
        asm_program: &mut AssemblyProgram,
        label: AssemblyLabel,
        origin: usize
    ) {
        self.dec_sp(asm_program, origin);        
        self.add_line(
            asm_program,
            bsa_ind_sp(),
            origin
        );        
    }
    

    
    
    pub fn store_sp_spt(&mut self,
        asm_program: &mut AssemblyProgram,        
        origin: usize
    ) {

        self.add_line(
            asm_program,
            load_sp(),
            origin
        );
        self.add_line(
            asm_program,
            store_spt(),
            origin
        );                

    }

    pub fn calc_spt_spo(&mut self,
        asm_program: &mut AssemblyProgram,
        label: AssemblyLabel,
        origin: usize
    ) {

        self.add_line(
            asm_program,
            load_spt(),
            origin
        );
        self.add_line(
            asm_program,
            add(label),
            origin
        );
        self.add_line(
            asm_program,
            store_spo(),
            origin
        );                
    }

    pub fn calc_load_tmp1(&mut self,
        asm_program: &mut AssemblyProgram,
        label: AssemblyLabel,
        origin: usize
    ) {
        self.calc_spo(asm_program,label,origin);
        self.load_stack_temp1(asm_program,origin);

    }
    pub fn calc_load_tmp2(&mut self,
        asm_program: &mut AssemblyProgram,
        label: AssemblyLabel,
        origin: usize
    ) {
        self.calc_spo(asm_program,label,origin);
        self.load_stack_temp2(asm_program,origin);

    }

    
    pub fn calc_spo(&mut self,
        asm_program: &mut AssemblyProgram,
        label: AssemblyLabel,
        origin: usize
    ) {

        self.add_line(
            asm_program,
            load_sp(),
            origin
        );
        self.add_line(
            asm_program,
            add(label),
            origin
        );
        self.add_line(
            asm_program,
            store_spo(),
            origin
        );                
    }

    pub fn load_stack_temp1(&mut self,
        asm_program: &mut AssemblyProgram,
        origin: usize
    ) {
        self.add_line(
            asm_program,
            load_ind_spo(),
            origin
        );
        self.add_line(
            asm_program,
            store_temp1(),
            origin
        );                
    }

    fn load_stack_temp2(&mut self,
        asm_program: &mut AssemblyProgram,
        origin: usize
    ) {
        self.add_line(
            asm_program,
            load_ind_spo(),
            origin
        );
        self.add_line(
            asm_program,
            store_temp2(),
            origin
        );                
    }

    pub fn add_temps(&mut self,
        asm_program: &mut AssemblyProgram,
        origin: usize
    ) {
        self.add_line(
            asm_program,
            load_temp1(),
            origin
        );        
        self.add_line(
            asm_program,
            add_temp2(),
            origin
        );                
        self.add_line(
            asm_program,
            store_temp1(),
            origin
        );                
    }

    pub fn load_retval(&mut self,
        asm_program: &mut AssemblyProgram,
        label: AssemblyLabel,
        origin: usize
    ) {        
        self.calc_spo(asm_program,label, origin);
        self.add_line(
            asm_program,
            load_retval(),
            origin
        );
        self.add_line(
            asm_program,
            store_ind_spo(),
            origin
        );
    }

    pub fn store_retval(&mut self,
        asm_program: &mut AssemblyProgram,
        label: AssemblyLabel,
        origin: usize
    ) {        
        self.calc_spo(asm_program,label, origin);
        self.add_line(
            asm_program,
            load_ind(label),
            origin
        );
        self.add_line(
            asm_program,
            store_retval(),
            origin
        );

    }
    

    
    pub fn store_temp1_stack(&mut self,
        asm_program: &mut AssemblyProgram,
        label: AssemblyLabel,
        origin: usize
    ) {

        self.calc_spo(asm_program, label, origin);
        
        self.add_line(
            asm_program,
            load_temp1(),
            origin
        );        
        self.add_line(
            asm_program,
            store_ind(spo()),
            origin
        );                
    }



}
