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
        // SOLUTION-START
        Tree {
            nodes: vec![NodeData {
                value,
                parent: None,
                children: Vec::new(),
            }],
        }
        // SOLUTION-END
    }

    pub fn root(&self) -> NodeId {
        NodeId(0)
    }

    pub fn add_child(&mut self, parent: NodeId, value: i32) -> NodeId {
        // SOLUTION-START
        let id = NodeId(self.nodes.len());
        self.nodes.push(NodeData {
            value,
            parent: Some(parent),
            children: Vec::new(),
        });
        self.nodes[parent.0].children.push(id);
        id
        // SOLUTION-END
    }

    pub fn value(&self, id: NodeId) -> i32 {
        // SOLUTION-START
        self.nodes[id.0].value
        // SOLUTION-END
    }

    pub fn set_value(&mut self, id: NodeId, value: i32) {
        // SOLUTION-START
        self.nodes[id.0].value = value;
        // SOLUTION-END
    }

    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        // SOLUTION-START
        self.nodes[id.0].parent
        // SOLUTION-END
    }

    pub fn children(&self, id: NodeId) -> &[NodeId] {
        // SOLUTION-START
        &self.nodes[id.0].children
        // SOLUTION-END
    }

    pub fn path_to_root(&self, id: NodeId) -> Vec<i32> {
        // SOLUTION-START
        std::iter::successors(Some(id), |&n| self.parent(n))
            .map(|n| self.value(n))
            .collect()
        // SOLUTION-END
    }

    pub fn sum_subtree(&self, id: NodeId) -> i32 {
        // SOLUTION-START
        self.value(id)
            + self
                .children(id)
                .iter()
                .map(|&c| self.sum_subtree(c))
                .sum::<i32>()
        // SOLUTION-END
    }

    pub fn depth(&self, id: NodeId) -> usize {
        // SOLUTION-START
        self.path_to_root(id).len() - 1
        // SOLUTION-END
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
