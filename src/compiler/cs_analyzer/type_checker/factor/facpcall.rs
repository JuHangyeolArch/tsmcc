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
use crate::compiler::cs_analyzer::type_checker::arglist;

pub fn tc_facpcall(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(ValueType,SideKind),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let func_name = id::tc_id(ast,lchild)?;

    let Some(func_info) = func_table
        .get_byname(&func_name) else {
            return Err(CsError::UnexpectedError(id))
        };

    let return_type = func_info.get_type();
    
    let rchild = node.get_rchild().unwrap();

    arglist::tc_arglist(ast,func_table,rchild,entry,func_info, 0)?;

    Ok((return_type,SideKind::Rvalue))
}
