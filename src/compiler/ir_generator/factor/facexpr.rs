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
    Register,
    IlocLabel,
};

use crate::compiler::ir_generator::SideKind;
use crate::compiler::ir_generator::IrGenError;
use crate::compiler::ir_generator::loadval;
use crate::compiler::ir_generator::assignexpr;


pub fn ir_facexpr(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    reg_count: usize,    
) -> Result<(SideKind,usize),IrGenError> {

    let node = ast.get(id).unwrap();

    let AstNodeKind::Identifier(name) = node.get_kind() else {
        return Err(IrGenError::UnexpectedError);
    };

    let lchild = node.get_lchild().unwrap();

    let (left_kind, left_reg) = assignexpr::ir_assexpr(ast,func_table,lchild,entry,iloc,reg_count)?;    
    
    Ok((left_kind,left_reg))
}
