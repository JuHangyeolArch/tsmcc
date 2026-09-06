use std::ops::Range;
use std::fmt;
use std::error::Error;
use crate::compiler::ProducedBy;

use super::SpanInfo;
use super::CompileError;
use super::token::{Token,TokenList,TokenType};
use super::ast::{
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



pub mod program;
pub mod funclist;
pub mod function;
pub mod functioninterface;
pub mod parameterlist;
pub mod declarationlist;
pub mod declaration;
pub mod declarator;
pub mod valuekind;
pub mod stars;
pub mod statement;
pub mod statementlist;
pub mod assignexpr;
pub mod condexpr;
pub mod addexpr;
pub mod term;
pub mod unaryexpr;
pub mod factor;
pub mod arglist;

pub mod return_immediate;


#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    cur_nodeid: Option<NodeID>,
    is_over: bool,
}

impl Parser {
    pub fn new() -> Self {
        Parser{
            cur_nodeid: None,
            is_over: false,
        }
    }

    pub fn step(
        &mut self,
        token_list: & TokenList,
        ast: &mut Ast,
    ) -> Option<Result<(),CompileError>> {
        if self.is_over() {
            None
        } else {
            match self.progress_node(
                token_list,
                ast,
            ) {
                Ok(()) => Some(Ok(())),
                Err(parse_error) => Some(Err(CompileError::ParseError(parse_error))),
            }
            
        }

    }


    
    fn set_curnode(&mut self, node_id: Option<NodeID>) {
        self.cur_nodeid = node_id;
    }

    fn get_curnode(&self) -> Option<NodeID> {
        match self.cur_nodeid {
            Some(id) => Some(id),
            None => None,
        }
    }

    fn get_curnode_kind(&self, ast: &Ast) -> AstNodeKind { 
        ast.get(self.cur_nodeid.unwrap())
                .unwrap()
                .get_kind()
    }

    fn set_curnode_kind(&self, ast: &mut Ast, kind: AstNodeKind) { 
        ast.get_mut(self.cur_nodeid.unwrap())
                .unwrap()
                .set_kind(kind)
    }

    
    fn get_parent(&self, ast: &Ast) -> Option<NodeID> {
        ast.get(self.get_curnode()?)?
            .get_parent()
    }
    
    fn get_lchild(&self, ast: &Ast) -> Option<NodeID> {
        ast.get(self.get_curnode()?)?
            .get_lchild()
    }

    fn get_rchild(&self, ast: &Ast) -> Option<NodeID> {
        ast.get(self.get_curnode()?)?
            .get_rchild()
    }
    
    fn get_cur_start(&self, ast: &Ast) -> usize {
        ast.get(self.get_curnode().unwrap()).unwrap()
            .get_start()
    }
    
    fn get_cur_end(&self, ast: &Ast) -> Option<usize> {
        ast.get(self.get_curnode()?)?
            .get_end()
    }

    fn get_lchild_end(&self, ast: &Ast) -> Option<usize> {
        ast.get(self.get_lchild(ast)?)?
            .get_end()
    }

    fn get_rchild_end(&self, ast: &Ast) -> Option<usize> {
        ast.get(self.get_rchild(ast)?)?
            .get_end()        
    }

    fn get_lchild_kind(&self, ast: &Ast) -> Option<AstNodeKind> {
        match self.get_lchild(ast) {
            Some(node) => Some(ast
                .get(node)
                .unwrap()
                .get_kind()),
            None => None,
        }
    }

    fn set_cur_end(&self, ast: &mut Ast, pos: usize){
        ast.get_mut(self.get_curnode().unwrap()).unwrap()
            .set_end(pos)
    }

    
    pub fn is_over(&self) -> bool {
        self.is_over
    }
    
   

