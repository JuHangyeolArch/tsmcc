use crate::compiler::assembly::{
    AssemblyInstruction,
    AssemblyLabel,
    AssemblyLine,
    AssemblyProgram,
    U12,
    AssemblyValue,
};

pub fn init_asm(asm_prog: &mut AssemblyProgram) {
    asm_zero(asm_prog);
    asm_mul(asm_prog);    
    asm_div(asm_prog);    
    asm_srv(asm_prog);    
    asm_var(asm_prog);
    
    asm_prog.add_line(None, AssemblyInstruction::ORG(U12::new(0x100)), None);
}

pub fn asm_zero(asm_prog: &mut AssemblyProgram) {
    
    asm_prog.add_line(None, AssemblyInstruction::ORG(U12::new(0x0)), None);

    // 2
    asm_prog.add_line(None, AssemblyInstruction::Data(0), None);
    asm_prog.add_line(Some(AssemblyLabel('Z','R','O')), AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('S','R','V')),false), None)    
}


// 91(16 * 9) = 2(16), 26(32), 35(48), 23(32), 5(16)

pub fn asm_mul(asm_prog: &mut AssemblyProgram) {
    // 26

    asm_prog.add_line(None, AssemblyInstruction::ORG(U12::new(0x010)), None);

    // multiply

    
    //preprocessing
    asm_prog.add_line(Some(AssemblyLabel('M','U','L')), AssemblyInstruction::Data(0x000), None);
    asm_prog.add_line(None, AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('C','T','R')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('M','C','T')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::CLA, None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('M','U','P')) , false), None);
    
    //loop
    asm_prog.add_line(Some(AssemblyLabel('M','L','P')), AssemblyInstruction::CLE, None);
    asm_prog.add_line(None, AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('M','U','Y')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::CIR, None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('M','U','Y')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::SZE, None);
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('M','U','1')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('M','U','0')) , false), None);

    //one
    asm_prog.add_line(Some(AssemblyLabel('M','U','1')), AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('M','U','X')),false), None);
    asm_prog.add_line(None, AssemblyInstruction::ADD(AssemblyValue::Label(AssemblyLabel('M','U','P')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('M','U','P')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::CLE, None);

    //zero
    asm_prog.add_line(Some(AssemblyLabel('M','U','0')), AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('M','U','X')),false), None);
    asm_prog.add_line(None, AssemblyInstruction::CIL, None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('M','U','X')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::ISZ(AssemblyValue::Label(AssemblyLabel('M','C','T')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('M','L','P')) , false), None);
    
    //return
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('M','U','L')) , true), None);

    
    // multiply input 1,2
    asm_prog.add_line(Some(AssemblyLabel('M','U','X')), AssemblyInstruction::Data(0x0), None);
    asm_prog.add_line(Some(AssemblyLabel('M','U','Y')), AssemblyInstruction::Data(0x0), None);

    // multiply data
    asm_prog.add_line(Some(AssemblyLabel('M','C','T')), AssemblyInstruction::Data(0x000), None);
    
    
    //multipy output 
    asm_prog.add_line(Some(AssemblyLabel('M','U','P')), AssemblyInstruction::Data(0x0), None);
    
}

pub fn asm_div(asm_prog: &mut AssemblyProgram) {

    // 35

    
    asm_prog.add_line(None, AssemblyInstruction::ORG(U12::new(0x030)), None);

    //divide
    asm_prog.add_line(Some(AssemblyLabel('D','I','V')), AssemblyInstruction::Data(0x000), None);

    //preprocessing
    asm_prog.add_line(None, AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('C','T','R')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('D','C','T')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::CLA, None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('D','R','M')) , false), None);

    //store remainder
    asm_prog.add_line(None, AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('D','I','Y')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('D','I','P')) , false), None);
    
    //store negative value
    asm_prog.add_line(None, AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('D','I','X')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::CMA, None);
    asm_prog.add_line(None, AssemblyInstruction::INC, None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('D','X','N')) , false), None);

    //loop
    asm_prog.add_line(Some(AssemblyLabel('D','L','P')), AssemblyInstruction::CLE, None);

    asm_prog.add_line(None, AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('D','I','P')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::CIL, None);
    asm_prog.add_line(None, AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('D','R','M')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::CIL, None);    
    asm_prog.add_line(None, AssemblyInstruction::ADD(AssemblyValue::Label(AssemblyLabel('D','X','N')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('D','R','M')) , false), None);

    asm_prog.add_line(None, AssemblyInstruction::SNA, None);
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('D','P','P')) , false), None);
    
    //restore remainder
    asm_prog.add_line(None, AssemblyInstruction::ADD(AssemblyValue::Label(AssemblyLabel('D','I','X')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('D','X','N')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('D','C','N')) , false), None);

    //increment quotient
    asm_prog.add_line(Some(AssemblyLabel('D','P','P')), AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('D','I','P')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::INC, None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('D','I','P')) , false), None);

    //count
    asm_prog.add_line(Some(AssemblyLabel('D','C','N')), AssemblyInstruction::ISZ(AssemblyValue::Label(AssemblyLabel('D','C','T')), false), None);    
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('D','L','P')) , false), None);
    
    //return
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('D','I','V')) , true), None);

    
    // divide input 1,2
    asm_prog.add_line(Some(AssemblyLabel('D','I','X')), AssemblyInstruction::Data(0x0), None);
    asm_prog.add_line(Some(AssemblyLabel('D','I','Y')), AssemblyInstruction::Data(0x0), None);

    // divide data
    asm_prog.add_line(Some(AssemblyLabel('D','C','T')), AssemblyInstruction::Data(0xFF8), None);
    asm_prog.add_line(Some(AssemblyLabel('D','X','N')), AssemblyInstruction::Data(0x0), None);
    asm_prog.add_line(Some(AssemblyLabel('D','R','M')), AssemblyInstruction::Data(0x0), None);
    
    // divide output
    asm_prog.add_line(Some(AssemblyLabel('D','I','P')), AssemblyInstruction::Data(0x0), None);

    
}


