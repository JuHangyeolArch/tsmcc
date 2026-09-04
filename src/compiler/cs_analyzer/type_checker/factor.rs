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

pub mod facvar;
pub mod facpcall;
pub mod facexpr;
pub mod factypecast;


pub fn tc_factor(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(ValueType,SideKind),CsError> {

    let node = ast.get(id).unwrap();

    println!("{:#?}",node.get_kind());

    match node.get_kind() {
        AstNodeKind::Factor(FactorKind::Variable) => facvar::tc_facvar(ast, func_table,id,entry),
        AstNodeKind::Factor(FactorKind::Literal) => Ok((ValueType::new(BasicValueKind::Int,0), SideKind::Rvalue)),
        AstNodeKind::Factor(FactorKind::ParenthAssign) => facexpr::tc_facexpr(ast, func_table,id,entry),
        AstNodeKind::Factor(FactorKind::ProcedureCall) => facpcall::tc_facpcall(ast, func_table,id,entry),
        AstNodeKind::Factor(FactorKind::TypeCast) => factypecast::tc_factypecast(ast, func_table,id,entry),
        _ => Err(CsError::UnexpectedError)
    }


}
