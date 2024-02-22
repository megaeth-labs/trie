#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Node {
    pub total_number: u64,
    pub leaf_number: u64,
    pub total_depth: u64,
    pub leaf_depth: u64,
}

impl Node {
    pub fn new_leaf() -> Self {
        Self { total_number: 1, leaf_number: 1, total_depth: 0, leaf_depth: 0 }
    }

    pub fn new_branch() -> Self {
        Self { total_number: 1, leaf_number: 0, total_depth: 0, leaf_depth: 0 }
    }

    pub fn new_extension(branch: &Self) -> Self {
        Self {
            total_number: branch.total_number + 1,
            leaf_number: branch.leaf_number,
            total_depth: branch.total_number + branch.total_depth + 1,
            leaf_depth: branch.leaf_number + branch.leaf_depth,
        }
    }

    pub fn add(&mut self, other: &Self) {
        self.total_number = self.total_number.checked_add(other.total_number).expect("overflow");
        self.leaf_number = self.leaf_number.checked_add(other.leaf_number).expect("overflow");
        self.total_depth = self.total_depth.checked_add(other.total_depth).expect("overflow");
        self.leaf_depth = self.leaf_depth.checked_add(other.leaf_depth).expect("overflow");
    }

    pub fn add_child(&mut self, other: &Self) {
        self.total_number = self.total_number.checked_add(other.total_number).expect("overflow");
        self.leaf_number = self.leaf_number.checked_add(other.leaf_number).expect("overflow");
        self.total_depth = self
            .total_depth
            .checked_add(other.total_depth)
            .and_then(|x| x.checked_add(other.total_number))
            .expect("overflow");
        self.leaf_depth = self
            .leaf_depth
            .checked_add(other.leaf_depth)
            .and_then(|x| x.checked_add(other.leaf_number))
            .expect("overflow");
    }

    pub fn checked_add(&mut self, rhs: Self) -> Option<Self>{
        self.total_number = self.total_number.checked_add(rhs.total_number).expect("overflow");
        self.leaf_number = self.leaf_number.checked_add(rhs.leaf_number).expect("overflow");
        self.total_depth = self.total_depth.checked_add(rhs.total_depth).expect("overflow");
        self.leaf_depth = self.leaf_depth.checked_add(rhs.leaf_depth).expect("overflow");

        Some(self.clone())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub trie_node: Node,
    pub state_node: Node,
}

impl TreeNode {
    pub fn new_leaf(root: Option<TreeNode>) -> Self {
        Self {
            trie_node: Node::new_leaf(),
            state_node: if root.is_none() {
                Node::new_leaf()
            } else {
                if TreeNode::default() == root.unwrap_or_default() {
                    Node::new_leaf()
                } else {
                    root.unwrap_or_default().state_node
                }
            },
        }
    }

    pub fn new_branch() -> Self {
        Self { trie_node: Node::new_branch(), state_node: Node::new_branch() }
    }

    pub fn new_extension(branch: &TreeNode) -> Self {
        Self {
            trie_node: Node::new_extension(&branch.trie_node),
            state_node: Node::new_extension(&branch.state_node),
        }
    }

    pub fn add(&mut self, other: &Self) {
        self.trie_node.checked_add(other.trie_node);
        self.state_node.checked_add(other.state_node);
    }

    pub fn add_child(&mut self, other: &Self) {
        self.trie_node.add_child(&other.trie_node);
        self.state_node.add_child(&other.state_node);
    }

    pub fn checked_add(&mut self, rhs: Self) -> Option<Self> {
        self.trie_node = self.trie_node.checked_add(rhs.trie_node).expect("overflow");
        self.state_node = self.state_node.checked_add(rhs.state_node).expect("overflow");

        Some(self.clone())
    }
}
