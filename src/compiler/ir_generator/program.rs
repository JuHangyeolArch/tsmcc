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
    IlocLabel,
    IlocLine,
    IlocInstruction,
};


use crate::compiler::ir_generator::IrGenError;
use crate::compiler::ir_generator::funclist;

pub fn ir_program(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    iloc: &mut IlocProgram,
) -> Result<(),IrGenError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    for i in 0..func_table.count_built_in() {

        let built_in = func_table
            .get_unmut_bypos(i)
            .unwrap();

        let name = built_in.get_name();
        let param_count = built_in.get_params();
        iloc.push_func(
            &name,
            i,
            param_count,
            0,
        );

        iloc
            .get_mut_bypos(i)
            .unwrap()
            .add_line(
                IlocInstruction::Empty,
                Some(IlocLabel::BuiltIn(name)),
                NodeID::new(0)
            )
    }

    
    funclist::ir_funclist(ast,func_table,lchild,func_table.count_built_in(),iloc)
    
}
