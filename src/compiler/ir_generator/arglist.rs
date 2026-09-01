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


use crate::compiler::ir_generator::assignexpr;


pub fn ir_arglist(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    reg_count: usize,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();

    let lchild: NodeID;

    if let Some(lchild) = node.get_lchild() {
        if index >= callee.get_params() {
            return Err(CsError::InvalidArguement(lchild));
        }
        let (arg_kind,arg_reg) = assignexpr::ir_assexpr(ast,func_table,lchild,entry,iloc,reg_count)?;

        let arg_eval_reg: usize = if arg_kind == SideKind::LValue {
            loadval::ir_loadval(id,entry,iloc,arg_reg)
        } else {
            arg_reg
        };
        let rchild = node.get_rchild().unwrap();

        ir_arglist(ast,func_table,rchild,entry,iloc,arg_eval_reg + 1)?;
        
        ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::ParamIn(Register(arg_eval_reg))), id);
        
    } else {
        return Ok(());
    };            
    
    Ok(())
}
