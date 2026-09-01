mod lexer;
mod parser;
mod cs_analyzer;
mod ir_generator;
mod code_generator;
mod assembler;
mod token;
mod ast;
mod symtable;
mod iloc;
mod assembly;
mod asm_table;
mod machine_code;

use::std::ops::Range;
use std::sync::mpsc;
use std::error::Error;

use crate::compiler::machine_code::MachineProgram;
use crate::compiler::asm_table::AsmTable;
use crate::compiler::assembly::AssemblyProgram;
use crate::compiler::code_generator::CodeGenerator;
use crate::compiler::iloc::IlocProgram;
use crate::compiler::symtable::FuncTable;
use crate::compiler::token::TokenList;
use crate::compiler::ast::Ast;
    
#[derive(Debug, Clone, PartialEq)]
pub enum OrderMessage {
    Setup,
    Go,
    Exit,
}

#[derive(Debug, PartialEq)]
pub enum Phase {
    Lexer(lexer::LexicalAnalyzer),
    Parser(parser::Parser),
    CsAnalyzer(cs_analyzer::CsAnalyzer),
    IrGenerator(ir_generator::IrGenerator),
    CodeGenerator(code_generator::CodeGenerator),
    Assembler(assembler::Assembler),
    Finished,
    Error(CompileError),
}

  
#[derive(Debug,Clone,PartialEq)]
pub enum CompileError {
    LexError(lexer::LexError),
    ParseError(parser::ParseError),
    CSError(cs_analyzer::CsError),
    IRGenError(ir_generator::IrGenError),
    CodeGenError(code_generator::CodeGenError),
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum SnapShotPhase {
    Lexer,
    Parser,
    CsAnalyzer,
    IrGenerator,
    CodeGenerator,
    Assembler,
    Finished,
    Error,
}

#[derive(Debug,Clone,PartialEq)]
pub struct CompileSnapShot {
    phase: SnapShotPhase,
    error: Option<CompileError>,
    text:  String,
    token_list: TokenList,
    

}

impl CompileSnapShot {

    pub fn new(cmp: &Compiler) -> Self{
        CompileSnapShot {
            phase: match & cmp.phase {
                Phase::Lexer(_) => SnapShotPhase::Lexer,
                Phase::Parser(_) => SnapShotPhase::Parser,
                Phase::CsAnalyzer(_) => SnapShotPhase::CsAnalyzer,
                Phase::IrGenerator(_) => SnapShotPhase::IrGenerator,
                Phase::CodeGenerator(_) => SnapShotPhase::CodeGenerator,
                Phase::Assembler(_) => SnapShotPhase::Assembler,
                Phase::Finished => SnapShotPhase::Finished,
                Phase::Error(_) => SnapShotPhase::Error,                
            },
            error: match & cmp.phase {
                Phase::Error(e) => Some(e.clone()),
                _ => None,
            },
            text: cmp.text.clone(),
            token_list: cmp.token_list.clone(),
        }
    }


    
}


impl Phase {

