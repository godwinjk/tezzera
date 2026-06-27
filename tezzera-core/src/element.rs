use crate::types::{ComponentId, Key};

/// An element representing a component instance in the tree.
#[derive(Clone, Debug)]
pub struct ComponentElement {
    /// The stable identity of this component instance.
    pub id: ComponentId,
    /// Optional reconciliation key.
    pub key: Option<Key>,
    /// Child elements produced by this component.
    pub children: Vec<Element>,
}

/// An element backed by a native platform widget.
#[derive(Clone, Debug)]
pub struct NativeElement {
    /// The widget tag identifying which native widget to instantiate.
    pub tag: &'static str,
    /// Child elements nested inside this native element.
    pub children: Vec<Element>,
}

/// An element representing a plain text node.
#[derive(Clone, Debug)]
pub struct TextElement {
    /// The text content to render.
    pub content: String,
}

/// The fundamental unit of the TEZZERA element tree.
///
/// Elements are lightweight descriptions of what to render; they are cheap to
/// create and discard. The reconciler compares successive element trees to
/// compute the minimal set of mutations applied to render objects.
#[derive(Clone, Debug)]
pub enum Element {
    /// A component-backed element.
    Component(ComponentElement),
    /// A native platform element.
    Native(NativeElement),
    /// A text leaf node.
    Text(TextElement),
    /// A no-op placeholder that renders nothing.
    Empty,
}

impl Element {
    /// Returns the `Empty` variant — renders nothing and produces no render object.
    pub fn empty() -> Self {
        Element::Empty
    }

    /// Creates a text leaf element from any value that converts into a `String`.
    pub fn text(content: impl Into<String>) -> Self {
        Element::Text(TextElement {
            content: content.into(),
        })
    }
}
