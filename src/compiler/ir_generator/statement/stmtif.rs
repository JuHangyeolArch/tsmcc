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



pub fn ir_stmtif(
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

    let ir_func = iloc
        .get_mut_bypos(entry)
        .unwrap();

    let block_in = ir_func.inc_internal_bc;
    let block_out = ir_func.inc_internal_bc;

    
    let zero_reg = expr_eval_reg + 1;


    ir_func.add_line(
        IlocInstruction::LoadI{
            constant: 0,
            target: Register(zero_reg),
        },
        None,
        id
    );        
    
    let control_reg = zero_reg + 1;

    ir_func.update_regmax(control_reg);    
    
    ir_func.add_line(
        IlocInstruction::CmpEQ{
            source1: Register(expr_eval_reg),
            source2: Register(zero_reg),
            target: Register(control_reg),
        },
        None,
        id
    );

    ir_func.add_line(
        IlocInstruction::Cbr{
            source: Register(control_reg),
            target_l1: GotoLabel(block_out),
            target_l2: GotoLabel(block_in),
        },
        None,
        id
    );

    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::Goto(GotoLabel(block_in))), id);
    
    let rchild = node.get_rchild().unwrap();

    stmtcomp::ir_stmtcomp(ast,func_table,rchild,entry,iloc)?;

    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::Goto(GotoLabel(block_out))), id);
    
    Ok(())
}
