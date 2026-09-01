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
    Register
};

use crate::compiler::ir_generator::IrGenError;
use crate::compiler::ir_generator::statement;


pub fn fc_stmtlist(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();

    let Some(lchild) = node.get_lchild() else {
        return Ok((cur_flow));
    };
    
    statement::ir_statement(ast,func_table,lchild,entry,iloc)?;            


    let rchild = node.get_rchild().unwrap();
    ir_stmtlist(ast,func_table,rchild,entry,iloc)

    
}

