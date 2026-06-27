pub mod app;
pub mod child_container;
pub mod component;
pub mod context;
pub mod element;
pub mod error;
pub mod error_boundary;
pub mod lifecycle;
pub mod render_object;
pub mod semantic_node;
pub mod types;

pub use app::TezzeraApp;
pub use child_container::ChildContainer;
pub use component::TezzeraComponent;
pub use context::Context;
pub use element::Element;
pub use error::{TezzeraError, TezzeraResult};
pub use error_boundary::ErrorBoundary;
pub use render_object::{AxisBound, Canvas, Constraints, RenderObject};
pub use semantic_node::{Role, SemanticNode};
pub use types::{AtomId, ComponentId, Key, Location, Point, Rect, Size};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lifecycle::on_mount;

    // ── component_builds_element ───────────────────────────────────────────

    struct Greeting;

    impl TezzeraComponent for Greeting {
        fn build(&self, _ctx: &mut Context) -> Element {
            Element::text("Hello, TEZZERA!")
        }
    }

    #[test]
    fn component_builds_element() {
        let greeting = Greeting;
        let mut ctx = Context::new(ComponentId(1));
        let element = greeting.build(&mut ctx);
        assert!(
            !matches!(element, Element::Empty),
            "build() must return a non-Empty element"
        );
    }

    // ── lifecycle_on_cleanup_registered ───────────────────────────────────

    #[test]
    fn lifecycle_on_cleanup_registered() {
        let mut ctx = Context::new(ComponentId(2));
        on_mount(&mut ctx, || {
            // mount work (none needed for the test)
            || { /* unmount cleanup */ }
        });
        assert!(
            !ctx.unmount_callbacks.is_empty(),
            "on_mount must register a cleanup callback"
        );
    }

    // ── error_boundary_has_fallback ────────────────────────────────────────

    #[test]
    fn error_boundary_has_fallback() {
        let boundary = ErrorBoundary::new()
            .fallback(|_e| Element::text("something went wrong"))
            .child(Element::text("normal content"));
        let result = boundary.render();
        assert!(
            !matches!(result, Element::Empty),
            "render() must return the child element"
        );
    }

    // ── child_container_order_preserved ───────────────────────────────────

    struct SimpleContainer {
        elements: Vec<Element>,
    }

    impl SimpleContainer {
        fn new() -> Self {
            SimpleContainer {
                elements: Vec::new(),
            }
        }
    }

    impl ChildContainer for SimpleContainer {
        fn child(mut self, element: impl Into<Element>) -> Self {
            self.elements.push(element.into());
            self
        }

        fn children<E: Into<Element>>(mut self, elements: Vec<E>) -> Self {
            self.elements
                .extend(elements.into_iter().map(|e| e.into()));
            self
        }

        fn prepend(mut self, element: impl Into<Element>) -> Self {
            self.elements.insert(0, element.into());
            self
        }
    }

    #[test]
    fn child_container_order_preserved() {
        let container = SimpleContainer::new()
            .child(Element::text("first"))
            .child(Element::text("second"))
            .child(Element::text("third"));

        assert_eq!(container.elements.len(), 3);

        // Verify insertion order is preserved.
        let texts: Vec<&str> = container
            .elements
            .iter()
            .filter_map(|e| {
                if let Element::Text(t) = e {
                    Some(t.content.as_str())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(texts, ["first", "second", "third"]);
    }

    // ── constraints_loose_has_zero_min ────────────────────────────────────

    #[test]
    fn constraints_loose_has_zero_min() {
        let c = Constraints::loose(800.0, 600.0);
        assert_eq!(c.min_width, 0.0);
        assert_eq!(c.min_height, 0.0);
    }

    // ── tezzera_error_display ─────────────────────────────────────────────

    #[test]
    fn tezzera_error_display() {
        let e = TezzeraError::not_found("User");
        assert!(
            e.to_string().contains("User"),
            "Display must include the resource name"
        );
    }

    // ── context_state_creates_atom ────────────────────────────────────────

    #[test]
    fn context_state_creates_atom() {
        let mut ctx = Context::new(ComponentId(1));
        let atom = ctx.state(42i32);
        assert_eq!(atom.get(), 42);
        atom.set(100);
        assert_eq!(atom.get(), 100);
    }
}
