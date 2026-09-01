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

mod function;
mod variable;
mod procedure_call;
mod func_return;
mod goto;
mod built_ins;

impl CodeGenerator {

    
    pub fn match_ir_label(&mut self,
        entry: usize,
        ir_label: &IlocLabel,
        line_number: usize,
        asm_program: &mut AssemblyProgram
    ) -> Result<(),CodeGenError> {

            match ir_label {
                IlocLabel::FunctionStart(_) => self.func_start(entry,asm_program),
                IlocLabel::FunctionPrologue => {self.epilogue(entry,line_number,asm_program)},
                IlocLabel::FunctionEpilogue => {self.prologue(entry,line_number,asm_program)},
                IlocLabel::Variable(idx, Register(target)) => {self.variable(entry,line_number,asm_program,*idx,*target)},
                IlocLabel::Paremeter(idx, Register(target)) => {self.parameter(entry,line_number,asm_program,*idx,*target)},
                IlocLabel::ParamIn(Register(source)) => {self.parin(entry,line_number,asm_program,*source)},
                IlocLabel::Call(callee, Register(target)) => {self.caller(entry,line_number,asm_program,*callee,*target)},
                IlocLabel::Return(Register(source)) => self.save_retval(entry,line_number,asm_program,*source),
                IlocLabel::Goto(GotoLabel(idx)) => self.goto_label(entry,line_number,asm_program,*idx),
                IlocLabel::BuiltIn(name) => {self.built_in(entry,line_number,asm_program,name)},

            }
    }

}
