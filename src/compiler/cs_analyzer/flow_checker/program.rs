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
use crate::compiler::cs_analyzer::flow_checker::funclist;


pub fn fc_program(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();
    funclist::fc_funclist(ast,func_table,lchild,func_table.count_built_in(),)
    
}
