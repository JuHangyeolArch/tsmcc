use std::arch::x86_64::_mm512_mask_fmsub_round_pd;
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

pub fn fc_stmtwhile(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    cur_flow: usize
) -> Result<(usize),CsError> {

    let node = ast.get(id).unwrap();
    let rchild = node.get_rchild().unwrap();

    func_table
        .get_bypos(entry)
        .unwrap()
        .get_mut_fg();

    let entrance_flow = func_table
        .get_bypos(entry)
        .unwrap()
        .get_mut_fg()
        .alloc();

    func_table
        .get_bypos(entry)
        .unwrap()
        .get_mut_fg()
        .get_mut(cur_flow)
        .unwrap()
        .set_child(entrance_flow);


    let exit_flow = stmtcomp::fc_stmtcomp(ast,func_table,rchild,entry,entrance_flow)?;

    func_table
        .get_bypos(entry)
        .unwrap()
        .get_mut_fg()
        .get_mut(exit_flow)
        .unwrap()
        .set_child(cur_flow);
    
    let next_flow = func_table
        .get_bypos(entry)
        .unwrap()
        .get_mut_fg()
        .alloc();

    func_table
        .get_bypos(entry)
        .unwrap()
        .get_mut_fg()
        .get_mut(cur_flow)
        .unwrap()
        .set_child(next_flow);    

    func_table
        .get_bypos(entry)
        .unwrap()
        .get_mut_fg()
        .get_mut(exit_flow)
        .unwrap()
        .set_child(next_flow);
    
    Ok(next_flow)
}
