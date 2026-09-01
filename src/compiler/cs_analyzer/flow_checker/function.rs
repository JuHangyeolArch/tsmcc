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
use crate::compiler::cs_analyzer::flow_checker::statement::stmtcomp;




pub fn fc_function(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();
    
    let rchild = node.get_rchild().unwrap();


    let cur_flow = func_table
        .get_bypos(entry)
        .unwrap()
        .get_mut_fg()
        .alloc();

    
    stmtcomp::fc_stmtcomp(ast,func_table,rchild, entry, cur_flow)?;

    if !func_table
        .get_bypos(entry)
        .unwrap()
        .get_mut_fg()
        .must_return() {
        return Err(CsError::PossiblyNoReturn(entry));
    };
    
    Ok(())
}
