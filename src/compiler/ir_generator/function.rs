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
    Register,
    IlocLabel,
    GotoLabel,
};

use crate::compiler::ir_generator::IrGenError;
use crate::compiler::ir_generator::statement::stmtcomp;




pub fn ir_function(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
) -> Result<(),IrGenError> {

    let node = ast.get(id).unwrap();

    let cur_func = func_table
        .get_unmut_bypos(entry)
        .unwrap();

    let func_name = cur_func.get_name();
    
    iloc.push_func(&func_name, entry,cur_func.get_params(),cur_func.get_vars.count() - cur_func.get_params());

    let ir_func = iloc
        .get_mut_bypos(entry)
        .unwrap();

    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::FunctionStart(entry)), id);
    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::FunctionPrologue), id);

    let return_bc = ir_func.inc_internal_bc();
    
    let rchild = node.get_rchild().unwrap();   
    stmtcomp::ir_stmtcomp(ast,func_table,rchild, entry,iloc)?;
    
    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::Goto(GotoLabel(return_bc))), id);
    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::FunctionEpilogue), id);
    
    Ok(())
}
