//! Accessibility (a11y) support for TEZZERA — Phase 5 stubs.
//!
//! Provides a data model for WAI-ARIA roles, accessible node trees,
//! and keyboard focus management. Screen reader integration is Phase 6+.
//!
//! # Example
//! ```rust,ignore
//! use tezzera_a11y::{A11yNode, A11yTree, FocusManager, Role};
//!
//! let mut tree = A11yTree::new();
//! let btn = tree.add(A11yNode::new(Role::Button, "Submit").focusable(true));
//! let inp = tree.add(A11yNode::new(Role::TextInput, "Email").focusable(true));
//!
//! let mut focus = FocusManager::new();
//! focus.focus_next(&mut tree);
//! assert_eq!(focus.focused_id(), Some(btn));
//! ```

pub mod focus;
pub mod node;
pub mod role;
pub mod tree;

pub use focus::FocusManager;
pub use node::A11yNode;
pub use role::Role;
pub use tree::A11yTree;
