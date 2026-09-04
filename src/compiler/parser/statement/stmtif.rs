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

    pub fn handle_stmtif(
        &mut self,
        token_list: &TokenList,
        ast: &mut Ast ) -> Result<(), ParseError> {


            self.node_case(
                ast,
                token_list,
                |p, a, l| {
                    
                    let start: usize =  p.get_cur_start(a);
                    let mut pos = start + 1;

                    if ! matches!(l.get(pos).unwrap().kind(), TokenType::LParenth) {
                        return Err(ParseError::InvalidSyntax(pos));
                    }
                    pos = pos +1;
                    

                    p.push_lchild(
                        a,
                        AstNodeKind::AssignExpr,
                        pos,
                        None
                    );                
                
                    Ok(())
                },
                |p, a, l|{


                    let start: usize = p
                        .get_lchild_end(a)
                        .unwrap();

                    let mut pos = start;
                    
                    if ! matches!(l.get(pos).unwrap().kind(), TokenType::RParenth) {
                        return Err(ParseError::InvalidSyntax(start));
                    }
                    pos = pos + 1;
                    
                    if ! matches!(l.get(pos).unwrap().kind(), TokenType::LBrace) {
                        return Err(ParseError::InvalidSyntax(start));
                    }

                    pos = pos + 1;

                    
                    p.push_rchild(
                        a,
                        AstNodeKind::Statement(StatementKind::Compound),
                        pos,
                        None
                    ); 
                
                    Ok(())

                },
                |p, a, l| {


                    let end: usize = p
                        .get_rchild_end(a)
                        .unwrap();
                    
                    if ! matches!(l.get(end).unwrap().kind(), TokenType::RBrace) {
                        return Err(ParseError::InvalidSyntax(end));
                    }                

                    p.set_cur_end(a,end+1);
                    p.set_curnode(p.get_parent(a));
                    Ok(())
                },
            )
        }
    
}
