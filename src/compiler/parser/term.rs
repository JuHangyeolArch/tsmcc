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
    UnaryOpKind,
    StatementKind,
    FactorKind,
    BasicValueKind,
};



impl Parser {

pub fn handle_term(
    &mut self,
    token_list: &TokenList,
    ast: &mut Ast,
) -> Result<(), ParseError> {


        self.node_case(
            ast,
            token_list,
            |p, a, l| {

                let start: usize = p.get_cur_start(a);
                
                p.push_lchild(
                    a,
                    AstNodeKind::UnaryExpr(UnaryOpKind::Nil),
                    start,
                    None
                );
                    
                Ok(())
            },
            |p, a, l|{

                let start: usize = p
                    .get_lchild_end(a)
                    .unwrap();

                let opkind: MultiplicativeOpKind;
                match l.get(start).unwrap().kind() {

                    TokenType::Star => {opkind = MultiplicativeOpKind::Multiply;},
                    TokenType::Slash => {opkind = MultiplicativeOpKind::Divide;},
                    _ => {
                        p.set_cur_end(a,p.get_lchild_end(a).unwrap());
                        p.set_curnode(p.get_parent(a));
                        return Ok(());
                    }
                }

                p.set_curnode_kind(a,AstNodeKind::Term(opkind));
                
                p.push_rchild(
                    a,
                    AstNodeKind::Term(MultiplicativeOpKind::Nil),
                    start+1,
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