    pub fn shift_phase(&mut self) {
        let next_state = match self {
            Phase::Lexer(_) => Phase::Parser(parser::Parser::new()),
            Phase::Parser(_) => Phase::CsAnalyzer(cs_analyzer::CsAnalyzer::new()),
            Phase::CsAnalyzer(_) => Phase::IrGenerator(ir_generator::IrGenerator::new()),
            Phase::IrGenerator(_) => Phase::CodeGenerator(code_generator::CodeGenerator::new()),
            Phase::CodeGenerator(_) => Phase::Assembler(assembler::Assembler::new()),
            Phase::Assembler(_) => Phase::Finished,
            Phase::Finished => Phase::Finished,
            Phase::Error(e) => Phase::Error(e.clone()),
        };

        std::mem::replace(self, next_state);
    }


}


pub fn compile_session(
    compiler_tx: mpsc::Sender<CompileSnapShot>,
    order_rx: mpsc::Receiver<OrderMessage>,
    contents: String,
) -> Result<(),Box<dyn Error>>
{
    let mut cmp = Compiler::new(&contents);

    loop {
        let order = order_rx.recv()?;
        match order{
            OrderMessage::Setup => {cmp  = Compiler::new(&contents)},
            OrderMessage::Go => cmp.run(&compiler_tx)?, 
            OrderMessage::Exit => return Ok(()), 
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Compiler{
    phase: Phase,
    text:  String,
    token_list: TokenList,
    ast: Ast,
    func_table: FuncTable,
    iloc: IlocProgram,
    assembly: AssemblyProgram,
    asm_table: AsmTable,
    machine_code: MachineProgram,
}

impl Compiler{
    fn new(text: &str) -> Self {
        Compiler {
            

            phase: Phase::Lexer(lexer::LexicalAnalyzer::new()),
            text: Compiler::trimmer(text),
            token_list: TokenList::new(),
            ast: Ast::new(),
            func_table: FuncTable::new(),
            iloc: IlocProgram::new(),
            assembly: AssemblyProgram::new(),
            asm_table: AsmTable::new(),
            machine_code: MachineProgram::new(),
        }
        
    }
    
    fn run(
        &mut self,
        compiler_tx: &mpsc::Sender<CompileSnapShot>
    ) -> Result<(), Box<dyn Error>>{
        
        if let Some(response) = self.next() {
            compiler_tx.send(response)?
        }
        else {
            compiler_tx.send(CompileSnapShot::new(self))?;
        }

        
        Ok(())
    }

    fn trimmer(text: &str) -> String {
        let trimmed_text  = text
            .lines()
            .map(|line| {
                if let Some(idx) = line.find("//") {
                    &line[..idx]
                } else {
                    line
                }                             
            })
            .collect::<Vec<&str>>()
            .join("\n");
        
        trimmed_text
    }

}

impl Iterator for Compiler{
    type Item = CompileSnapShot;

    fn next(&mut self) -> Option<Self::Item>{

        match &self.phase {
            Phase::Finished | Phase::Error(_) => return None,
            _ => {},
        }
        
        
        let result = match &mut self.phase {
            Phase::Lexer(lexer) => lexer.step(&self.text,&mut self.token_list),
            Phase::Parser(parser) => parser.step(&self.token_list,&mut self.ast),
            Phase::CsAnalyzer(cs_analyzer) => cs_analyzer.step(&self.ast,&mut self.func_table),
            Phase::IrGenerator (ir_generator) => ir_generator.step(&self.ast,&self.func_table,&mut self.iloc),
            Phase::CodeGenerator(code_generator) => code_generator.step(&self.iloc, &mut self.assembly),
            Phase::Assembler(assembler) => assembler.step(&self.assembly,&mut self.asm_table,&mut self.machine_code),
            _ => None,
        };

        if let Some(step_result) = result 
        {
            match step_result {
                Ok(_) => Some(CompileSnapShot::new(self)),
                Err(e) => {
                    self.phase = Phase::Error(e.clone());
                    Some(CompileSnapShot::new(self))},
            }                
        } else {
            self.phase.shift_phase();
            Some(CompileSnapShot::new(self))
        }        
    }

}

#[derive(Debug,Clone,PartialEq)]
pub struct SpanInfo {
    produced_by: SnapShotPhase,
    start: Option<usize>,
    end: Option<usize>,
}

impl SpanInfo {
    fn new (produced_by: SnapShotPhase, start: Option<usize>, end: Option<usize>) -> Self{
        SpanInfo {
            produced_by,
            start,
            end,
        }
    }

    pub fn get_start(&self) -> Option<usize> {
        self.start
    }
    pub fn get_end(&self) -> Option<usize> {
        self.end
    }

    pub fn set_start(&mut self, pos: usize){
        self.start = Some(pos);
    }
    pub fn set_end(&mut self, pos: usize){
        self.end = Some(pos);
    }



}

pub trait ProducedBy {
    fn produced_by(&self) -> Option<SpanInfo>;
}
