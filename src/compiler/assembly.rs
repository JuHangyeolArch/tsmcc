use crate::compiler::ProducedBy;

use crate::compiler::SpanInfo;
use crate::compiler::CompileError;



#[derive(Debug, Clone, PartialEq)]
pub struct AssemblyProgram {
    lines: Vec<AssemblyLine>,
    mem_size: usize,

}

impl AssemblyProgram {
    pub fn new() -> Self {
        AssemblyProgram {
            lines: Vec::new(),
            mem_size: 0,
        }        
    }

    pub fn add_line(&mut self,
        label: Option<AssemblyLabel>,
        instruction: AssemblyInstruction,
        origin: Option<usize>
    ){
        self.mem_size = match instruction {
            AssemblyInstruction::ORG(pos) => pos.value() as usize,
            AssemblyInstruction::END => self.mem_size, 
            _ => self.mem_size + 1,
        };
        
        self.lines.push(AssemblyLine::new(label,instruction, origin));
    }

    pub fn last_line(&self) -> Option<&AssemblyLine> {

        if self.count_line() > 0 {
            self.get_line(self.count_line() - 1)
        } else {
            None
        }
    }
    
    pub fn get_line(&self, pos: usize) -> Option<&AssemblyLine>{
        self.lines.get(pos)
    }

    pub fn count_line(&self) -> usize {
        self.lines.len()
    }

    pub fn get_memsize(&self) -> usize {
        self.mem_size
    }
    
    pub fn to_string(&self) -> Vec<String> {
        self.lines
            .iter()
            .map(|item| item.to_string())
            .collect()
    }

    
    pub fn iter(&self) -> impl Iterator<Item = &AssemblyLine> {
        self.lines.iter()
        
    }


}


#[derive(Debug, Clone, PartialEq)]
pub struct AssemblyLine {
    label: Option<AssemblyLabel>,
    instruction: AssemblyInstruction,
    span: SpanInfo,
}

impl AssemblyLine {

    pub fn new(
        label: Option<AssemblyLabel>,
        instruction: AssemblyInstruction,
        origin: Option<usize>
    ) -> Self {

        let end = if let Some(pos) = origin {
            Some(pos + 1)
        }else {
            None
        };
        
        AssemblyLine {
            label,
            instruction,
            span: SpanInfo::new(
                super::SnapShotPhase::CodeGenerator,
                origin,
                end
            )
        }

    }

    pub fn add_label(&mut self, label: AssemblyLabel) {
        self.label = Some(label);
    }

    pub fn get_inst(&self) -> AssemblyInstruction {
        self.instruction.clone()
    }


    pub fn get_label(&self) -> Option<AssemblyLabel> {
        self.label
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        
        if let Some(label) = self.label {
            result.push_str(&label.to_string());
            result.push_str(", ");
        };

        let instruction = match &self.instruction {
            AssemblyInstruction::Data(val) => format!("{:X}", val),
            AssemblyInstruction::ORG(val) => {"ORG ".to_string() + &val.to_hex_uppercase()},
            AssemblyInstruction::END => "END".to_string() ,
            AssemblyInstruction::AND(val, indirect) => {"AND ".to_string() + &val.to_hex_uppercase() + if *indirect {&" I"} else {&""}},
            AssemblyInstruction::ADD(val, indirect) => {"ADD ".to_string() + &val.to_hex_uppercase() + if *indirect {&" I"} else {&""}},
            AssemblyInstruction::LDA(val, indirect) => {"LDA ".to_string() + &val.to_hex_uppercase() + if *indirect {&" I"} else {&""}},
            AssemblyInstruction::STA(val, indirect) => {"STA ".to_string() + &val.to_hex_uppercase() + if *indirect {&" I"} else {&""}},
            AssemblyInstruction::BUN(val, indirect) => {"BUN ".to_string() + &val.to_hex_uppercase() + if *indirect {&" I"} else {&""}},
            AssemblyInstruction::BSA(val, indirect) => {"BSA ".to_string() + &val.to_hex_uppercase() + if *indirect {&" I"} else {&""}},
            AssemblyInstruction::ISZ(val, indirect) => {"ISZ ".to_string() + &val.to_hex_uppercase() + if *indirect {&" I"} else {&""}},
            AssemblyInstruction::CLA => "CLA".to_string(),
            AssemblyInstruction::CLE => "CLE".to_string(),
            AssemblyInstruction::CMA => "CMA".to_string(),
            AssemblyInstruction::CME => "CME".to_string(),
            AssemblyInstruction::CIR => "CIR".to_string(),
            AssemblyInstruction::CIL => "CIL".to_string(),
            AssemblyInstruction::INC => "INC".to_string(),
            AssemblyInstruction::SPA => "SPA".to_string(),
            AssemblyInstruction::SNA => "SNA".to_string(),
            AssemblyInstruction::SZA => "SZA".to_string(),
            AssemblyInstruction::SZE => "SZE".to_string(),
            AssemblyInstruction::HLT => "HLT".to_string(),
            AssemblyInstruction::INP => "INP".to_string(),
            AssemblyInstruction::OUT => "OUT".to_string(),
            AssemblyInstruction::SKI => "SKI".to_string(),
            AssemblyInstruction::SKO => "SKO".to_string(),
            AssemblyInstruction::ION => "ION".to_string(),
            AssemblyInstruction::IOF => "IOF".to_string(),

        };

        result.push_str(&instruction);

        result
    }

    
}


#[derive(Debug, Clone, PartialEq)]
pub enum AssemblyValue {
    Label(AssemblyLabel),
    Constant(U12)
}

impl AssemblyValue {

    pub fn to_hex_uppercase(&self) -> String {
        match self {
            AssemblyValue::Label(label) => label.to_string(),
            AssemblyValue::Constant(val) => val.to_hex_uppercase(),
        }

    }

}


#[derive(Debug, Clone, Eq, Hash, Copy, PartialEq)]
pub struct AssemblyLabel(pub char,pub char,pub char);

impl AssemblyLabel {

    pub fn to_string(&self) -> String {
        self.0.to_ascii_uppercase().to_string() + &self.1.to_ascii_uppercase().to_string() + &self.2.to_ascii_uppercase().to_string()
    }
   
}


#[derive(Debug, Clone, PartialEq)]
pub enum AssemblyInstruction {

    //Data
    Data(u16),
    
    //Pseudo
    ORG(U12),
    END,
    
    //Memref
    AND(AssemblyValue, bool),
    ADD(AssemblyValue, bool),
    LDA(AssemblyValue, bool),
    STA(AssemblyValue, bool),
    BUN(AssemblyValue, bool),
    BSA(AssemblyValue, bool),
    ISZ(AssemblyValue, bool),

    //Regref
    CLA,
    CLE,
    CMA,
    CME,
    CIR,
    CIL,
    INC,
    SPA,
    SNA,
    SZA,
    SZE,
    HLT,

    //InputOutputref
    INP,
    OUT,
    SKI,
    SKO,
    ION,
    IOF
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct U12(u16);

impl U12 {
    pub fn new(value: u16) -> Self {
        U12(value & 0x0FFF) 
    }

    pub fn value(&self) -> u16 {
        self.0 
    }

    pub fn to_hex_uppercase(&self) -> String {
        format!("{:X}", self.0)
    }
}

