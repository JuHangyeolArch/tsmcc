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

use crate::compiler::ir_generator::assignexpr;

pub fn ir_stmtexpr(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
) -> Result<(),IrGenError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let localsize = iloc
        .get_mut_bypos(entry)
        .unwrap()
        .get_localsize();
    
    assignexpr::ir_assexpr(ast, func_table,lchild,entry,iloc,1)?;
    
    Ok(())

}
