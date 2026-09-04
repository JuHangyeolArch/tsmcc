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

use crate::compiler::ir_generator::SideKind;
use crate::compiler::ir_generator::IrGenError;
use crate::compiler::ir_generator::loadval;


pub fn ir_faclit(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    reg_count: usize,    
) -> Result<(SideKind,usize),IrGenError> {

    let node = ast.get(id).unwrap();

    let lchild = node.get_lchild().unwrap();    
    
    let AstNodeKind::Literal(val) = ast.get(lchild).unwrap().get_kind() else {
        return Err(IrGenError::UnexpectedError(lchild));
    };

    let ir_func = iloc
        .get_mut_bypos(entry)
        .unwrap();

    ir_func.update_regmax(reg_count);

    ir_func.add_line(
        IlocInstruction::LoadI{
            constant: val,
            target: Register(reg_count),
        },
        None,
        id
    );

    

    
    Ok((SideKind::RValue,reg_count))
}
