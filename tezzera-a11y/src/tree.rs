use std::collections::HashMap;

use crate::node::A11yNode;

/// A flat registry of accessibility nodes, built alongside the widget tree each frame.
pub struct A11yTree {
    nodes: HashMap<u64, A11yNode>,
    next_id: u64,
    /// Root node IDs (top-level nodes with no parent).
    roots: Vec<u64>,
}

impl A11yTree {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_id: 1,
            roots: Vec::new(),
        }
    }

    /// Add a node and return its assigned ID.
    pub fn add(&mut self, mut node: A11yNode) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        node.id = id;
        self.roots.push(id);
        self.nodes.insert(id, node);
        id
    }

    /// Add a node as child of `parent_id`.
    pub fn add_child(&mut self, parent_id: u64, mut node: A11yNode) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        node.id = id;
        self.nodes.insert(id, node);
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(id);
        }
        id
    }

    pub fn get(&self, id: u64) -> Option<&A11yNode> {
        self.nodes.get(&id)
    }
    pub fn get_mut(&mut self, id: u64) -> Option<&mut A11yNode> {
        self.nodes.get_mut(&id)
    }

    /// All focusable node IDs in insertion order.
    pub fn focusable_ids(&self) -> Vec<u64> {
        let mut ids: Vec<u64> = self
            .nodes
            .values()
            .filter(|n| n.focusable && !n.disabled)
            .map(|n| n.id)
            .collect();
        ids.sort();
        ids
    }

    /// Total node count.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Clear all nodes (call at start of each frame rebuild).
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.roots.clear();
        self.next_id = 1;
    }

    /// Emit the full tree as a JSON array string.
    pub fn to_aria_json(&self) -> String {
        let mut ids: Vec<u64> = self.nodes.keys().copied().collect();
        ids.sort();
        let items: Vec<String> = ids
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .map(|n| n.to_json())
            .collect();
        format!("[{}]", items.join(","))
    }
}

impl Default for A11yTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::role::Role;

    #[test]
    fn tree_new_empty() {
        let tree = A11yTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn tree_add_assigns_id() {
        let mut tree = A11yTree::new();
        let id = tree.add(A11yNode::new(Role::Button, "OK"));
        assert_eq!(id, 1);
        assert_eq!(tree.get(id).unwrap().id, 1);
    }

    #[test]
    fn tree_add_increments_ids() {
        let mut tree = A11yTree::new();
        let id1 = tree.add(A11yNode::new(Role::Button, "A"));
        let id2 = tree.add(A11yNode::new(Role::Button, "B"));
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn tree_add_child() {
        let mut tree = A11yTree::new();
        let parent_id = tree.add(A11yNode::new(Role::List, "Items"));
        let child_id = tree.add_child(parent_id, A11yNode::new(Role::ListItem, "Item 1"));
        let parent = tree.get(parent_id).unwrap();
        assert!(parent.children.contains(&child_id));
        assert_eq!(tree.get(child_id).unwrap().id, child_id);
    }

    #[test]
    fn tree_get_returns_node() {
        let mut tree = A11yTree::new();
        let id = tree.add(A11yNode::new(Role::Link, "Home"));
        let node = tree.get(id).unwrap();
        assert_eq!(node.label, "Home");
    }

    #[test]
    fn tree_focusable_ids_filters() {
        let mut tree = A11yTree::new();
        tree.add(A11yNode::new(Role::Image, "Logo")); // not focusable
        let btn_id = tree.add(A11yNode::new(Role::Button, "Submit").focusable(true));
        let inp_id = tree.add(A11yNode::new(Role::TextInput, "Email").focusable(true));
        let ids = tree.focusable_ids();
        assert_eq!(ids, vec![btn_id, inp_id]);
    }

    #[test]
    fn tree_focusable_ids_excludes_disabled() {
        let mut tree = A11yTree::new();
        let btn_id = tree.add(A11yNode::new(Role::Button, "Submit").focusable(true));
        tree.add(
            A11yNode::new(Role::Button, "Disabled")
                .focusable(true)
                .disabled(true),
        );
        let ids = tree.focusable_ids();
        assert_eq!(ids, vec![btn_id]);
    }

    #[test]
    fn tree_clear_resets() {
        let mut tree = A11yTree::new();
        tree.add(A11yNode::new(Role::Button, "X"));
        tree.clear();
        assert!(tree.is_empty());
        // next_id should reset so new additions start from 1
        let id = tree.add(A11yNode::new(Role::Button, "Y"));
        assert_eq!(id, 1);
    }

    #[test]
    fn tree_to_aria_json() {
        let mut tree = A11yTree::new();
        tree.add(A11yNode::new(Role::Button, "Click"));
        let json = tree.to_aria_json();
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
        assert!(json.contains("button"));
    }
}
