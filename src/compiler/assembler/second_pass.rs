use crate::compiler::asm_table::{
AsmTable
};

use crate::compiler::assembly::{

    AssemblyInstruction,
    AssemblyLabel,
    AssemblyLine,
    AssemblyProgram,
    AssemblyValue,
    U12
    
};

use crate::compiler::machine_code::{
    MachineLine,
    MachineProgram,
    
};

use crate::compiler::assembler::Assembler;


impl Assembler {

    pub fn second_pass(&mut self,
        asm_program: &AssemblyProgram,
        asm_table: &mut AsmTable,
        machine_code: &mut MachineProgram
    ) {

        for (idx,line) in asm_program.iter().enumerate() {

            

            match line.get_inst() {

                AssemblyInstruction::ORG(address) => { self.program_counter = address.value(); continue;},
                AssemblyInstruction::END => { self.second_ended = true; return;},
                AssemblyInstruction::Data(val) => machine_code.push(U12::new(self.program_counter) , val , idx),
                AssemblyInstruction::AND(val, indirect) => machine_code.push(U12::new(self.program_counter) , 0x0000 + if indirect {0x8000} else {0x0000} + asm_table.resolve_value(val), idx) ,
                AssemblyInstruction::ADD(val, indirect) => machine_code.push(U12::new(self.program_counter) , 0x1000 + if indirect {0x8000} else {0x0000} + asm_table.resolve_value(val), idx) ,
                AssemblyInstruction::LDA(val, indirect) => machine_code.push(U12::new(self.program_counter) , 0x2000 + if indirect {0x8000} else {0x0000} + asm_table.resolve_value(val), idx) ,
                AssemblyInstruction::STA(val, indirect) => machine_code.push(U12::new(self.program_counter) , 0x3000 + if indirect {0x8000} else {0x0000} + asm_table.resolve_value(val), idx) ,
                AssemblyInstruction::BUN(val, indirect) => machine_code.push(U12::new(self.program_counter) , 0x4000 + if indirect {0x8000} else {0x0000} + asm_table.resolve_value(val), idx) ,
                AssemblyInstruction::BSA(val, indirect) => machine_code.push(U12::new(self.program_counter) , 0x5000 + if indirect {0x8000} else {0x0000} + asm_table.resolve_value(val), idx) ,
                AssemblyInstruction::ISZ(val, indirect) => machine_code.push(U12::new(self.program_counter) , 0x6000 + if indirect {0x8000} else {0x0000} + asm_table.resolve_value(val), idx) ,

                                
                AssemblyInstruction::CLA => machine_code.push(U12::new(self.program_counter) ,0x7800,idx),
                AssemblyInstruction::CLE => machine_code.push(U12::new(self.program_counter) ,0x7400,idx),
                AssemblyInstruction::CMA => machine_code.push(U12::new(self.program_counter) ,0x7200,idx),
                AssemblyInstruction::CME => machine_code.push(U12::new(self.program_counter) ,0x7100,idx),
                AssemblyInstruction::CIR => machine_code.push(U12::new(self.program_counter) ,0x7080,idx),
                AssemblyInstruction::CIL => machine_code.push(U12::new(self.program_counter) ,0x7040,idx),
                AssemblyInstruction::INC => machine_code.push(U12::new(self.program_counter) ,0x7020,idx),
                AssemblyInstruction::SPA => machine_code.push(U12::new(self.program_counter) ,0x7010,idx),
                AssemblyInstruction::SNA => machine_code.push(U12::new(self.program_counter) ,0x7008,idx),
                AssemblyInstruction::SZA => machine_code.push(U12::new(self.program_counter) ,0x7004,idx),
                AssemblyInstruction::SZE => machine_code.push(U12::new(self.program_counter) ,0x7802,idx),
                AssemblyInstruction::HLT => machine_code.push(U12::new(self.program_counter) ,0x7801,idx),
                AssemblyInstruction::INP => machine_code.push(U12::new(self.program_counter) ,0xF800,idx),
                AssemblyInstruction::OUT => machine_code.push(U12::new(self.program_counter) ,0xF400,idx),
                AssemblyInstruction::SKI => machine_code.push(U12::new(self.program_counter) ,0xF200,idx),
                AssemblyInstruction::SKO => machine_code.push(U12::new(self.program_counter) ,0xF100,idx),
                AssemblyInstruction::ION => machine_code.push(U12::new(self.program_counter) ,0xF080,idx),
                AssemblyInstruction::IOF => machine_code.push(U12::new(self.program_counter) ,0xF040,idx),
                _ => {},
                
            }

            self.program_counter = self.program_counter + 1;
            
        }
        

    }

}
