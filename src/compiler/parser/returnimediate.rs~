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

    pub fn handle_basicvalue(
        &mut self,
        token_list: &TokenList,
        ast: &mut Ast ) -> Result<(), ParseError> {


            self.node_case(
                ast,
                token_list,
                |p, a, l| {
                    p.set_curnode(p.get_parent(a));
                    Ok(())
                },
                |p, a, l|{
                    Err(ParseError::UnexpectedError)
                },
                |p, a, l| {
                    Err(ParseError::UnexpectedError)
                },
            )
        }
    
}
