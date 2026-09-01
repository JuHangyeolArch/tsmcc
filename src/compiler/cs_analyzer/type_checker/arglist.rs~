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


use crate::compiler::cs_analyzer::CsError;
use crate::compiler::cs_analyzer::type_checker::SideKind;
use crate::compiler::cs_analyzer::type_checker::assignexpr;


pub fn tc_arglist(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    callee: &FunctionInfo,
    index: usize,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();

    let lchild: NodeID;

    if let Some(lchild) = node.get_lchild() {
        if index >= callee.get_params() {
            return Err(CsError::InvalidArguement(lchild));
        }
        let arg_type = assignexpr::tc_assexpr(ast,func_table,lchild,entry)?;

        if !arg_type.is_equal(callee
            .get_unmut_vars()
            .get_bypos(index)
            .unwrap()
            .get_type()) {
                return Err(CsError::TypeMismatched(id));
            }
        
        let rchild = node.get_rchild().unwrap();
        tc_arglist(ast,func_table,rchild,entry,callee,index + 1)        
        
    } else {
        if index < callee.get_params() {
            return Err(CsError::InvalidArguement(id));
        }
        return Ok(());
    };            
    
    Ok(())
}
