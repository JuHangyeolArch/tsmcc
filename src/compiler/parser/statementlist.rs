use std::ops::Range;
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

pub fn handle_statementlist(
    &mut self,
    token_list: &TokenList,
    ast: &mut Ast ) -> Result<(), ParseError> {

        self.node_case(
            ast,
            token_list,
            |p, a, l| {

                let start: usize = p.get_cur_start(a);

                let kind = match l
                    .get(start)
                    .unwrap()
                    .kind() {
                        TokenType::Id(_) | TokenType::Number(_) | TokenType::LParenth|
                        TokenType::Amper | TokenType::Star  => StatementKind::Expression,
                        TokenType::If => StatementKind::If,
                        TokenType::While => StatementKind::While,
                        TokenType::LBrace => StatementKind::Compound,
                        TokenType::Return => StatementKind::Return,
                        TokenType::Semic => StatementKind::Empty,
                        TokenType::RBrace => {
                            p.set_cur_end(a,start);
                            p.set_curnode(p.get_parent(a));
                            return Ok(());
                        },
                        _ => {return Err(ParseError::InvalidSyntax(start));}
                    };

                p.push_lchild(
                    a,
                    AstNodeKind::Statement(kind),
                    start,
                    None
                );
            
                Ok(())
            },
            |p, a, l| {
                let start: usize = p
                    .get_lchild_end(a)
                    .unwrap();

                
                p.push_rchild(
                    a,
                    AstNodeKind::StatementList,
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

