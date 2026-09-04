use super::SpanInfo;
use super::SnapShotPhase;
use super::ProducedBy;

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


use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct FuncTable {
    vec: Vec<FunctionInfo>,
    map: HashMap<String, usize>,
    count: usize,
    built_in: usize,
}



impl FuncTable {

    pub fn new() -> Self {
        let mut table = FuncTable {
            vec: Vec::new(),
            map: HashMap::new(),
            count: 0,
            built_in: 0,
        };

        table.add_built_in("get",&ValueType::new(BasicValueKind::Int,0));        
        table.add_built_in("put",&ValueType::new(BasicValueKind::Int,0));
        table
            .get_mut_byname("put")
            .unwrap()
            .add_param("value",&ValueType::new(BasicValueKind::Int,0),None);
        
        
        table
    }
    
    pub fn add(&mut self, name: &str, value_type: &ValueType,id: Option<NodeID>) {
        self.vec.push(FunctionInfo::new(name,value_type,id));
        self.map.insert(name.to_string(),self.vec.len() -1);
        self. count = self.count + 1
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        Some(*self.map.get(name)?)
    }

    pub fn get_byname(&self, name: &str) -> Option<&FunctionInfo> {
        self.vec.get( self.get_index(name)?)
    }

    pub fn get_mut_byname(&mut self, name: &str) -> Option<&mut FunctionInfo> {
        let i= self.get_index(name)?;
        self.vec.get_mut(i)
    }
    
    pub fn get_bypos(&mut self, pos: usize) -> Option<&mut FunctionInfo> {
        self.vec.get_mut(pos)
    }

    pub fn get_unmut_bypos(&self, pos: usize) -> Option<&FunctionInfo> {
        self.vec.get(pos)
    }

    fn add_built_in(&mut self, name: &str, value_type:&ValueType) {
        self.add(name,value_type,None);
        self.built_in = self.built_in + 1;
    }

    
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn count_built_in(&self) -> usize {
        self.built_in
    }
    
}





#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInfo {
    name: String,
    value_type: ValueType,
    param_count: usize,
    var_list: VarList,
    span: SpanInfo,
    flow: FlowGraph,

}

