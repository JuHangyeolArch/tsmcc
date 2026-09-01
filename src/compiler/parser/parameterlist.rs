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

pub fn handle_paramlist(
    &mut self,
    token_list: &TokenList,
    ast: &mut Ast ) -> Result<(), ParseError> {


        self.node_case(
            ast,
            token_list,
            |p, a, l| {

                let start: usize = p.get_cur_start(a);
               

                if matches!(l.get(start).unwrap().kind(), TokenType::RParenth) {
                    p.set_cur_end(a,start);
                    p.set_curnode(p.get_parent(a));
                } else {
                    p.push_lchild(
                        a,
                        AstNodeKind::Declarator,
                        start,
                        None,
                    );
                    p.set_curnode(p.get_lchild(a));
                }
                
                Ok(())
            },
            |p, a, l|{

                let mut start: usize = p
                    .get_lchild_end(a)
                    .unwrap();

                let token_kind = l.get(start).unwrap().kind();

                
                if ! matches!(token_kind.clone(), TokenType::Comma) {
                    if ! matches!(token_kind, TokenType::RParenth){
                        return Err(ParseError::InvalidSyntax(start));
                    }                    
                }
                else {
                    start = start + 1;
                }
                
                p.push_rchild(
                        a,
                        AstNodeKind::ParameterList,
                    start,
                    None
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
