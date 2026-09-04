use std::ops::Range;

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

pub fn handle_unaryexpr(
    &mut self,
    token_list: &TokenList,
    ast: &mut Ast,
) -> Result<(), ParseError> {


        self.node_case(
            ast,
            token_list,
            |p, a, l| {

                let start: usize = p.get_cur_start(a);

                let opkind : UnaryOpKind;
                
                match l.get(start).unwrap().kind(){
                    TokenType::Star => {opkind = UnaryOpKind::Dereference;},
                    TokenType::Amper => {opkind = UnaryOpKind::Reference;},
                    TokenType::Plus => {opkind = UnaryOpKind::Plus;},
                    TokenType::Minus => {opkind = UnaryOpKind::Minus;},
                    _ => {opkind = UnaryOpKind::Nil}                    
                }

                if ! matches!(opkind, UnaryOpKind::Plus |UnaryOpKind::Minus |UnaryOpKind::Reference |UnaryOpKind::Dereference) {
                    p.push_lchild(
                        a,
                        AstNodeKind::Factor(FactorKind::Nil),
                        start,
                        None
                    );
                    return Ok(());
                }

                    p.push_lchild(
                        a,
                        AstNodeKind::UnaryExpr(opkind),
                        start+1,
                        None
                    );                
                
                Ok(())
            },
            |p, a, l|{
                p.set_cur_end(a,p.get_lchild_end(a).unwrap());
                p.set_curnode(p.get_parent(a));
                Ok(())
            },
            |p, a, l| {
                Err(ParseError::UnexpectedError)
            },
        )
    }



    
}

