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

pub fn handle_fi(
    &mut self,
    token_list: &TokenList,
    ast: &mut Ast ) -> Result<(), ParseError> {


        self.node_case(
            ast,
            token_list,
            |p, a, l| {

                let end: usize;
                match l.match_pair_in(
                    p.get_cur_range(a).unwrap(),
                    &mut |t| {t.kind() == TokenType::LParenth},
                    &mut |t| {t.kind() == TokenType::RParenth}
                ) {
                    Some(r) => {end = r.start},
                    None => {return Err(ParseError::InvalidSyntax(p.curnode_start(a)));},
                };
                
                let lchild = a.alloc(
                    AstNodeKind::Declarator,
                    Some(p.get_curnode().unwrap()),
                    p.curnode_start(a),
                    end
                );

                a
                    .get_mut(p.get_curnode().unwrap())
                    .unwrap()
                    .set_lchild(lchild);
                                    
                p.set_curnode(Some(lchild));
                
                Ok(())
            },
            |p, a, l|{
                let lchild_end = p.lchild_end(a)
                    .unwrap();

                
                let rchild = a.alloc(
                    AstNodeKind::ParameterList,
                    Some(p.get_curnode().unwrap()),
                    lchild_end + 1,
                    p.curnode_end(a) - 1
                );

                a
                    .get_mut(p.get_curnode().unwrap())
                    .unwrap()
                    .set_rchild(rchild);
                                    
                p.set_curnode(Some(rchild));

                
                Ok(())
            },
            |p, a, l| {
                p.set_curnode(p.get_parent(a));
                Ok(())
            },
        )
    }
    
}
