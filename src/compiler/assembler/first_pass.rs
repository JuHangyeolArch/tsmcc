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

    pub fn first_pass(&mut self, asm_program: &AssemblyProgram, asm_table: &mut AsmTable) {

        for line in asm_program.iter() {

            match line.get_inst() {
                AssemblyInstruction::ORG(address) => { self.program_counter = address.value();}
                AssemblyInstruction::END => { self.first_ended = true; return;}
                _ => {},
            }
            
            if let Some(label) = line.get_label() {
                asm_table
                    .push(label,self.program_counter)
            };
            
            self.program_counter = self.program_counter + 1;
        }

    }


}
