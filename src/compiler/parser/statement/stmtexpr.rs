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

    pub fn handle_stmtexpr(
        &mut self,
        token_list: &TokenList,
        ast: &mut Ast ) -> Result<(), ParseError> {


            self.node_case(
                ast,
                token_list,
                |p, a, l| {

                    let start: usize = p.get_cur_start(a);

                    p.push_lchild(
                        a,
                        AstNodeKind::AssignExpr,
                        start,
                        None
                    );
                    
                    Ok(())
                },
                |p, a, l|{
                    let start: usize = p
                        .get_lchild_end(a)
                        .unwrap();
                    
                    if ! matches!(l.get(start).unwrap().kind(), TokenType::Semic) {
                        return Err(ParseError::InvalidSyntax(start));
                    }                

                    p.set_cur_end(a,start+1);
                    p.set_curnode(p.get_parent(a));
                    Ok(())
                },
                |p, a, l| {
                    Err(ParseError::UnexpectedError)
                },
            )
        }
    
}
