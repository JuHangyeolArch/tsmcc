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
use crate::compiler::ir_generator::term;
use crate::compiler::ir_generator::loadval;

pub fn ir_addexpr(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    reg_count: usize,    
) -> Result<(SideKind,usize),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let (left_kind, left_reg) = term::ir_term(ast,func_table,lchild,entry,iloc,reg_count)?;
    
    let Some(rchild) = node.get_rchild() else {
        return Ok((left_kind,left_reg));
    };

    let AstNodeKind::AdditiveExpr(opkind) = node.get_kind() else {
        return Err(IrGenError::UnexpectedError);
    };

    
    let left_eval_reg= if left_kind == SideKind::LValue {
         loadval::ir_loadval(id,entry,iloc,left_reg)
    } else {
        left_reg
    };
    
    let (right_kind,right_reg) = ir_addexpr(ast,func_table,rchild,entry,iloc,left_eval_reg+1)?;

    let right_eval_reg: usize = if right_kind == SideKind::LValue {
         loadval::ir_loadval(id,entry,iloc,right_reg)
    } else {
        right_reg
    };
    
    let result_reg = if left_eval_reg + 1 < right_eval_reg {
        left_eval_reg + 1
    } else {
        right_eval_reg + 1
    };

    let ir_func = iloc
        .get_mut_bypos(entry)
        .unwrap();

    ir_func.update_regmax(result_reg);

    match opkind {
        AdditiveOpKind::Add => {
            ir_func.add_line(
                IlocInstruction::Add{
                    source1: Register(left_eval_reg),
                    source2: Register(right_eval_reg),
                    target: Register(result_reg),
                },
                None,
                id
            );
        },
        AdditiveOpKind::Substract => {
            ir_func.add_line(
                IlocInstruction::Sub{
                    source1: Register(left_eval_reg),
                    source2: Register(right_eval_reg),
                    target: Register(result_reg),
                },
                None,
                id
            );
        },
        _ => {return Err(IrGenError::UnexpectedError);}
    }
    
    Ok((SideKind::RValue,result_reg))
   
}
