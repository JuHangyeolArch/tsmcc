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

use crate::compiler::ir_generator::SideKind;
use crate::compiler::ir_generator::IrGenError;
use crate::compiler::ir_generator::loadval;


pub fn ir_faclit(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
) -> Result<(SideKind,usize),IrGenError> {

    let node = ast.get(id).unwrap();

    let ir_func = iloc
        .get_mut_bypos(entry)
        .unwrap();

    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::FunctionPrologue), id);

    let return_bc = ir_func.inc_internal_bc();
    
    let rchild = node.get_rchild().unwrap();   
    stmtcomp::ir_stmtcomp(ast,func_table,rchild, entry,iloc)?;
    
    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::Goto(GotoLabel(return_bc))), id);
    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::FunctionEpilogue), id);
    
    Ok(())
}
