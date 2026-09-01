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

use crate::compiler::ir_generator::SideKind;

use crate::compiler::cs_analyzer::type_checker::condexpr;


pub fn tc_assexpr(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(ValueType,SideKind),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let (ltype, lside) = condexpr::tc_condexpr(ast,func_table,lchild,entry)?;
    
    let Some(rchild) = node.get_rchild() else {
        return Ok((ltype,lside));
    };

    let (rtype,rside) = tc_assexpr(ast,func_table,rchild,entry)?;

    if !ltype.is_equal(rtype) {
        return Err(CsError::TypeMismatched(id))
    };

    if lside == SideKind::Lvalue {
        return Err(CsError::AssginToRValue(lchild))
    };
    
    Ok((ltype,SideKind::Rvalue))
}
