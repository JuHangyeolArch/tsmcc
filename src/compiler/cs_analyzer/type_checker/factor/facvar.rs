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
use crate::compiler::cs_analyzer::type_checker::id;

pub fn tc_facvar(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(ValueType,SideKind),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let var_name = id::tc_id(ast,lchild)?;

    let Some(var) = func_table
        .get_unmut_bypos(entry)
        .unwrap()
        .get_unmut_vars()
        .get_byname(&var_name) else {
            return Err(CsError::UnexpectedError);
        };
    
    Ok((var.get_type(),SideKind::Lvalue))
    
}
