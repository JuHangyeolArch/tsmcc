use super::SpanInfo;
use super::SnapShotPhase;
use super::ProducedBy;


#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    nodes: Vec<AstNode>,    
}

impl Ast {
    pub fn new() -> Self {
        Ast{
            nodes: Vec::new()
        }
    }

    pub fn alloc (
        &mut self,
        kind: AstNodeKind,
        parent: Option<NodeID>,
        start_pos: usize,
        end_pos: Option<usize>
    ) -> NodeID  {
            self.nodes.push(AstNode::new(kind, parent, start_pos, end_pos));
            NodeID::new(self.nodes.len() - 1)
    }

    pub fn get(&self, id: NodeID ) -> Option<&AstNode> {
        self.nodes.get(id.to_usize())
    }

    pub fn get_mut(&mut self, id: NodeID) -> Option<&mut AstNode> {
        self.nodes.get_mut(id.to_usize())        
    }

    pub fn get_root(& self) -> NodeID {
        NodeID::new(0)
    }
    
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstNode {
    kind: AstNodeKind,
    parent: Option<NodeID>,
    lchild: Option<NodeID>,
    rchild: Option<NodeID>,
    span: SpanInfo,
}


impl AstNode {
    pub fn new(kind: AstNodeKind, parent: Option<NodeID>, start_pos: usize, end_pos: Option<usize>) -> Self {
        AstNode {
            kind: kind,
            parent: parent,
            lchild: None,
            rchild: None,
            span: SpanInfo::new(SnapShotPhase::Parser,Some(start_pos), end_pos)
        }
    }

    pub fn get_kind(&self) -> AstNodeKind {
        self.kind.clone()
    }

    pub fn set_kind(&mut self,kind: AstNodeKind) {
        self.kind = kind;
    }
    
    pub fn get_parent(&self) -> Option<NodeID> {
        self.parent
    }
    
    pub fn get_lchild(&self) -> Option<NodeID> {
        self.lchild
    }

    pub fn get_rchild(&self) -> Option<NodeID> {
        self.rchild
    }

    pub fn set_lchild(&mut self, id: NodeID) {
        self.lchild = Some(id)
    }

    pub fn set_rchild(&mut self, id: NodeID) {
        self.rchild = Some(id)
    }

    pub fn get_start(&self) -> usize {
        self.span.get_start().unwrap()
    }

    pub fn get_end(&self) -> Option<usize>{
        self.span.get_end()
    }
    
    pub fn set_start(&mut self, pos: usize) {
        self.span.set_start(pos)
    }

    pub fn set_end(&mut self, pos: usize) {
        self.span.set_end(pos)
    }


}

impl ProducedBy for AstNode {
    fn produced_by(&self) -> Option<SpanInfo>{
        Some(self.span.clone())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NodeID(usize);

impl NodeID {

    pub fn new(pos: usize) -> Self {
        NodeID(pos)
    }
    
    pub fn to_usize(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNodeKind {
    Program,
    FunctionList,
    Function,
    FunctionInterface,
    ParameterList,
    DeclarationList,
    Declaration,
    Declarator,
    ValueKind,
    BasicValue(BasicValueKind),
    Stars,
    Star,
    StatementList,
    Statement(StatementKind),
    AssignExpr,
    CondtionExpr,
    AdditiveExpr(AdditiveOpKind),
    Term(MultiplicativeOpKind),
    UnaryExpr(UnaryOpKind),
    Factor(FactorKind),
    Identifier(String),
    Literal(i32),
    ArgList,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AdditiveOpKind {
    Nil,
    Add,
    Substract,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MultiplicativeOpKind{
    Nil,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Nil,
    Plus,
    Minus,
    Reference,
    Dereference,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    Expression,
    If,
    While,
    Compound,
    Return,
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FactorKind {
    Nil,
    Literal,
    Variable,
    ParenthAssign,
    ProcedureCall,
    TypeCast,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BasicValueKind {
    Int
}
