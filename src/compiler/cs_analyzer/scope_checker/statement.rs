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
use crate::compiler::cs_analyzer::scope_checker::ScopeStack;

pub mod stmtcomp;
pub mod stmtexpr;
pub mod stmtif;

pub fn sc_statement(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    ss: &mut ScopeStack
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();

    match node.get_kind() {
        AstNodeKind::Statement(StatementKind::Expression) => stmtexpr::sc_stmtexpr(ast,func_table,id,entry,ss),
        AstNodeKind::Statement(StatementKind::If) => stmtif::sc_stmtif(ast,func_table,id,entry,ss),
        AstNodeKind::Statement(StatementKind::While) => stmtif::sc_stmtif(ast,func_table,id,entry,ss),
        AstNodeKind::Statement(StatementKind::Compound) => {
            ss.block_in();
            stmtcomp::sc_stmtcomp(ast,func_table,id,entry,ss)?;
            ss.pop();
            Ok(())
        },
        AstNodeKind::Statement(StatementKind::Return) => stmtexpr::sc_stmtexpr(ast,func_table,id,entry,ss),
        AstNodeKind::Statement(StatementKind::Empty) => Ok(()),
        _ => Err(CsError::UnexpectedError)
    }
}
