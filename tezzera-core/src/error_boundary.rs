use crate::element::Element;
use crate::error::TezzeraError;

/// Catches panics (and future framework-level errors) from a child subtree and
/// renders a fallback element in their place.
///
/// # Phase 1 note
///
/// In Phase 1 the `child` is already a resolved `Element`, so `render()` simply
/// returns the child clone. Panic catching around `TezzeraComponent::build()`
/// calls happens at the framework dispatch level (in `tezzera-render` /
/// `tezzera-cli`), not here. The `ErrorBoundary` struct and its API are the
/// stable surface; the real panic-catching wiring is added in a later phase.
pub struct ErrorBoundary {
    fallback: Box<dyn Fn(&TezzeraError) -> Element + Send + Sync>,
    child: Element,
}

impl ErrorBoundary {
    /// Creates an `ErrorBoundary` with a default fallback that renders the
    /// error message as a text element.
    pub fn new() -> Self {
        ErrorBoundary {
            fallback: Box::new(|e| Element::text(format!("Error: {e}"))),
            child: Element::Empty,
        }
    }

    /// Replaces the fallback renderer.
    pub fn fallback(
        mut self,
        f: impl Fn(&TezzeraError) -> Element + Send + Sync + 'static,
    ) -> Self {
        self.fallback = Box::new(f);
        self
    }

    /// Sets the child element to render when no error has occurred.
    pub fn child(mut self, element: impl Into<Element>) -> Self {
        self.child = element.into();
        self
    }

    /// Returns the child element.
    ///
    /// In Phase 1 this always returns the child as-is. The real panic-catching
    /// path wraps `TezzeraComponent::build()` at the framework dispatch level.
    pub fn render(&self) -> Element {
        self.child.clone()
    }
}

impl Default for ErrorBoundary {
    fn default() -> Self {
        ErrorBoundary::new()
    }
}
