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
use crate::compiler::ir_generator::SideKind;

use crate::compiler::ir_generator::condexpr;
use crate::compiler::ir_generator::loadval;

pub fn ir_assexpr(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    reg_count: usize,
) -> Result<(SideKind,usize),IrGenError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();
    
    let (left_kind, left_reg) = condexpr::ir_condexpr(ast,func_table,lchild,entry,iloc,reg_count)?;
    
    let Some(rchild) = node.get_rchild() else {
        return Ok((left_kind, left_reg));
    };

    let (right_kind,right_reg) = ir_assexpr(ast,func_table,rchild,entry,iloc,left_reg + 1)?;


    let right_eval_reg: usize = if right_kind == SideKind::LValue {
         loadval::ir_loadval(id,entry,iloc,right_reg)
    } else {
        right_reg
    };

    iloc
        .get_mut_bypos(entry)
        .unwrap()    
        .add_line(
        IlocInstruction::Store{
            source: Register(right_eval_reg),
            target: Register(left_reg),
        },
        None,
        id
    );
    
    Ok((SideKind::RValue,left_reg))
}
