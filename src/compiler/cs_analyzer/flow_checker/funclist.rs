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
use crate::compiler::cs_analyzer::flow_checker::function;



pub fn fc_funclist(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    cur_func: usize,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();
    let Some(lchild) = node.get_lchild() else {
        return Ok(());
    };
    
    function::fc_function(ast,func_table,lchild,cur_func)?;

    let rchild = node.get_rchild().unwrap();

    fc_funclist(ast,func_table,rchild,cur_func+1)

    
}
