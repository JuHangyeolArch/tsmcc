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
    UnaryOpKind,
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
use crate::compiler::cs_analyzer::scope_checker::factor;
use crate::compiler::cs_analyzer::scope_checker::ScopeStack;


pub fn sc_unaryexpr(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    ss: &ScopeStack
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    println!("a {:#?}",id);
    let AstNodeKind::UnaryExpr(opkind) = ast
        .get(id)
        .unwrap()
        .get_kind() else {
            return Err(CsError::UnexpectedError(id));
        };
    
    match opkind {
        UnaryOpKind::Nil => factor::sc_factor(ast, func_table, lchild, entry,ss), 
        _ => sc_unaryexpr(ast, func_table, lchild, entry,ss),
    }

}
