use super::SpanInfo;
use super::SnapShotPhase;
use super::ProducedBy;


use crate::compiler::ast::NodeID;

use std::option::Iter;

#[derive(Debug, Clone, PartialEq)]
pub struct IlocProgram {
    functions: Vec<IlocFunction>,
}

impl IlocProgram {
    pub fn new() -> Self {
       IlocProgram {
            functions: Vec::new()
       }
    }

    pub fn len(&self) -> usize {
        self.functions.len()
    }

    pub fn push_func(&mut self, name: &str, entry: usize, param_size: usize, local_size: usize) {
        self.functions.push(IlocFunction::new(name, entry, param_size, local_size))
    }

    pub fn count_func(&self) -> usize{
        self.functions.len()
    }

    

    pub fn get_by_global(&self, pos: usize) -> Option<IlocLine> {
        if let Some((entry,line)) = self.find_by_global_pos(pos) {
            Some(self.get_bypos(entry)
                .unwrap()
                .get_line(line)
                .unwrap()
                .clone())
        } else {
            None
        }
    }
    
    pub fn get_global_pos(&self, func: usize, line: usize) -> Option<usize> {
        let mut result: usize = 0;
        
        for i in 0..func {
            result = result + self.functions.get(i)?.len();
        }
        
        result = result + line;
        
        Some(result)
    }

    pub fn find_by_global_pos(&self, pos: usize) -> Option<(usize,usize)> {
        let mut result = pos as i32;
        
        for i in 0..self.functions.len() as i32 {
            let func_line = self.functions.get(i as usize)?.len();
            
            if (result - func_line as i32) < 0 {
                return Some((i as usize,result as usize));
            } else {
                result = result - func_line as i32;
            }
        }

        None
    }
    
    pub fn get_mut_bypos(&mut self, pos: usize) -> Option<&mut IlocFunction> {
        self.functions.get_mut(pos)
    }

    pub fn get_bypos(&self, pos: usize) -> Option<&IlocFunction> {
        self.functions.get(pos)
    }

    pub fn as_iter(&self) -> impl Iterator<Item = IlocLine> {
        let mut combined: Box<dyn Iterator<Item = IlocLine>> = Box::new(std::iter::empty());

        for f in self.functions.iter() {
            combined = Box::new(combined.chain(f.lines.clone().into_iter()));
        }
        combined
    }
    
}


#[derive(Debug, Clone, PartialEq)]
pub enum IlocLabel {
    FunctionStart(usize),
    FunctionPrologue,
    FunctionEpilogue,
    Variable(usize,Register),
    Paremeter(usize,Register),
    ParamIn(Register),
    Call(usize,Register),
    Return(Register),
    Goto(GotoLabel),
    BuiltIn(String),
    NoOp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GotoLabel(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub struct IlocFunction {
    name: String,
    entry: usize,
    lines: Vec<IlocLine>,
    param_size: usize,
    local_size: usize,
    reg_max: usize,
    internal_branch_count: usize,
}

impl IlocFunction {

    pub fn new(name: &str, entry: usize, param_size: usize, local_size: usize) -> Self {
        IlocFunction {
            name: name.to_string(),
            entry: entry,
            lines: Vec::new(),
            param_size: param_size,
            local_size: local_size,
            reg_max: 0,
            internal_branch_count: 0,
        }
    }
    

    pub fn get_internal_bc(&self) -> usize {
        self.internal_branch_count
    }
    
    pub fn inc_internal_bc(&mut self) -> usize {
        let cur_bc = self.get_internal_bc();
        self.internal_branch_count = self.internal_branch_count + 1;
        cur_bc
    }

    pub fn get_localsize(&self) -> usize {
        self.local_size
    }    

    pub fn get_paramsize(&self) -> usize {
        self.param_size
    }    
    
    pub fn get_regmax(&self) -> usize {
        self.reg_max
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }
    
    
    pub fn update_regmax(&mut self, comparison: usize) {
        if comparison + 1 > self.get_regmax() {
            self.reg_max = comparison + 1
        } 
    }
    
    pub fn add_line(&mut self, instruction: IlocInstruction, label: Option<IlocLabel>,origin: NodeID) {
        self.lines.push(IlocLine::new(instruction, label,origin))
    }

    pub fn get_line(&self, idx: usize) -> Option<&IlocLine> {
        self.lines.get(idx)
    }

    
    pub fn iter(&self) -> impl Iterator<Item = &IlocLine>{
        self.lines.iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IlocLine {
    instruction: IlocInstruction,
    label: Option<IlocLabel>,
    span: SpanInfo
}

impl IlocLine {
    pub fn get_instruction(&self) -> IlocInstruction {
        self.instruction.clone()
    }

    pub fn set_label(&mut self, label: IlocLabel) {
        self.label = Some(label)
    }

    pub fn get_label(&self,) -> Option<IlocLabel> {
        self.label.clone()
    }
    
    fn new(instruction: IlocInstruction, label: Option<IlocLabel>, origin: NodeID) -> Self {
        IlocLine {
            instruction,
            label,
            span: SpanInfo::new(SnapShotPhase::IrGenerator,Some(origin.to_usize()), Some(origin.to_usize() + 1))
        }
    }


}

impl ProducedBy for IlocLine {
    fn produced_by(&self) ->  Option<SpanInfo> {
        Some(self.span.clone())
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct Register(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub enum IlocInstruction {

    //
    Empty,

    //Arithmetic
    Add {
        source1: Register,
        source2: Register,
        target: Register,
    },

    
    Sub {
        source1: Register,
        source2: Register,
        target: Register,        
    },

    
    Mult {
        source1: Register,
        source2: Register,
        target: Register,        
    },


    Div {
        source1: Register,
        source2: Register,
        target: Register,        
    },

    //Load and Store

    Load {
        source: Register,
        target: Register,
    },

    LoadI {
        constant: i32,
        target: Register,
    },
    
    Store {
        source: Register,
        target: Register,
    },

    //Control and Branch

    CmpEQ {
        source1: Register,
        source2: Register,
        target: Register,        
    },
    
    Cbr {
        source: Register,
        target_l1: GotoLabel,
        target_l2: GotoLabel,
    },
    
    //Jump
    Jump {
        target: Register
    },

    JumpI {
        targetl: GotoLabel
    }
   
}

