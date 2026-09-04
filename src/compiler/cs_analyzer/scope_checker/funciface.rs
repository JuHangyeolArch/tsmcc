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
use crate::compiler::cs_analyzer::scope_checker::declarator;
use crate::compiler::cs_analyzer::scope_checker::paramlist;

use crate::compiler::cs_analyzer::scope_checker::ScopeStack;

pub fn sc_funciface(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    ss: &mut ScopeStack
) -> Result<usize,CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let dec = declarator::sc_declarator(ast,lchild)?;

    if let Some(_) = func_table.get_byname(&dec.get_name()) {
        return Err(CsError::DucplicateDeclaration(id));
    };
    
    println!("{:#?}", dec.get_type());
    
    func_table.add(&dec.get_name(),&dec.get_type(),Some(lchild));
    let entry = func_table.count() - 1;

    let rchild = node.get_rchild().unwrap();
    
    paramlist::sc_paramlist(ast, func_table,rchild, entry,ss)?;

    Ok(entry)
}
