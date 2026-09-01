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

use crate::compiler::ir_generator::arglist;
use crate::compiler::ir_generator::id;


pub fn ir_facpcall(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    reg_count: usize,    
) -> Result<(SideKind,usize),IrGenError> {

    let node = ast.get(id).unwrap();

    let lchild = node.get_lchild().unwrap();


    let AstNodeKind::Identifier(name) = node.get_kind() else {
        return Err(IrGenError::UnexpectedError);
    };

    let name = id::ir_id(ast,nodeid)?;
        
    let func_idx = func_table
        .get_index(&name).unwrap();

    let ir_func = iloc
        .get_mut_bypos(entry)
        .unwrap();

    let psize = ir_func.get_paramsize();
    
    let rchild = node.get_rchild().unwrap();
    arglist::ir_arglist(ast,func_table,rchild,entry,iloc,reg_count)?;

    ir_func.update_regmax(reg_count);
    ir_func.add_line(IlocInstruction::Empty,Some(IlocLabel::Call(func_idx,Register(reg_count))), id);

   
    Ok((Rvalue,reg_count))
}
