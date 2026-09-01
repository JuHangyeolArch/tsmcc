use std::any::Any;

use crate::compiler::parser::Parser;
use crate::compiler::parser::ParseError;
use crate::compiler::ProducedBy;
use super::SpanInfo;
use super::CompileError;
use crate::compiler::token::{Token,TokenList,TokenType};
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



impl Parser {

pub fn handle_funclist(
    &mut self,
    token_list: &TokenList,
    ast: &mut Ast ) -> Result<(), ParseError> {

        self.node_case(
            ast,
            token_list,
            |p, a, l| {

                let start: usize = p.get_cur_start(a);

                if matches!(l.get(start).unwrap().kind(), TokenType::Eof) {
                    p.set_cur_end(a,start);
                    p.set_curnode(p.get_parent(a));
                } else {
                    p.push_lchild(
                        a,
                        AstNodeKind::Function,
                        start,
                        None,
                    );
                    p.set_curnode(p.get_lchild(a));
                }
                Ok(())
            },
            |p, a, l| {
                let start: usize = p
                    .get_lchild_end(a)
                    .unwrap();
                
                p.push_rchild(
                    a,
                    AstNodeKind::FunctionList,
                    start,
                    None,
                );

                Ok(())
            },
            |p, a, l| {

                p.set_cur_end(a,p.get_rchild_end(a).unwrap());
                p.set_curnode(p.get_parent(a));

                Ok(())
            },
        )
    }
    
}
