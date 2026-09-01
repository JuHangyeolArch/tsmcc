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
use crate::compiler::cs_analyzer::type_checker::assignexpr;
use crate::compiler::cs_analyzer::type_checker::SideKind;

pub fn tc_facexpr(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(ValueType,SideKind),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let (ltype, lside) = assignexpr::tc_assexpr(ast,func_table,lchild,entry)?;
    
    Ok((ltype, lside))

}