pub fn asm_srv(asm_prog: &mut AssemblyProgram) {

    //23
    
    // interrupt service routine
    asm_prog.add_line(None, AssemblyInstruction::ORG(U12::new(0x060)), None);

    asm_prog.add_line(Some(AssemblyLabel('S','R','V')), AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('S','A','C')), false), None);
    asm_prog.add_line(None, AssemblyInstruction::CIR, None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('S','E',' ')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::SKI, None);
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('N','X','T')), false), None);
    asm_prog.add_line(None, AssemblyInstruction::INP, None);
    asm_prog.add_line(None, AssemblyInstruction::OUT, None);
    asm_prog.add_line(None, AssemblyInstruction::STA(AssemblyValue::Label(AssemblyLabel('P','T','1')), true), None);
    asm_prog.add_line(None, AssemblyInstruction::ISZ(AssemblyValue::Label(AssemblyLabel('P','T','1')), false), None);
    asm_prog.add_line(Some(AssemblyLabel('N','X','T')), AssemblyInstruction::SKO, None);
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('E','X','T')), false), None);
    asm_prog.add_line(None, AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('P','T','2')) , true), None);
    asm_prog.add_line(None, AssemblyInstruction::OUT, None);
    asm_prog.add_line(None, AssemblyInstruction::ISZ(AssemblyValue::Label(AssemblyLabel('P','T','2')) , true), None);
    asm_prog.add_line(Some(AssemblyLabel('E','X','T')), AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('S','E',' ')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::CIL, None);
    asm_prog.add_line(None, AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('S','A','C')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::ION, None);
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('Z','R','O')), true), None);


    // Data
    asm_prog.add_line(Some(AssemblyLabel('S','A','C')), AssemblyInstruction::Data(0x0), None);
    asm_prog.add_line(Some(AssemblyLabel('S','E',' ')), AssemblyInstruction::Data(0x0), None);
    asm_prog.add_line(Some(AssemblyLabel('P','T','1')), AssemblyInstruction::Data(0x0), None);
    asm_prog.add_line(Some(AssemblyLabel('P','T','2')), AssemblyInstruction::Data(0x0), None);


    
}

pub fn asm_cond(asm_prog: &mut AssemblyProgram) {

    //conditionnal
    asm_prog.add_line(None, AssemblyInstruction::ORG(U12::new(0x080)), None);
    asm_prog.add_line(Some(AssemblyLabel('C','O','N')), AssemblyInstruction::Data(0x000), None);    

    asm_prog.add_line(None, AssemblyInstruction::SZA, None);
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('N','Z','R')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('O','N','E')) , false), None);
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('C','O','D')) , false), None);

    asm_prog.add_line(Some(AssemblyLabel('N','Z','O')), AssemblyInstruction::CLA, None);
    asm_prog.add_line(Some(AssemblyLabel('C','O','D')), AssemblyInstruction::LDA(AssemblyValue::Label(AssemblyLabel('C','O','P')) , false), None);    
    asm_prog.add_line(None, AssemblyInstruction::BUN(AssemblyValue::Label(AssemblyLabel('C','O','N')) , true), None);
    
    // Data
    asm_prog.add_line(Some(AssemblyLabel('O','N','E')), AssemblyInstruction::Data(0x0), None);
    asm_prog.add_line(Some(AssemblyLabel('C','O','X')), AssemblyInstruction::Data(0x0), None);
    asm_prog.add_line(Some(AssemblyLabel('C','O','P')), AssemblyInstruction::Data(0x0), None);

    
}

pub fn asm_var(asm_prog: &mut AssemblyProgram) {

    //5
    
    asm_prog.add_line(None, AssemblyInstruction::ORG(U12::new(0x090)), None);        

    //retrun value
    asm_prog.add_line(Some(AssemblyLabel('R','T','V')), AssemblyInstruction::Data(0x0), None);

    // temporary variable
    asm_prog.add_line(Some(AssemblyLabel('T','P','1')), AssemblyInstruction::Data(0x0), None);
    asm_prog.add_line(Some(AssemblyLabel('T','P','2')), AssemblyInstruction::Data(0x0), None);
    
    //counter
    asm_prog.add_line(Some(AssemblyLabel('C','T','R')), AssemblyInstruction::Data(0xFF8), None);    
    
    // stack pointer
    asm_prog.add_line(Some(AssemblyLabel('S','P',' ')), AssemblyInstruction::Data(0x0FFF), None); 

    // temporary SP for parameter in 
    asm_prog.add_line(Some(AssemblyLabel('S','P','T')), AssemblyInstruction::Data(0x0), None); 

    // for SP + offset
    asm_prog.add_line(Some(AssemblyLabel('S','P','O')), AssemblyInstruction::Data(0x0), None); 


}

