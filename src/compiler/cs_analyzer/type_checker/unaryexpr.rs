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

use crate::compiler::cs_analyzer::CsError;
use crate::compiler::cs_analyzer::type_checker::SideKind;
use crate::compiler::cs_analyzer::type_checker::factor;

pub fn tc_unaryexpr(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(ValueType,SideKind),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let AstNodeKind::UnaryExpr(opkind) = ast
        .get(id)
        .unwrap()
        .get_kind() else {
            return Err(CsError::UnexpectedError(id));
        };


    if opkind == UnaryOpKind::Nil {
        return factor::tc_factor(ast, func_table, lchild, entry);
    }
    
    let (ltype,lside) = tc_unaryexpr(ast, func_table, lchild, entry)?;

    
    match opkind {
        UnaryOpKind::Plus | UnaryOpKind::Minus => {
            Ok((ltype,SideKind::Rvalue))
        },
        UnaryOpKind::Reference => {
            if lside == SideKind::Rvalue {
                return Err(CsError::ReferenceToRValue(id));
            }
            
            Ok((ltype.get_ref(),SideKind::Rvalue))            
        },
        UnaryOpKind::Dereference => {
            let Some(deref_type) = ltype.get_deref() else {
                return Err(CsError::CannotDereference(id));
            };

            Ok((deref_type,SideKind::Lvalue))            
        },
        _ => Err(CsError::UnexpectedError(id))
    }
            

}
