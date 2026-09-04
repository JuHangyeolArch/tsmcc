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

use crate::compiler::cs_analyzer::type_checker::assignexpr;

pub fn tc_stmtret(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let (ltype,_) = assignexpr::tc_assexpr(ast, func_table,lchild,entry)?;


    println!("{:#?}", func_table
        .get_unmut_bypos(entry)
        .unwrap()
        .get_type());
    
    if !func_table
        .get_unmut_bypos(entry)
        .unwrap()
        .get_type()
        .is_equal(ltype) {
            return Err(CsError::TypeMismatched(id));
        }
    
    Ok(())

}
