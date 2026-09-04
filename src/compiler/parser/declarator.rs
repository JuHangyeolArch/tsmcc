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

    pub fn handle_declarator(
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
                        AstNodeKind::ValueKind,
                        start,
                        None,
                    );
                    println!("{:#?}",p.get_curnode());
                    Ok(())
                },
                |p, a, l|{

                    let start: usize = p
                        .get_lchild_end(a)
                        .unwrap();

                    println!("{:#?}", l
                        .get(start)
                        .unwrap()
                        .kind());
                    
                    let TokenType::Id(id) = l
                        .get(start)
                        .unwrap()
                        .kind() else {
                            return Err(ParseError::UnexpectedError);
                        };
                    
                    p.push_rchild(
                        a,
                        AstNodeKind::Identifier(id),
                        start,
                        Some(start +1)
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
