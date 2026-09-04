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
pub mod stmtexpr;
pub mod stmtif;
pub mod stmtret;

pub fn tc_statement(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();

    match node.get_kind() {
        AstNodeKind::Statement(StatementKind::Expression) => stmtexpr::tc_stmtexpr(ast,func_table,id,entry),
        AstNodeKind::Statement(StatementKind::If) => stmtif::tc_stmtif(ast,func_table,id,entry),
        AstNodeKind::Statement(StatementKind::While) => stmtif::tc_stmtif(ast,func_table,id,entry),
        AstNodeKind::Statement(StatementKind::Compound) => stmtcomp::tc_stmtcomp(ast,func_table,id,entry),
        AstNodeKind::Statement(StatementKind::Return) => stmtret::tc_stmtret(ast,func_table,id,entry),
        AstNodeKind::Statement(StatementKind::Empty) => Ok(()),
        _ => Err(CsError::UnexpectedError(id))
    }
}
