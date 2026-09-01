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



pub fn ir_stmtret(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
) -> Result<(),IrGenError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();
    let return_bc = 0;
        
    let (expr_kind, expr_reg)= assignexpr::ir_assexpr(ast,func_table,lchild,entry,iloc,1)?;

    let expr_eval_reg: usize = if expr_kind == SideKind::LValue {
         loadval::ir_loadval(id,entry,iloc,expr_reg)
    } else {
        expr_reg
    };

    let ir_func = iloc
        .get_mut_bypos(entry)
        .unwrap();

    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::Return(Register(expr_eval_reg))), id);

    ir_func.add_line(
        IlocInstruction::JumpI{
            targetl: GotoLabel(return_bc),
        },
        None,
        id
    );

    
    Ok(())
}
