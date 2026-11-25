#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 8.4 — The same tree as Lab 8.3, rebuilt as an arena with indices.
//!
//! No `Rc`, no `RefCell`, no `Weak`: the `Tree` owns every node in a `Vec`,
//! and links are plain `NodeId` indices. This is how many real Rust
//! codebases (compilers, ECS game engines, graph libraries) model graphs.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

#[derive(Debug)]
struct NodeData {
    value: i32,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<NodeData>,
}

impl Tree {
    /// A tree containing only a root node.
    pub fn with_root(value: i32) -> Self {
        todo!("lab-8-4-arena-tree: implement me")
    }

    pub fn root(&self) -> NodeId {
        NodeId(0)
    }

    pub fn add_child(&mut self, parent: NodeId, value: i32) -> NodeId {
        todo!("lab-8-4-arena-tree: implement me")
    }

    pub fn value(&self, id: NodeId) -> i32 {
        todo!("lab-8-4-arena-tree: implement me")
    }

    pub fn set_value(&mut self, id: NodeId, value: i32) {
        todo!("lab-8-4-arena-tree: implement me")
    }

    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        todo!("lab-8-4-arena-tree: implement me")
    }

    pub fn children(&self, id: NodeId) -> &[NodeId] {
        todo!("lab-8-4-arena-tree: implement me")
    }

    pub fn path_to_root(&self, id: NodeId) -> Vec<i32> {
        todo!("lab-8-4-arena-tree: implement me")
    }

    pub fn sum_subtree(&self, id: NodeId) -> i32 {
        todo!("lab-8-4-arena-tree: implement me")
    }

    pub fn depth(&self, id: NodeId) -> usize {
        todo!("lab-8-4-arena-tree: implement me")
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
