use std::ops::Range;
use std::fmt;
use std::error::Error;
use crate::compiler::ProducedBy;

use crate::compiler::SpanInfo;
use crate::compiler::CompileError;
use crate::compiler::ast::{
    Ast,
    AstNode,
    NodeID,
    AstNodeKind,
    AdditiveOpKind,
    MultiplicativeOpKind,
    StatementKind,
    FactorKind,
    BasicValueKind,
};
use crate::compiler::symtable::{
    FuncTable,
    FunctionInfo,
    ValueType,
    VarList,
    Variable,  
};

use crate::compiler::iloc::{
    IlocProgram,
    IlocLine,
    IlocInstruction,
    Register
};


use crate::compiler::ir_generator::IrGenError;
use crate::compiler::ir_generator::SideKind;

pub fn ir_negative(
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    source_reg: usize,
) -> usize {

    let ir_func = iloc
        .get_mut_bypos(entry)
        .unwrap();

    let zero_reg = source_reg + 1;

    ir_func.update_regmax(zero_reg);
    
    ir_func.add_line(
        IlocInstruction::LoadI{
            constant: 0,
            target: Register(zero_reg),
        },
        None,
        id
    );

    let result_reg = zero_reg + 1;
    ir_func.update_regmax(result_reg);
    ir_func.add_line(
        IlocInstruction::Sub{
            source1: Register(zero_reg),
            source2: Register(source_reg),
            target: Register(result_reg),
        },
        None,
        id
    );

    
    result_reg
}
