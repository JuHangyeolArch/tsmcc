use std::collections::HashMap;

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


#[derive(Debug, Clone, PartialEq)]
pub struct LabelResolver {

    func_list: Vec<AsmFuncData>,
    
    // u 상수 할당받기
    constant_usage: usize,

    //r 레지스터 할당받기
    register_usage: usize,
    
    // v 변수 오프셋 할당받기
    variable_usage: usize,

    // w 파라미터 오프셋 할당받기
    parameter_usage: usize,

    // b 브랜치문마다 새 번호 할당받을 수 있도록 한다.
    branch_usage: usize,
    
}


impl LabelResolver {

    pub fn new() -> Self {
        LabelResolver {
            func_list: Vec::new(),
            constant_usage: 0,
            register_usage: 0,
            variable_usage: 0,
            parameter_usage: 0,
            branch_usage: 0,
        }
    }

    pub fn get_var_count(&self, entry: usize) -> usize {
        self.func_list.get(entry).unwrap().get_var_count()
    }

    pub fn get_par_count(&self, entry: usize) -> usize {
        self.func_list.get(entry).unwrap().get_par_count()
    }

    pub fn get_reg_count(&self, entry: usize) -> usize {
        self.func_list.get(entry).unwrap().get_reg_count()
    }

    pub fn get_cons_list(&self, entry: usize) -> Vec<(AssemblyLabel, u16)>  {
        self.func_list.get(entry).unwrap().get_cons_list()
    }


    pub fn get_reg_list(&self, entry: usize) -> HashMap<usize, (AssemblyLabel, u16)> {
        self.func_list.get(entry).unwrap().get_reg_list()
    }

    pub fn get_var_list(&self, entry: usize) -> HashMap<usize, (AssemblyLabel, u16)> {
        self.func_list.get(entry).unwrap().get_var_list()
    }

    pub fn get_par_list(&self, entry: usize) -> HashMap<usize, (AssemblyLabel, u16)> {
        self.func_list.get(entry).unwrap().get_par_list()
    }

    
    
    pub fn init_resolver(&mut self, ir_program: &IlocProgram) {
        
        for i in 0..ir_program.len() {
           let ir_func = ir_program
                .get_bypos(i)
               .unwrap();
            let asm_func = AsmFuncData::new();
            self.func_list.push(asm_func);

            self.init_var(i, ir_func);
            self.init_reg(i, ir_func);
            self.init_par(i, ir_func);
            self.init_bra(i, ir_func);
        }

    }

    fn init_var(&mut self, entry: usize, ir_func: &IlocFunction) {

        let size= ir_func.get_localsize();

        
        for i in 0..size {
            let cur = self.consume_variable();
            self.func_list
                .get_mut(entry)
                .unwrap()
                .gen_variable(ir_func,i,cur);
        }

    }

    fn init_reg(&mut self, entry: usize, ir_func: &IlocFunction) {

        let size = ir_func.get_regmax();

        
        for i in 0..size {
            let cur = self.consume_register();
            self.func_list
            .get_mut(entry)
            .unwrap()
            .gen_register(i,cur);
            
        }

    }

    fn init_par(&mut self, entry: usize, ir_func: &IlocFunction) {

        let size= ir_func.get_paramsize();
        
        for i in 0..size {
            
            let cur = self.consume_parameter();
            self.func_list
            .get_mut(entry)
            .unwrap()
            .gen_parameter(ir_func,i,cur);
            
        }

    }

    fn init_bra(&mut self, entry: usize, ir_func: &IlocFunction) {

        for i in 0..ir_func.get_internal_bc() {
            let cur = self.consume_branch();            
            self.func_list
            .get_mut(entry)
            .unwrap()
            .gen_bra(i,cur);
        }

    }


    
    fn consume_constant(&mut self) -> usize {
        let cur = self.constant_usage;
        self.constant_usage = self.constant_usage + 1;
        cur
    }
    fn consume_variable(&mut self) -> usize {
        let cur = self.variable_usage;
        self.variable_usage = self.variable_usage + 1;
        cur
    }
    fn consume_register(&mut self) -> usize {
        let cur = self.register_usage;
        self.register_usage = self.register_usage + 1;
        cur
    }
    fn consume_parameter(&mut self) -> usize {
        let cur = self.parameter_usage;
        self.parameter_usage = self.parameter_usage + 1;
        cur
    }    
    fn consume_branch(&mut self) -> usize {
        let cur = self.branch_usage;
        self.branch_usage = self.branch_usage + 1;
        cur
    }



