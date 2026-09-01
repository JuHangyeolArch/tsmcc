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

use crate::compiler::cs_analyzer::type_checker::addexpr;

pub fn tc_condexpr(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(ValueType, SideKind),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let (ltype,lside) = addexpr::tc_addexpr(ast,func_table,lchild,entry)?;
    
    let Some(rchild) = node.get_rchild() else {
        return Ok((ltype,lside));
    };

    let (rtype, _) = tc_condexpr(ast,func_table,rchild,entry)?;

    if !ltype.is_equal(rtype) {
        return Err(CsError::TypeMismatched(id))
    };
    
    Ok((ValueType::new(BasicValueKind::Int,0),SideKind::Rvalue))

}
