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
};

use crate::compiler::ir_generator::SideKind;
use crate::compiler::ir_generator::IrGenError;
use crate::compiler::ir_generator::loadval;
use crate::compiler::ir_generator::id;


pub fn ir_facvar(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    reg_count: usize,    
) -> Result<(SideKind,usize),IrGenError> {

    let node = ast.get(id).unwrap();

    let lchild = node.get_lchild().unwrap();

    let name = id::ir_id(ast,id)?;

    let ir_func = iloc
        .get_mut_bypos(entry)
        .unwrap();

    let cur_func = func_table
        .get_unmut_bypos(entry)
        .unwrap();    
    
    let Some(mut var_idx) = cur_func
        .get_unmut_vars()
        .get_index(&name) else {
            return Err(IrGenError::UnexpectedError);
        };
    ir_func.update_regmax(reg_count);
    
    if cur_func.get_params() <= var_idx {
        var_idx = var_idx - cur_func.get_params();
        ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::Variable(var_idx,Register(reg_count))), id);
    }

    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::Paremeter(var_idx,Register(reg_count))), id);
    
    Ok((SideKind::LValue,reg_count))
}