impl FunctionInfo {
    pub fn new(name: &str, value_type: &ValueType, id: Option<NodeID>) -> Self {

        let span: SpanInfo;
        
        if let Some(node) = id {
            span = SpanInfo::new(SnapShotPhase::CsAnalyzer,Some(node.to_usize()),Some(node.to_usize()+1));
        } else {
            span = SpanInfo::new(SnapShotPhase::CsAnalyzer, None,None);
        }

        FunctionInfo {
            name: name.to_string(),
            value_type: value_type.clone(),
            param_count: 0,
            var_list: VarList::new(),
            span,
            flow: FlowGraph::new()
            
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    
    pub fn get_params(&self) -> usize {
        self.param_count
    }

    pub fn get_type(&self) -> ValueType {
        self.value_type
    }

    
    pub fn add_param(&mut self, name: &str,value_type: &ValueType,id: Option<NodeID>) {
        self.var_list.add(name,value_type,id);
        
        self.param_count = self.param_count + 1;
    }
    
    pub fn get_vars(&mut self) -> &mut VarList{
        &mut self.var_list
    }
    
    pub fn get_unmut_vars(&self) -> &VarList{
        &self.var_list
    }
    
    pub fn get_span(&self) -> SpanInfo {
        self.span.clone()
    }

    pub fn get_mut_fg(&mut self) -> &mut FlowGraph {
        &mut self.flow
    }

    
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValueType {
    base_kind: BasicValueKind,
    ref_depth: u32,
}

impl ValueType {
    pub fn new(base_kind: BasicValueKind,ref_depth: u32) -> Self {
        ValueType {
            base_kind,
            ref_depth,
        }   
    }

    pub fn get_base(&self) -> BasicValueKind {
        self.base_kind
    }

    pub fn get_depth(&self) -> u32 {
        self.ref_depth
    }

    pub fn get_ref(&self) -> Self {
        ValueType::new(
            self.get_base(),
            self.get_depth() + 1

        )
    }

    pub fn get_deref(&self) -> Option<Self> {

        if self.get_depth() <= 0 {
            return None;
        };
            
        Some(ValueType::new(
            self.get_base(),
            self.get_depth() - 1
        ))

    }

    
    pub fn is_equal(&self, target: Self) -> bool {
        self.get_base() == target.get_base() &&
            self.get_depth() == target.get_depth()
        
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    name: String,
    value_type: ValueType,    
    span: SpanInfo,
}

impl Variable {

    pub fn new(name: &str, value_type: &ValueType, id: Option<NodeID>) -> Self {

        let span;
        
        if let Some(node) = id {
            span = SpanInfo::new(SnapShotPhase::CsAnalyzer,Some(node.to_usize()),Some(node.to_usize()+1));
        } else {
            span = SpanInfo::new(SnapShotPhase::CsAnalyzer, None,None);
        }


        Variable {
            name: name.to_string(),
            value_type: value_type.clone(),
            span
        }        
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }


    pub fn get_type(&self) -> ValueType {
        self.value_type
    }

    pub fn get_span(&self) -> SpanInfo {
        self.span.clone()
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct VarList {
    vec: Vec<Variable>,
    map: HashMap<String, usize>,
    count: usize,
}

impl VarList {

    pub fn new() -> Self {
        VarList {
            vec: Vec::new(),
            map: HashMap::new(),
            count: 0,
        }
    }
    
    pub fn add(&mut self, name: &str, value_type: &ValueType,id: Option<NodeID>) {
        self.vec.push(Variable::new(name,value_type,id));
        self.map.insert(name.to_string(),self.vec.len() -1);
        self. count = self.count + 1
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        Some(*self.map.get(name)?)
    }

    pub fn get_byname(&self, name: &str) -> Option<&Variable> {
        self.vec.get( self.get_index(name)?)
    }

    pub fn get_bypos(&self, pos: usize) -> Option<&Variable> {
        self.vec.get(pos)
    }

    pub fn count(&self) -> usize {
        self.count
    }

}


#[derive(Debug, Clone, PartialEq)]
pub struct FlowGraph {
    vec: Vec<FGNode>
}

impl FlowGraph {

    pub fn new() -> Self {
        FlowGraph {
            vec: Vec::new()
        }
    }

    pub fn alloc(&mut self) -> usize {
        self.vec.push(FGNode::new(false));
        
        self.vec.len() -1
    }

    pub fn get(&self, index:usize) -> Option<&FGNode> {
        self.vec.get(index)
    }
    pub fn get_mut(&mut self, index:usize) -> Option<&mut FGNode> {
        self.vec.get_mut(index)
    }

    pub fn get_root(&self) -> Option<usize> {
        self.get(0)?;
        Some(0)
    }
    
    pub fn must_return(&self) -> bool {

        let mut visited: HashMap<usize,bool> = HashMap::new();

        let Some(root) = self.get_root() else {
            return false;
        };

        self.path_return(root,&mut visited)

    }

    fn path_return(&self, cur_idx: usize, visited: &mut HashMap<usize, bool>) -> bool {

        let cur_node =  self.get(cur_idx).unwrap();

        if  let Some(cache) = visited.get(&cur_idx) {
            return *cache;
        }
        
        let val_return = cur_node.has_return();

        visited.insert(cur_idx, val_return);
        
        if cur_node.child_count() == 0 {
            val_return
        } else {
            let child_return = cur_node
                .successor
                .iter()
                .fold(val_return,
                    |result,index| {
                        result || self.path_return(*index,visited)                        
                    }
                );
            child_return
        }
        
        
    }
    
}

#[derive(Eq, Hash, Debug, Clone, PartialEq)]
pub struct FGNode {
    ret: bool,
    pub successor: Vec<usize>
}

impl FGNode {

    pub fn new(ret: bool) -> Self {
        FGNode {
            ret,
            successor: Vec::new()
        }
    }

    pub fn has_return(&self) -> bool {
        self.ret
    }

    pub fn set_return(&mut self) {
        self.ret = true
    }
    
    pub fn child_count(&self) -> usize {
        self.successor.len()
    }

    pub fn get_child(&self, index: usize) -> Option<usize> {
        Some(*self.successor.get(index)?)
    }

    pub fn set_child(&mut self, index: usize) {
        self.successor.push(index)
    }
    

}
