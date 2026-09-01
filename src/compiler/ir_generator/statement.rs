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

use crate::compiler::ir_generator::IrGenError;


pub mod stmtcomp;
pub mod stmtexpr;
pub mod stmtif;
pub mod stmtwhile;
pub mod stmtret;

pub fn ir_statement(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
) -> Result<(),IrGenError> {

    let node = ast.get(id).unwrap();

    match node.get_kind() {
        AstNodeKind::Statement(StatementKind::Expression) => stmtexpr::ir_stmtexpr(ast,func_table,id,entry,iloc),
        AstNodeKind::Statement(StatementKind::If) => stmtif::ir_stmtif(ast,func_table,id,entry,iloc),
        AstNodeKind::Statement(StatementKind::While) => stmtwhile::ir_stmtwhile(ast,func_table,id,entry,iloc),
        AstNodeKind::Statement(StatementKind::Compound) => stmtcomp::ir_stmtcomp(ast,func_table,id,entry,iloc),
        AstNodeKind::Statement(StatementKind::Return) => stmtret::ir_stmtret(ast,func_table,id,entry,iloc),
        AstNodeKind::Statement(StatementKind::Empty) => Ok(()),
        _ => Err(CsError::UnexpectedError)
    }
}