    fn progress_node(
        &mut self,
        token_list: &TokenList,
        ast: &mut Ast) -> Result<(), ParseError> {
            
            let node_id  = self.get_curnode().unwrap_or_else(|| {
                self.init_ast(token_list, ast);
                self.get_curnode().unwrap()
            });
            
            match ast.get(node_id) {
                Some(node) => match node.get_kind() {
                    AstNodeKind::Program => self.handle_program(token_list,ast),
                    AstNodeKind::FunctionList => self.handle_funclist(token_list,ast),
                    AstNodeKind::Function => self.handle_function(token_list,ast),
                    AstNodeKind::FunctionInterface => self.handle_fi(token_list,ast),
                    AstNodeKind::ParameterList => self.handle_paramlist(token_list,ast),
                    AstNodeKind::DeclarationList => self.handle_declarationlist(token_list,ast),
                    AstNodeKind::Declaration => self.handle_declaration(token_list,ast),
                    AstNodeKind::Declarator => self.handle_declarator(token_list,ast),
                    AstNodeKind::ValueKind => self.handle_valuekind(token_list,ast),
                    AstNodeKind::BasicValue(_) => self.handle_return_immediate(token_list,ast),
                    AstNodeKind::Stars => self.handle_stars(token_list,ast),
                    AstNodeKind::Star => self.handle_return_immediate(token_list,ast),
                    AstNodeKind::StatementList => self.handle_statementlist(token_list,ast),
                    AstNodeKind::Statement(_) => self.handle_statement(token_list,ast),
                    AstNodeKind::AssignExpr => self.handle_assignexpr(token_list,ast),
                    AstNodeKind::CondtionExpr => self.handle_condexpr(token_list,ast),
                    AstNodeKind::AdditiveExpr(_) => self.handle_addexpr(token_list,ast),
                    AstNodeKind::Term(_) => self.handle_term(token_list,ast),
                    AstNodeKind::UnaryExpr(_) => self.handle_unaryexpr(token_list,ast),
                    AstNodeKind::Factor(_) => self.handle_factor(token_list,ast),
                    AstNodeKind::Identifier(_) => self.handle_return_immediate(token_list,ast),
                    AstNodeKind::Literal(_) => self.handle_return_immediate(token_list,ast),
                    AstNodeKind::ArgList => self.handle_arglist(token_list,ast),
                },                
                None => Err(ParseError::UnexpectedError),
            }
            
        }


    fn init_ast(&mut self, token_list: &TokenList,ast: &mut Ast) {
        self.set_curnode ( Some(
            ast.alloc(AstNodeKind::Program,
                None,
                0,
                Some(token_list.len() - 1))
        ));
    }

    
    fn node_case<F,G,H>(
        &mut self,
        ast: &mut Ast,
        token_list: &TokenList,
        mut empty: F,
        mut left: G,
        mut right: H)
    -> Result<(()), ParseError>
    where
    F: FnMut(&mut Self, &mut Ast, &TokenList) -> Result<(), ParseError>,
    G: FnMut(&mut Self, &mut Ast, &TokenList) -> Result<(), ParseError>,
    H: FnMut(&mut Self, &mut Ast, &TokenList) -> Result<(), ParseError>, {


        let node = ast.get(self.get_curnode().unwrap()).unwrap();
        
        match (node.get_lchild(), node.get_rchild()){
            (None, None) => empty(self, ast,token_list) ,
            (Some(_), None) => left(self, ast,token_list),
            (Some(_),Some(_)) => right(self, ast,token_list),
            (None,Some(_)) => Err(ParseError::UnexpectedError),
        }
     
    }

    fn push_lchild(&mut self, ast: &mut Ast, kind: AstNodeKind, start: usize, end: Option<usize>){
        let lchild = ast.alloc(
            kind,
            Some(self.get_curnode().unwrap()),
            start,
            end,
        );

        ast
            .get_mut(self.get_curnode().unwrap())
            .unwrap()
            .set_lchild(lchild);
                                    
        self.set_curnode(Some(lchild));
    }

    fn push_rchild(&mut self, ast: &mut Ast, kind: AstNodeKind, start: usize, end: Option<usize>){
        let rchild = ast.alloc(
            kind,
            Some(self.get_curnode().unwrap()),
            start,
            end,
        );

        ast
            .get_mut(self.get_curnode().unwrap())
            .unwrap()
            .set_rchild(rchild);
                                    
        self.set_curnode(Some(rchild));
    }


}

    
#[derive(Debug,Clone,PartialEq)]
pub enum ParseError {
    InvalidSyntax(usize),
    UnexpectedError,
}
