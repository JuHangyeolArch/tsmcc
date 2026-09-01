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
    UnaryOpKind,
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
use crate::compiler::ir_generator::negative;
use crate::compiler::ir_generator::factor;

pub fn ir_unaryexpr(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
    iloc: &mut IlocProgram,
    reg_count: usize,    
) -> Result<(SideKind_usize),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let AstNodeKind::UnaryExpr(opkind) = ast
        .get(id)
        .unwrap()
        .get_kind() else {
            return Err(IrGenError::UnexpectedError);
        };

    if opkind == UnaryOpKind::Nil {
        let (fac_kind, fac_reg) = factor::ir_factor(ast, func_table, lchild, entry,iloc,reg_count)?;
        return Ok((fac_kind,fac_reg));
    }
    
    let (right_kind,right_reg) = ir_unaryexpr(ast, func_table, lchild, entry,iloc,reg_count)?;

    let ir_func = iloc
        .get_mut_bypos(entry)
        .unwrap();

    
    if opkind == UnaryOpKind::Reference  {

        let ref_reg = right_reg + 1;
        
        ir_func.update_regmax(ref_reg);

        ir_func.add_line(
                IlocInstruction::Load{
                    source: Register(right_reg),
                    target: Register(ref_reg),
                },
                None,
                id
            );
        return Ok((SideKind::Rvalue,ref_reg));
    };

    let right_eval_reg: usize = if right_kind == SideKind::LValue {
        loadval::ir_loadval(id,entry,iloc,right_reg)
    } else {
        right_reg
    };
 
    match opkind {
        UnaryOpKind::Plus => {            
            Ok((SideKind::Rvalue,right_eval_reg))            
        },

        UnaryOpKind::Minus => {
            let negative_reg = negative::ir_negative(id,entry,iloc,right_eval_reg);
            Ok((SideKind::Rvalue, negative_reg))
        },
        UnaryOpKind::Dereference => {
            Ok((SideKind::Lvalue,right_eval_reg))            
        },
        _ => Err(IrGenError::UnexpectedError)
    }

            

}
