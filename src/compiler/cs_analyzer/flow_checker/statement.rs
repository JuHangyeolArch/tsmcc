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

pub mod stmtcomp;
pub mod stmtif;
pub mod stmtwhile;
pub mod stmtret;

pub fn fc_statement(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    cur_flow: usize,
) -> Result<(usize),CsError> {

    let node = ast.get(id).unwrap();

    match node.get_kind() {
        AstNodeKind::Statement(StatementKind::Expression) => Ok(cur_flow),
        AstNodeKind::Statement(StatementKind::If) => stmtif::fc_stmtif(ast,func_table,id,entry,cur_flow),
        AstNodeKind::Statement(StatementKind::While) => stmtwhile::fc_stmtwhile(ast,func_table,id,entry,cur_flow),
        AstNodeKind::Statement(StatementKind::Compound) => stmtcomp::fc_stmtcomp(ast,func_table,id,entry,cur_flow),
        AstNodeKind::Statement(StatementKind::Return) => stmtret::fc_stmtret(ast,func_table,id,entry,cur_flow),
        AstNodeKind::Statement(StatementKind::Empty) => Ok((cur_flow)),
        _ => Err(CsError::UnexpectedError)
    }
}