    // function
    pub fn res_function(&self, entry: usize) ->  Option<AssemblyLabel> {

        let first  = usize_to_alphanumeric(entry / 36)?;
        let second = usize_to_alphanumeric(entry % 36)?;

        Some(AssemblyLabel('F',first,second))        
    }


    // fuction return label
    pub fn res_func_ret(&self, entry: usize) ->  Option<(AssemblyLabel, u16)> {

        let offset = (self.get_var_count(entry) + self.get_reg_count(entry))  as u16;
        
        let first  = usize_to_alphanumeric(entry / 36)?;

        let second = usize_to_alphanumeric(entry % 36)?;
        
        Some((AssemblyLabel('G',first,second), offset))
    }

    // function bsa 우회 명령어 label
    pub fn res_funcbun_ret(&self, entry: usize) ->  Option<AssemblyLabel> {
        
        let first  = usize_to_alphanumeric(entry / 36)?;

        let second = usize_to_alphanumeric(entry % 36)?;
        
        Some(AssemblyLabel('I',first,second))
    }

    
    // internal branch label
    pub fn res_branch(&self, entry: usize, idx: usize) ->  Option<(AssemblyLabel)> {

        let asm_func= self.func_list
            .get(entry)?;

        asm_func.get_bra(idx)
              
    }

    // local variale offset label
    pub fn res_variable(&mut self, entry: usize, idx: usize) ->  Option<(AssemblyLabel)> {

        let asm_func= self.func_list
            .get(entry)?;

        asm_func.get_var(idx)        
    }

    // spilled out register offset label
    pub fn res_register(&mut self, entry: usize, idx: usize) ->  Option<(AssemblyLabel)> {

        let asm_func= self.func_list
            .get(entry)?;

        asm_func.get_reg(idx)        
    }

    // parameter offset label
    pub fn res_parameter(&mut self, entry: usize, idx: usize) ->  Option<(AssemblyLabel)> {

        let asm_func= self.func_list
            .get(entry)?;

        asm_func.get_par(idx)        
    }

    
    // constant offset label
    pub fn res_constant(&mut self, entry: usize, val: u16) ->  Option<(AssemblyLabel)> {

        let cur = self.consume_constant();

        
        let first  = usize_to_alphanumeric(cur / 36)?;
        let second = usize_to_alphanumeric(cur % 36)?;

        let res_label = AssemblyLabel('U',first,second);

        self.func_list
            .get_mut(entry)?
            .push_constant(res_label, val);
        
        Some(res_label)
    }


    
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsmFuncData {
    constant : Vec<(AssemblyLabel,u16)>,
    variable : HashMap<usize,(AssemblyLabel,u16)>,
    register : HashMap<usize,(AssemblyLabel,u16)>,
    parameter : HashMap<usize,(AssemblyLabel,u16)>,
    branch :  HashMap<usize, AssemblyLabel>,
}


impl AsmFuncData {

    pub fn new() -> Self {
        AsmFuncData {
            constant: Vec::new(),
            variable: HashMap::new(),
            register: HashMap::new(),
            parameter: HashMap::new(),
            branch: HashMap::new(),
        }
    }

    pub fn get_var_count(&self) -> usize {
        self.variable.len()
    }

    pub fn get_reg_count(&self) -> usize {
        self.register.len()
    }

    pub fn get_par_count(&self) -> usize {
        self.parameter.len()
    }

    pub fn get_cons_count(&self) -> usize {
        self.constant.len()
    }

    
    pub fn get_cons_list(&self) -> Vec<(AssemblyLabel, u16)> {        
        self.constant.clone()
    }

    pub fn get_var_list(&self) -> HashMap<usize, (AssemblyLabel, u16)> {        
        self.variable.clone()
    }

    pub fn get_reg_list(&self) -> HashMap<usize, (AssemblyLabel, u16)> {        
        self.register.clone()
    }

    pub fn get_par_list(&self) -> HashMap<usize, (AssemblyLabel, u16)> {        
        self.parameter.clone()
    }

    pub fn get_var(&self, idx: usize) -> Option<AssemblyLabel> {
        let data = self.variable
            .get(&idx)?
            .clone();
        Some(data.0)
    }

