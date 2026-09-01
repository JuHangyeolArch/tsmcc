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

use crate::compiler::iloc::{
    IlocProgram,
    IlocLine,
    IlocInstruction,
};


use crate::compiler::ir_generator::IrGenError;
use crate::compiler::ir_generator::function;



pub fn ir_funclist(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    cur_func: usize,
    iloc: &mut IlocProgram,
) -> Result<(),IrGenError> {

    let node = ast.get(id).unwrap();
    let Some(lchild) = node.get_lchild() else {
        return Ok(());
    };
    
    function::ir_function(ast,func_table,lchild,cur_func,iloc)?;

    let rchild = node.get_rchild().unwrap();

    ir_funclist(ast,func_table,rchild,cur_func+1,iloc)

    
}
