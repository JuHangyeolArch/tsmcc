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

mod ir_label;
mod idiom;
mod func_data;
mod arithmetic;
mod memory;
mod condition;
mod jump;


impl CodeGenerator {

    pub fn asm_translate(&mut self, ir_program: &IlocProgram, asm_program: &mut AssemblyProgram) -> Result<(),CodeGenError> {

        self.match_ir_line(
            self.cur_ir_func,
            ir_program
                .get_bypos(self.cur_ir_func)
                .unwrap().get_line(self.cur_ir_line)
                .unwrap(),
            ir_program.get_global_pos(self.cur_ir_func,self.cur_ir_line)
                .unwrap(),
            asm_program
        )?;
        
        let Some(next) = self.next_line(ir_program) else {
            asm_program.add_line(None, AssemblyInstruction::END, None);
            self.is_over = true;
            return Ok(());
        };

        if self.cur_ir_func < next.0 {
            self.func_data(self.cur_ir_func,self.cur_ir_line,asm_program);

        }
        
        (self.cur_ir_func,self.cur_ir_line) = next;

        Ok(())
               
    }

    fn next_line(&self, ir_program: &IlocProgram) -> Option<(usize,usize)> {
        let cur_global_line = ir_program
            .get_global_pos(self.cur_ir_func,self.cur_ir_line)?;

        ir_program
            .find_by_global_pos(cur_global_line + 1)
    }

    fn add_line(&mut self,
        asm_program: &mut AssemblyProgram,
        instruction: AssemblyInstruction,
        origin: usize
    ) {

        asm_program
            .add_line(self.temp_label, instruction, Some(origin));

        self.temp_label = None;
    }


    fn match_ir_line(&mut self, entry: usize, ir_line: &IlocLine, line_number: usize, asm_program: &mut AssemblyProgram) -> Result<(),CodeGenError> {

        match ir_line.get_instruction() {
            IlocInstruction::Empty => { self.match_ir_label(entry,&ir_line.get_label().unwrap(),line_number,asm_program)},
            IlocInstruction::Add{source1: Register(r1), source2: Register(r2), target: Register(rt)} => {self.adder(entry,line_number,asm_program,r1,r2,rt)},
            IlocInstruction::Sub {source1: Register(r1), source2: Register(r2), target: Register(rt)} => {self.substractor(entry,line_number,asm_program,r1,r2,rt)},
            IlocInstruction::Mult{source1: Register(r1), source2: Register(r2), target: Register(rt)} => {self.multiplier(entry,line_number,asm_program,r1,r2,rt)},
            IlocInstruction::Div {source1: Register(r1), source2: Register(r2), target: Register(rt)} => {self.divider(entry,line_number,asm_program,r1,r2,rt)},

            IlocInstruction::Load { source: Register(r1), target: Register(rt) }  => { self.loader(entry,line_number,asm_program,r1,rt)},
            //상수
            IlocInstruction::LoadI { constant: val , target: Register(rt)} => {self.loader_immmediate(entry,line_number,asm_program,val as u16,rt)},
            IlocInstruction::Store { source: Register(r1), target: Register(rt) } => {self.store(entry,line_number,asm_program,r1,rt)},
            IlocInstruction::CmpEQ {source1: Register(r1), source2: Register(r2), target: Register(rt)} => {self.cmp_eq(entry,line_number, asm_program,r1,r2,rt)},
            IlocInstruction::Cbr { source: Register(r), target_l1: GotoLabel(idx1), target_l2: GotoLabel(idx2) } => {self.cond_branch(entry,line_number,asm_program,r,idx1,idx2)},
            IlocInstruction::Jump { target : Register(rt)} => {self.jump(entry,line_number,asm_program,rt)},
            IlocInstruction::JumpI { targetl: GotoLabel(idx)} => {self.jump_immediate(entry,line_number,asm_program,idx)} 
        }
            
    }
    
}


