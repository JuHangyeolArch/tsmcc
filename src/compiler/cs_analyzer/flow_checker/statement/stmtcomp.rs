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
use crate::compiler::cs_analyzer::flow_checker::stmtlist;

pub fn fc_stmtcomp(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    cur_flow: usize,
) -> Result<(usize),CsError> {

    let node = ast.get(id).unwrap();
    
    let rchild = node.get_rchild().unwrap();

    let next_flow = stmtlist::fc_stmtlist(ast,func_table,rchild,entry,cur_flow)?;

    Ok((next_flow))
}
