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


use crate::compiler::cs_analyzer::scope_checker::ScopeStack;


pub fn sc_declarartion(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    ss: &mut ScopeStack
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let var = declarator::sc_declarator(ast,lchild)?;
    
    let varlist = func_table
        .get_bypos(entry)
        .unwrap()
        .get_vars();

    if let Some(_) = varlist.get_byname(&var.get_name()) {
        return Err(CsError::DucplicateDeclaration(id));
    };

    varlist.add(&var.get_name(),&var.get_type(),Some(id));

    ss.push(var.get_name());
    
    Ok(())
}
