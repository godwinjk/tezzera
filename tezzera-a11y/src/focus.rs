use crate::tree::A11yTree;

/// Manages keyboard focus across an A11yTree.
pub struct FocusManager {
    focused_id: Option<u64>,
}

impl FocusManager {
    pub fn new() -> Self {
        Self { focused_id: None }
    }

    /// Currently focused node ID.
    pub fn focused_id(&self) -> Option<u64> {
        self.focused_id
    }

    /// Move focus to the next focusable node (wraps around).
    pub fn focus_next(&mut self, tree: &mut A11yTree) {
        let ids = tree.focusable_ids();
        if ids.is_empty() {
            return;
        }
        self.clear_focus(tree);
        let next = match self.focused_id {
            None => ids[0],
            Some(cur) => {
                let idx = ids
                    .iter()
                    .position(|&id| id == cur)
                    .map(|i| (i + 1) % ids.len())
                    .unwrap_or(0);
                ids[idx]
            }
        };
        self.focused_id = Some(next);
        if let Some(node) = tree.get_mut(next) {
            node.focused = true;
        }
    }

    /// Move focus to the previous focusable node (wraps around).
    pub fn focus_prev(&mut self, tree: &mut A11yTree) {
        let ids = tree.focusable_ids();
        if ids.is_empty() {
            return;
        }
        self.clear_focus(tree);
        let prev = match self.focused_id {
            None => *ids.last().unwrap(),
            Some(cur) => {
                let idx = ids
                    .iter()
                    .position(|&id| id == cur)
                    .map(|i| if i == 0 { ids.len() - 1 } else { i - 1 })
                    .unwrap_or(0);
                ids[idx]
            }
        };
        self.focused_id = Some(prev);
        if let Some(node) = tree.get_mut(prev) {
            node.focused = true;
        }
    }

    /// Focus a specific node by ID.
    pub fn focus(&mut self, tree: &mut A11yTree, id: u64) {
        self.clear_focus(tree);
        self.focused_id = Some(id);
        if let Some(node) = tree.get_mut(id) {
            node.focused = true;
        }
    }

    /// Remove focus from all nodes.
    pub fn blur(&mut self, tree: &mut A11yTree) {
        self.clear_focus(tree);
        self.focused_id = None;
    }

    fn clear_focus(&self, tree: &mut A11yTree) {
        if let Some(id) = self.focused_id {
            if let Some(node) = tree.get_mut(id) {
                node.focused = false;
            }
        }
    }
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::A11yNode;
    use crate::role::Role;

    fn make_tree_with_two_buttons() -> A11yTree {
        let mut tree = A11yTree::new();
        tree.add(A11yNode::new(Role::Button, "First").focusable(true));
        tree.add(A11yNode::new(Role::Button, "Second").focusable(true));
        tree
    }

    #[test]
    fn focus_manager_new_no_focus() {
        let fm = FocusManager::new();
        assert_eq!(fm.focused_id(), None);
    }

    #[test]
    fn focus_next_focuses_first() {
        let mut tree = make_tree_with_two_buttons();
        let mut fm = FocusManager::new();
        fm.focus_next(&mut tree);
        assert_eq!(fm.focused_id(), Some(1));
        assert!(tree.get(1).unwrap().focused);
    }

    #[test]
    fn focus_next_wraps_around() {
        let mut tree = make_tree_with_two_buttons();
        let mut fm = FocusManager::new();
        fm.focus_next(&mut tree); // focus id=1
        fm.focus_next(&mut tree); // focus id=2
        fm.focus_next(&mut tree); // wraps back to id=1
        assert_eq!(fm.focused_id(), Some(1));
    }

    #[test]
    fn focus_prev_wraps_around() {
        let mut tree = make_tree_with_two_buttons();
        let mut fm = FocusManager::new();
        // No focus yet → focus_prev should focus last item
        fm.focus_prev(&mut tree);
        assert_eq!(fm.focused_id(), Some(2));
        // Going prev from id=2 should go to id=1
        fm.focus_prev(&mut tree);
        assert_eq!(fm.focused_id(), Some(1));
        // Going prev from id=1 should wrap to id=2
        fm.focus_prev(&mut tree);
        assert_eq!(fm.focused_id(), Some(2));
    }

    #[test]
    fn focus_specific_node() {
        let mut tree = make_tree_with_two_buttons();
        let mut fm = FocusManager::new();
        fm.focus(&mut tree, 2);
        assert_eq!(fm.focused_id(), Some(2));
        assert!(tree.get(2).unwrap().focused);
        assert!(!tree.get(1).unwrap().focused);
    }

    #[test]
    fn focus_blur_clears() {
        let mut tree = make_tree_with_two_buttons();
        let mut fm = FocusManager::new();
        fm.focus_next(&mut tree);
        fm.blur(&mut tree);
        assert_eq!(fm.focused_id(), None);
        assert!(!tree.get(1).unwrap().focused);
    }

    #[test]
    fn focus_skips_disabled() {
        let mut tree = A11yTree::new();
        tree.add(A11yNode::new(Role::Button, "Enabled").focusable(true));
        tree.add(
            A11yNode::new(Role::Button, "Disabled")
                .focusable(true)
                .disabled(true),
        );
        let mut fm = FocusManager::new();
        fm.focus_next(&mut tree);
        assert_eq!(fm.focused_id(), Some(1));
        fm.focus_next(&mut tree);
        // Only one focusable node, should wrap back to 1
        assert_eq!(fm.focused_id(), Some(1));
    }

    #[test]
    fn focus_empty_tree_no_panic() {
        let mut tree = A11yTree::new();
        let mut fm = FocusManager::new();
        fm.focus_next(&mut tree); // should not panic
        assert_eq!(fm.focused_id(), None);
        fm.focus_prev(&mut tree); // should not panic
        assert_eq!(fm.focused_id(), None);
    }

    #[test]
    fn focus_marks_node_focused_true() {
        let mut tree = A11yTree::new();
        let id = tree.add(A11yNode::new(Role::TextInput, "Name").focusable(true));
        let mut fm = FocusManager::new();
        fm.focus(&mut tree, id);
        assert!(tree.get(id).unwrap().focused);
    }
}