    pub fn get_reg(&self, idx: usize) -> Option<AssemblyLabel> {
        let data = self.register
            .get(&idx)?
            .clone();
        Some(data.0)
    }

    pub fn get_par(&self, idx: usize) -> Option<AssemblyLabel> {
        let data = self.parameter
            .get(&idx)?
            .clone();
        Some(data.0)
    }

    pub fn get_bra(&self, idx: usize) -> Option<AssemblyLabel> {
        let label = self.branch
            .get(&idx)?
            .clone();
        Some(label)
    }

    pub fn push_constant(&mut self, label: AssemblyLabel, val: u16) {
        self.constant
            .push((label,val))
    }

    pub fn push_variable(&mut self, idx: usize, label: AssemblyLabel, offset: u16) {
        self.variable
            .insert(idx, (label,offset));
    }

    pub fn push_register(&mut self, idx: usize, label: AssemblyLabel, offset: u16) {
        self.register
            .insert(idx, (label,offset));
    }

    pub fn push_parameter(&mut self, idx: usize, label: AssemblyLabel, offset: u16) {
        self.parameter
            .insert(idx, (label,offset));
    }

    pub fn push_branch(&mut self, idx: usize, label: AssemblyLabel) {
        self.branch
            .insert(idx, label);
    }

    pub fn gen_constant(&mut self, idx: usize, val: u16, cur: usize) {

        let first  = usize_to_alphanumeric(cur / 36).unwrap();
        let second = usize_to_alphanumeric(cur % 36).unwrap();
        
        let res_label = AssemblyLabel('U',first,second);

        self
            .push_constant(res_label,val);
    }


    pub fn gen_variable(&mut self, ir_function: &IlocFunction, idx: usize, cur: usize) {

        let first  = usize_to_alphanumeric(cur / 36).unwrap();
        let second = usize_to_alphanumeric(cur % 36).unwrap();
        
        let res_label = AssemblyLabel('V',first,second);

        let offset = ir_function
            .get_regmax() + idx;

        self
            .push_variable(idx,res_label,offset as u16);        
    }

    pub fn gen_register(&mut self, idx: usize, cur: usize) {

        let first  = usize_to_alphanumeric(cur / 36).unwrap();
        let second = usize_to_alphanumeric(cur % 36).unwrap();
        
        let res_label = AssemblyLabel('R',first,second);

        let offset =  idx;

        self
            .push_register(idx,res_label,offset as u16);        
    }

    
    pub fn gen_parameter(&mut self, ir_function: &IlocFunction, idx: usize, cur: usize) {

        let first  = usize_to_alphanumeric(cur / 36).unwrap();
        let second = usize_to_alphanumeric(cur % 36).unwrap();
        
        let res_label = AssemblyLabel('W',first,second);

        let offset = ir_function
            .get_regmax() + ir_function
                .get_localsize() + 2 + idx;

        self
            .push_parameter(idx,res_label,offset as u16);        
    }

    pub fn gen_bra(&mut self, idx: usize, cur: usize) {

        let first  = usize_to_alphanumeric(cur / 36).unwrap();
        let second = usize_to_alphanumeric(cur % 36).unwrap();
        
        let res_label = AssemblyLabel('B',first,second);

        self
            .push_branch(idx,res_label);        
    }

    
    
}


fn usize_to_alphanumeric(number: usize) -> Option<char> {

    match number {
        0 => Some('0'),
        1 => Some('1'),
        2 => Some('2'),
        3 => Some('3'),
        4 => Some('4'),
        5 => Some('5'),
        6 => Some('6'),
        7 => Some('7'),
        8 => Some('8'),
        9 => Some('9'),
        10 => Some('A'),
        11 => Some('B'),
        12 => Some('C'),
        13 => Some('D'),
        14 => Some('E'),
        15 => Some('F'),
        16 => Some('G'),
        17 => Some('H'),
        18 => Some('I'),
        19 => Some('J'),
        20 => Some('K'),
        21 => Some('L'),
        22 => Some('M'),
        23 => Some('N'),
        24 => Some('O'),
        25 => Some('P'),
        26 => Some('Q'),
        27 => Some('R'),
        28 => Some('S'),
        29 => Some('T'),
        30 => Some('U'),
        31 => Some('V'),
        32 => Some('W'),
        33 => Some('X'),
        34 => Some('Y'),
        35 => Some('Z'),
        _ => None,
    }
}

