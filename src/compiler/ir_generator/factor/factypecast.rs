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
use crate::compiler::ir_generator::factor;

pub fn ir_factypecast(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    reg_count: usize,    
) -> Result<(SideKind,usize),IrGenError> {

    let node = ast.get(id).unwrap();
    
    let rchild = node.get_rchild().unwrap();

    let (fac_kind, fac_reg)= factor::ir_factor(ast,func_table,rchild,entry,iloc,reg_count)?;            

    let fac_eval_reg: usize = if fac_kind == SideKind::LValue {
         loadval::ir_loadval(id,entry,iloc,fac_reg)
    } else {
        fac_reg
    };
    
    Ok((Rvalue,fac_eval_reg))
}
