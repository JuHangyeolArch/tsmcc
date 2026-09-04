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

pub mod facvar;
pub mod facpcall;
pub mod facexpr;
pub mod factypecast;
use crate::compiler::cs_analyzer::scope_checker::ScopeStack;

pub fn sc_factor(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    ss: &ScopeStack
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();

    match node.get_kind() {
        AstNodeKind::Factor(FactorKind::Variable) => facvar::sc_facvar(ast,id,ss),
        AstNodeKind::Factor(FactorKind::Literal) => Ok(()),
        AstNodeKind::Factor(FactorKind::ParenthAssign) => facexpr::sc_facexpr(ast, func_table,id,entry,ss),
        AstNodeKind::Factor(FactorKind::ProcedureCall) => facpcall::sc_facpcall(ast, func_table,id,entry,ss),
        AstNodeKind::Factor(FactorKind::TypeCast) => factypecast::sc_factypecast(ast, func_table,id,entry,ss),
        _ => Err(CsError::UnexpectedError(id))
    }

    

}
