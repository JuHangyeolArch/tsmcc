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

pub fn handle_program(
    &mut self,
    token_list: &TokenList,
    ast: &mut Ast ) -> Result<(), ParseError> {

        self.node_case(
            ast,
            token_list,
            |p, a, l| {

                let start = p.get_cur_start(a);
                
                p.push_lchild(
                    a,
                    AstNodeKind::FunctionList,
                    start,
                    None,
                );
                Ok(())
            },
            |p, a, l| {

                p.set_cur_end(a,p.get_lchild_end(a).unwrap());
                p.set_curnode(p.get_parent(a));
              
                p.is_over = true;
                Ok(())
            },
            |p, a, l| {
                Err(ParseError::UnexpectedError)
            },
        )
    }
    
}
