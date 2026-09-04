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
use crate::compiler::ir_generator::factor;

pub mod faclit;
pub mod facvar;
pub mod facpcall;
pub mod facexpr;
pub mod factypecast;


pub fn ir_factor(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    reg_count: usize,    
) -> Result<(SideKind,usize),IrGenError> {

    let node = ast.get(id).unwrap();
    
    match node.get_kind() {
        AstNodeKind::Factor(FactorKind::Variable) => facvar::ir_facvar(ast, func_table,id,entry,iloc,reg_count),
        AstNodeKind::Factor(FactorKind::Literal) => faclit::ir_faclit(ast, func_table,id,entry,iloc,reg_count),
        AstNodeKind::Factor(FactorKind::ParenthAssign) => facexpr::ir_facexpr(ast, func_table,id,entry,iloc,reg_count),
        AstNodeKind::Factor(FactorKind::ProcedureCall) => facpcall::ir_facpcall(ast, func_table,id,entry,iloc,reg_count),
        AstNodeKind::Factor(FactorKind::TypeCast) => factypecast::ir_factypecast(ast, func_table,id,entry,iloc,reg_count),
        _ => Err(IrGenError::UnexpectedError(id))
    }

    

}
