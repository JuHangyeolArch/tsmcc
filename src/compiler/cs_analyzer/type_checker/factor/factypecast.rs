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

use crate::compiler::cs_analyzer::CsError;
use crate::compiler::cs_analyzer::type_checker::SideKind;
use crate::compiler::cs_analyzer::type_checker::valuekind;
use crate::compiler::cs_analyzer::type_checker::factor;


pub fn tc_factypecast(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(ValueType,SideKind),CsError> {

    let node = ast.get(id).unwrap();    
    let lchild = node.get_lchild().unwrap();    

    let casting_type = valuekind::tc_valuekind(ast,id)?;
    
    let rchild = node.get_rchild().unwrap();

    factor::tc_factor(ast,func_table,rchild,entry)?;
    
    Ok((casting_type,SideKind::Rvalue))

}
