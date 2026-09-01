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

    pub fn get_global_pos(&self, func: usize, line: usize) -> Option<usize> {
        let mut result: usize = 0;
        
        for i in 0 as usize..func {
            result = result + self.functions.get(i)?.len();
        }
        result = result + line;

        Some(result)
    }

    pub fn find_by_global_pos(&self, pos: usize) -> Option<(usize,usize)> {
        let mut result = pos;

        let mut entry = 0;
        
        for i in 0 as usize..self.functions.len() {
            entry = i;
            let func_line = self.functions.get(i)?.len();

            result = result - func_line;

            if result < 0 {
                result = result + func_line;
                break;
            }
        }

        Some((entry,result))
    }
    
    pub fn get_mut_bypos(&mut self, pos: usize) -> Option<&mut IlocFunction> {
        self.functions.get_mut(pos)
    }

    pub fn get_bypos(&self, pos: usize) -> Option<&IlocFunction> {
        self.functions.get(pos)
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
    BuiltIn(String)
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
        if comparison > self.get_regmax() {
            self.reg_max = comparison
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

