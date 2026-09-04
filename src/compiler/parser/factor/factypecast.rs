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

    pub fn handle_factypecast(
        &mut self,
        token_list: &TokenList,
        ast: &mut Ast ) -> Result<(), ParseError> {


            self.node_case(
                ast,
                token_list,
                |p, a, l| {

                    let start: usize = p.get_cur_start(a);

                    if ! matches!(l.get(start).unwrap().kind(), TokenType::LParenth) {
                        return Err(ParseError::InvalidSyntax(start));
                    }
                    
                    p.push_lchild(
                        a,
                        AstNodeKind::ValueKind,
                        start + 1,
                        None
                    );
                    
                    Ok(())
                },
                |p, a, l|{
                    let end: usize = p
                        .get_lchild_end(a)
                        .unwrap();

                    if ! matches!(l.get(end).unwrap().kind(), TokenType::RParenth) {
                        return Err(ParseError::InvalidSyntax(end));
                    }                

                    p.push_rchild(
                        a,
                        AstNodeKind::Factor(FactorKind::Nil),
                        end + 1,
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
