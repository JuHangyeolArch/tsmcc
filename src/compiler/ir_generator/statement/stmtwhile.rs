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
    GotoLabel,
};

use crate::compiler::ir_generator::SideKind;
use crate::compiler::ir_generator::IrGenError;


use crate::compiler::ir_generator::assignexpr;
use crate::compiler::ir_generator::loadval;
use crate::compiler::ir_generator::statement::stmtcomp;



pub fn ir_stmtwhile(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
) -> Result<(),IrGenError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let (expr_kind, expr_reg)= assignexpr::ir_assexpr(ast,func_table,lchild,entry,iloc,1)?;

    let expr_eval_reg: usize = if expr_kind == SideKind::LValue {
         loadval::ir_loadval(id,entry,iloc,expr_reg)
    } else {
        expr_reg
    };

    let block_in = iloc
        .get_mut_bypos(entry)
        .unwrap()
        .inc_internal_bc();

    let block_out = iloc
        .get_mut_bypos(entry)
        .unwrap()
        .inc_internal_bc();

    iloc
        .get_mut_bypos(entry)
        .unwrap()
        .update_regmax(expr_eval_reg);    
    
    iloc
        .get_mut_bypos(entry)
        .unwrap()
        .add_line(
        IlocInstruction::Cbr{
            source: Register(expr_eval_reg),
            target_l1: GotoLabel(block_in),
            target_l2: GotoLabel(block_out),
        },
        None,
        id
    );

    iloc
        .get_mut_bypos(entry)
        .unwrap()
        .add_line(IlocInstruction::Empty,Some(IlocLabel::Goto(GotoLabel(block_in))), id);
    
    let rchild = node.get_rchild().unwrap();

    stmtcomp::ir_stmtcomp(ast,func_table,rchild,entry,iloc)?;

    iloc
        .get_mut_bypos(entry)
        .unwrap()
        .add_line(
        IlocInstruction::JumpI{
            targetl: GotoLabel(block_in),
        },
        None,
        id
    );

    iloc
        .get_mut_bypos(entry)
        .unwrap()
        .add_line(IlocInstruction::Empty,Some(IlocLabel::Goto(GotoLabel(block_out))), id);
    
    Ok(())
}
